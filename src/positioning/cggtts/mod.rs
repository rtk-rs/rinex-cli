//! CGGTTS special resolution opmoode.
use clap::ArgMatches;

use std::{cell::RefCell, collections::HashMap, str::FromStr};

mod post_process;
pub use post_process::post_process;

mod report;
pub use report::Report;

use rinex::{
    carrier::Carrier,
    prelude::{Observable, SV},
};

use gnss_rtk::prelude::{
    Bias, Candidate, Carrier as RTKCarrier, Method, Observation, OrbitSource, Signal, Solver,
    SPEED_OF_LIGHT_M_S,
};

use cggtts::prelude::{
    CommonViewCalendar, CommonViewClass, FittedData, Observation as FitObservation, SVTracker,
    Track,
};

use crate::{
    cli::Context,
    positioning::{
        cast_rtk_carrier, ClockStateProvider, EphemerisSource, Error as PositioningError,
    },
};

fn ref_rinex_observable(ppp: bool, rtk_carrier: RTKCarrier) -> Observable {
    let mut string = if ppp {
        "L".to_string()
    } else {
        "C".to_string()
    };

    let carrier = rtk_carrier.to_string();
    string.push_str(&carrier); // TODO: this is wrong

    Observable::from_str(&string).unwrap_or_else(|e| {
        panic!(
            "Signal identification issue: non supported constellation? - {}",
            e
        )
    })
}

/// Resolves CGGTTS tracks from input context
pub fn resolve<'a, 'b, CK: ClockStateProvider, O: OrbitSource, B: Bias>(
    ctx: &Context,
    eph: &'a RefCell<EphemerisSource<'b>>,
    mut clock: CK,
    mut solver: Solver<O, B>,
    method: Method,
    matches: &ArgMatches,
) -> Result<Vec<Track>, PositioningError> {
    let obs_data = ctx
        .data
        .observation()
        .expect("RNX2CGGTTS requires OBS RINEX");

    let t0 = obs_data
        .first_epoch()
        .expect("failed to determine first epoch, empty observations?");

    let sampling_period = obs_data
        .dominant_sampling_interval()
        .expect("RNX2CGGTTS requires steady GNSS observations");

    // scheduling
    let mut past_t = t0;

    // TODO: allow customizations
    let cv_calendar = CommonViewCalendar::bipm();

    let mut collecting = false;
    let mut next_period_start = cv_calendar.next_period_start_after(t0);
    let mut next_collection_start = cv_calendar.next_data_collection_after(t0);

    let mut tracks = Vec::<Track>::new();

    let mut trackers = HashMap::<(SV, Observable), SVTracker>::with_capacity(16);

    let mut sv_reference = HashMap::<SV, Observable>::new();
    let mut sv_observations = HashMap::<SV, Vec<Observation>>::new();

    info!(
        "{} - CGGTTS mode deployed - {} until next tracking",
        t0,
        next_collection_start - t0
    );

    let mut release = false;

    for (index, (t, signal)) in obs_data.signal_observations_sampling_ok_iter().enumerate() {
        if index > 0 && t > past_t {
            if collecting {
                info!("{} - new epoch", past_t);
                // solving attempt
                for (sv, observations) in sv_observations.iter() {
                    let mut cd = Candidate::new(*sv, past_t, observations.clone());

                    match clock.next_clock_at(past_t, *sv) {
                        Some(dt) => cd.set_clock_correction(dt),
                        None => error!("{} ({}) - no clock correction available", past_t, *sv),
                    }

                    if let Some((_, _, eph)) = eph.borrow_mut().select(past_t, *sv) {
                        if let Some(tgd) = eph.tgd() {
                            debug!("{} ({}) - tgd: {}", past_t, *sv, tgd);
                            cd.set_group_delay(tgd);
                        }
                    }

                    match solver.resolve(past_t, &[cd]) {
                        Ok((_, pvt)) => {
                            let contrib = pvt
                                .sv
                                .iter()
                                .find(|contrib| contrib.sv == *sv)
                                .unwrap_or_else(|| {
                                    panic!("internal error: missing SV information")
                                });

                            let ref_observable = match contrib.signal {
                                Signal::Single(lhs) | Signal::Dual((lhs, _)) => {
                                    ref_rinex_observable(method == Method::PPP, lhs)
                                },
                            };

                            let refsys = pvt.clock_offset.to_seconds();
                            let refsv =
                                refsys + contrib.clock_correction.unwrap_or_default().to_seconds();

                            let elev_deg = contrib.elevation;
                            let azim_deg = contrib.azimuth;

                            // tracker
                            info!(
                                "({} ({}) : new pvt solution (elev={:.2}°, azim={:.2}°, refsv={:.3E}, refsys={:.3E})",
                                past_t, signal.sv, elev_deg, azim_deg, refsv, refsys,
                            );

                            // tropod model
                            let mdtr = contrib.tropo_bias.unwrap_or_default() / SPEED_OF_LIGHT_M_S;

                            // ionod
                            let mdio = 0.0;
                            // match pvt_data.iono_bias {
                            //     Some(IonosphereBias::Modeled(bias)) => Some(bias),
                            //     _ => None,
                            // };

                            let msio = None;
                            // match pvt_data.iono_bias {
                            //     Some(IonosphereBias::Measured(bias)) => Some(bias),
                            //     _ => None,
                            // };

                            // tracking
                            let data = FitObservation {
                                refsv,
                                refsys,
                                mdtr,
                                msio,
                                mdio,
                                epoch: past_t,
                                azimuth: azim_deg,
                                elevation: elev_deg,
                            };

                            if let Some(tracker) = trackers.get_mut(&(*sv, ref_observable.clone()))
                            {
                                tracker.new_observation(data);
                            } else {
                                let mut tracker =
                                    SVTracker::new(*sv).with_gap_tolerance(sampling_period);

                                tracker.new_observation(data);
                                trackers.insert((*sv, ref_observable.clone()), tracker);
                            }
                        },
                        Err(e) => {
                            // any PVT solution failure will introduce a gap in the track fitter
                            error!("{} - solver error: {}", past_t, e);
                        },
                    }
                } // for each sv

                sv_observations.clear();
            } // collecting
        } // new epoch

        let carrier = Carrier::from_observable(signal.sv.constellation, &signal.observable);
        if carrier.is_err() {
            continue;
        }

        let carrier = carrier.unwrap();
        let rtk_carrier = cast_rtk_carrier(carrier);

        if let Some((_, observations)) = sv_observations
            .iter_mut()
            .filter(|(k, _)| **k == signal.sv)
            .reduce(|k, _| k)
        {
            if let Some(observation) = observations
                .iter_mut()
                .filter(|k| k.carrier == rtk_carrier)
                .reduce(|k, _| k)
            {
                match signal.observable {
                    Observable::PhaseRange(_) => {
                        observation.ambiguity = None;
                        observation.phase_range_m = Some(signal.value);
                    },
                    Observable::PseudoRange(_) => {
                        observation.pseudo_range_m = Some(signal.value);
                    },
                    Observable::Doppler(_) => {
                        observation.doppler = Some(signal.value);
                    },
                    _ => {},
                }
            } else {
                match signal.observable {
                    Observable::PhaseRange(_) => {
                        observations.push(Observation::ambiguous_phase_range(
                            rtk_carrier,
                            signal.value,
                            None,
                        ));
                    },
                    Observable::PseudoRange(_) => {
                        observations.push(Observation::pseudo_range(
                            rtk_carrier,
                            signal.value,
                            None,
                        ));
                    },
                    Observable::Doppler(_) => {
                        observations.push(Observation::doppler(rtk_carrier, signal.value, None));
                    },
                    _ => {},
                }
            }
        } else {
            match signal.observable {
                Observable::PhaseRange(_) => {
                    sv_observations.insert(
                        signal.sv,
                        vec![Observation::ambiguous_phase_range(
                            rtk_carrier,
                            signal.value,
                            None,
                        )],
                    );
                },
                Observable::PseudoRange(_) => {
                    sv_observations.insert(
                        signal.sv,
                        vec![Observation::pseudo_range(rtk_carrier, signal.value, None)],
                    );
                },
                Observable::Doppler(_) => {
                    sv_observations.insert(
                        signal.sv,
                        vec![Observation::doppler(rtk_carrier, signal.value, None)],
                    );
                },
                _ => {},
            }
        }

        // update only on new epochs
        if index > 0 && t > past_t {
            if collecting {
                if t > next_period_start {
                    release = true;
                    // end of period: release attempt
                    for ((sv, sv_ref), tracker) in trackers.iter_mut() {
                        match tracker.fit() {
                            Ok(fitted) => {
                                debug!(
                                    "{}({}) - new CGGTTS fit - azim={:.3}° - elev={:.3}° - refsv={:.5E}s srsv={:.5E}s/s",
                                    past_t,
                                    sv,
                                    fitted.azimuth_deg,
                                    fitted.elevation_deg,
                                    fitted.refsv_s,
                                    fitted.srsv_s_s,
                                );

                                let data = 0; // TODO
                                let class = CommonViewClass::SingleChannel; // TODO
                                let new_track = fitted.to_track(class, data, &sv_ref.to_string());

                                tracks.push(new_track);
                            },
                            Err(e) => {
                                error!("{}({}) - CGGTTS fit error: {}", past_t, sv, e);
                            },
                        }
                    }
                } else {
                    info!("{} - {} until CGGTTS release", t, next_period_start - t);
                }
            } else {
                // not collecting
                if t >= next_collection_start {
                    next_period_start = cv_calendar.next_period_start_after(t);
                    collecting = true;
                    debug!("{} - CGGTTS tracking started", t);
                    info!("{} - {} until CGGTTS release", t, next_period_start - t);
                }

                if !collecting {
                    debug!("{} - {} until next tracking", t, next_collection_start - t);
                }
            }
        }

        if release {
            // reset
            release = false;
            next_period_start = cv_calendar.next_period_start_after(t);
            next_collection_start = cv_calendar.next_data_collection_after(past_t);
            collecting = t > next_collection_start;

            if collecting {
                debug!("{} - CGGTTS tracking started", t);
            } else {
                debug!("{} - {} until next tracking", t, next_collection_start - t);
            }

            info!("{} - {} until CGGTTS release", t, next_period_start - t);
        }

        past_t = t;
    }

    Ok(tracks)
}
