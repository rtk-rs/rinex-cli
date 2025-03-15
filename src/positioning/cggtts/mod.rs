//! CGGTTS special resolution opmoode.
use clap::ArgMatches;

use std::{cell::RefCell, collections::HashMap, str::FromStr};

mod post_process;
pub use post_process::post_process;

mod report;
pub use report::Report;

use gnss::prelude::{Constellation, SV};

use rinex::{
    carrier::Carrier,
    prelude::{Observable, TimeScale},
};

use gnss_rtk::prelude::{
    Bias, Candidate, Carrier as RTKCarrier, Duration, Method, Observation, OrbitSource, Solver,
    SPEED_OF_LIGHT_M_S,
};

use cggtts::prelude::{
    CommonViewCalendar, CommonViewClass, FittedData, Observation as FitObservation, SVTracker,
    Track,
};

use hifitime::Unit;

use crate::{
    cli::Context,
    positioning::{
        cast_rtk_carrier, ClockStateProvider, EphemerisSource, Error as PositioningError,
    },
};

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
    let mut collecting = false;

    let C1C = Observable::from_str("C1C").unwrap();

    let cv_calendar = CommonViewCalendar::bipm();

    let mut tracks = Vec::<Track>::new();
    let mut trackers = HashMap::<(SV, Observable), SVTracker>::new();

    let mut next_reset = cv_calendar.time_to_next_data_collection(t0);

    let mut sv_reference = HashMap::<SV, Observable>::new();
    let mut sv_observations = HashMap::<SV, Vec<Observation>>::new();

    for (index, (t, signal)) in obs_data.signal_observations_sampling_ok_iter().enumerate() {
        if index > 0 && t > past_t {
            info!("{:?} - new epoch", past_t);

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
                    Ok((t, pvt)) => {
                        let contrib = pvt
                            .sv
                            .iter()
                            .find(|contrib| contrib.sv == *sv)
                            .unwrap_or_else(|| panic!("internal error: missing SV information"));

                        let refsys = pvt.clock_offset.to_seconds();
                        let refsv =
                            refsys + contrib.clock_correction.unwrap_or_default().to_seconds();

                        let elev_deg = contrib.elevation;
                        let azim_deg = contrib.azimuth;

                        // tracker
                        info!(
                            "({:?} ({}) : new pvt solution (elev={:.2}°, azim={:.2}°, refsv={:.3E}, refsys={:.3})",
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

                        if let Some(tracker) = trackers.get_mut(&(cd.sv, C1C.clone())) {
                            tracker.new_observation(data);
                        } else {
                            let mut tracker = SVTracker::new(cd.sv, Some(sampling_period));
                            tracker.new_observation(data);
                            trackers.insert((cd.sv, C1C.clone()), tracker);
                        }
                    },
                    Err(e) => {
                        // any PVT solution failure will introduce a gap in the track fitter
                        error!("{:?} - solver error: {}", past_t, e);
                    },
                }
            } // for each sv
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
            // first SV encounter
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

                    if method == Method::PPP {
                        if matches!(
                            rtk_carrier,
                            RTKCarrier::L1 | RTKCarrier::E1 | RTKCarrier::B1aB1c | RTKCarrier::B1I
                        ) {
                            sv_reference.insert(signal.sv, signal.observable.clone());
                        }
                    }
                },
                Observable::PseudoRange(_) => {
                    sv_observations.insert(
                        signal.sv,
                        vec![Observation::pseudo_range(rtk_carrier, signal.value, None)],
                    );

                    if method != Method::PPP {
                        if matches!(
                            rtk_carrier,
                            RTKCarrier::L1 | RTKCarrier::E1 | RTKCarrier::B1aB1c | RTKCarrier::B1I
                        ) {
                            sv_reference.insert(signal.sv, signal.observable.clone());
                        }
                    }
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

        past_t = t;
    }
    Ok(tracks)
}
