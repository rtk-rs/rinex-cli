use crate::cli::{Cli, Context};
use clap::ArgMatches;
use std::cell::RefCell;
use std::fs::read_to_string;

mod buffer;
pub use buffer::Buffer;

mod snapshot;
pub use snapshot::{CenteredDataPoints, CenteredSnapshot};

mod eph;
use eph::EphemerisSource;

mod ppp; // precise point positioning
use ppp::{
    post_process::{post_process as ppp_post_process, Error as PPPPostError},
    Report as PPPReport,
};

#[cfg(feature = "cggtts")]
mod cggtts; // CGGTTS special solver

#[cfg(feature = "cggtts")]
use cggtts::{post_process as cggtts_post_process, Report as CggttsReport};

// mod rtk;
// pub use rtk::RemoteRTKReference;

mod orbit;
use orbit::Orbits;

mod clock;
use clock::Clock;
pub use clock::ClockStateProvider;

use rinex::{
    carrier::Carrier,
    prelude::{Constellation, Rinex},
};

use gnss_qc::prelude::QcExtraPage;

use gnss_rtk::prelude::{
    Bias, BiasRuntime, Carrier as RTKCarrier, Config, Duration, Epoch, Error as RTKError, KbModel,
    Method, Solver, TroposphereModel,
};

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Coords3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coords3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl CenteredDataPoints<Coords3d> for Coords3d {
    fn zero() -> Coords3d {
        Coords3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

struct BiasModel {}

impl Bias for BiasModel {
    fn ionosphere_bias_m(&self, _: &BiasRuntime) -> f64 {
        0.0
    }

    fn troposphere_bias_m(&self, rtm: &BiasRuntime) -> f64 {
        TroposphereModel::Niel.bias_m(rtm)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("solver error")]
    SolverError(#[from] RTKError),
    #[error("no solutions: check your settings or input")]
    NoSolutions,
    #[error("i/o error")]
    StdioError(#[from] std::io::Error),
    #[error("post process error")]
    PPPPost(#[from] PPPPostError),
}

/*
 * Converts `RTK Carrier` into compatible struct
 */
pub fn rtk_carrier_cast(carrier: RTKCarrier) -> Carrier {
    match carrier {
        RTKCarrier::L2 => Carrier::L2,
        RTKCarrier::L5 => Carrier::L5,
        RTKCarrier::L6 => Carrier::L6,
        RTKCarrier::E1 => Carrier::E1,
        RTKCarrier::E5 => Carrier::E5,
        RTKCarrier::E6 => Carrier::E6,
        RTKCarrier::E5A => Carrier::E5a,
        RTKCarrier::E5B => Carrier::E5b,
        RTKCarrier::B1I => Carrier::B1I,
        RTKCarrier::B2 => Carrier::B2,
        RTKCarrier::B3 => Carrier::B3,
        RTKCarrier::B2A => Carrier::B2A,
        RTKCarrier::B2iB2b => Carrier::B2I,
        RTKCarrier::B1aB1c => Carrier::B1A,
        RTKCarrier::L1 => Carrier::L1,
    }
}

/*
 * Converts `Carrier` into RTK compatible struct
 */
pub fn cast_rtk_carrier(carrier: Carrier) -> RTKCarrier {
    match carrier {
        Carrier::L2 => RTKCarrier::L2,
        Carrier::L5 => RTKCarrier::L5,
        Carrier::L6 => RTKCarrier::L6,
        Carrier::E1 => RTKCarrier::E1,
        Carrier::E5 => RTKCarrier::E5,
        Carrier::E6 => RTKCarrier::E6,
        Carrier::E5a => RTKCarrier::E5A,
        Carrier::E5b => RTKCarrier::E5B,
        Carrier::B1I => RTKCarrier::B1I,
        Carrier::B2 => RTKCarrier::B2,
        Carrier::B3 | Carrier::B3A => RTKCarrier::B3,
        Carrier::B2A => RTKCarrier::B2A,
        Carrier::B2I | Carrier::B2B => RTKCarrier::B2iB2b,
        Carrier::B1A | Carrier::B1C => RTKCarrier::B1aB1c,
        Carrier::L1 | _ => RTKCarrier::L1,
    }
}

// helper in reference signal determination
fn rtk_reference_carrier(carrier: RTKCarrier) -> bool {
    matches!(
        carrier,
        RTKCarrier::L1 | RTKCarrier::E1 | RTKCarrier::B1aB1c | RTKCarrier::B1I
    )
}

//use map_3d::{ecef2geodetic, rad2deg, Ellipsoid};

//pub fn tropo_components(meteo: Option<&Rinex>, t: Epoch, lat_ddeg: f64) -> Option<(f64, f64)> {
//    const MAX_LATDDEG_DELTA: f64 = 15.0;
//    let max_dt = Duration::from_hours(24.0);
//    let rnx = meteo?;
//    let meteo = rnx.header.meteo.as_ref().unwrap();
//
//    let delays: Vec<(Observable, f64)> = meteo
//        .sensors
//        .iter()
//        .filter_map(|s| match s.observable {
//            Observable::ZenithDryDelay => {
//                let (x, y, z, _) = s.position?;
//                let (lat, _, _) = ecef2geodetic(x, y, z, Ellipsoid::WGS84);
//                let lat = rad2deg(lat);
//                if (lat - lat_ddeg).abs() < MAX_LATDDEG_DELTA {
//                    let value = rnx
//                        .zenith_dry_delay()
//                        .filter(|(t_sens, _)| (*t_sens - t).abs() < max_dt)
//                        .min_by_key(|(t_sens, _)| (*t_sens - t).abs());
//                    let (_, value) = value?;
//                    debug!("{:?} lat={} zdd {}", t, lat_ddeg, value);
//                    Some((s.observable.clone(), value))
//                } else {
//                    None
//                }
//            },
//            Observable::ZenithWetDelay => {
//                let (x, y, z, _) = s.position?;
//                let (mut lat, _, _) = ecef2geodetic(x, y, z, Ellipsoid::WGS84);
//                lat = rad2deg(lat);
//                if (lat - lat_ddeg).abs() < MAX_LATDDEG_DELTA {
//                    let value = rnx
//                        .zenith_wet_delay()
//                        .filter(|(t_sens, _)| (*t_sens - t).abs() < max_dt)
//                        .min_by_key(|(t_sens, _)| (*t_sens - t).abs());
//                    let (_, value) = value?;
//                    debug!("{:?} lat={} zdd {}", t, lat_ddeg, value);
//                    Some((s.observable.clone(), value))
//                } else {
//                    None
//                }
//            },
//            _ => None,
//        })
//        .collect();
//
//    if delays.len() < 2 {
//        None
//    } else {
//        let zdd = delays
//            .iter()
//            .filter_map(|(obs, value)| {
//                if obs == &Observable::ZenithDryDelay {
//                    Some(*value)
//                } else {
//                    None
//                }
//            })
//            .reduce(|k, _| k)
//            .unwrap();
//
//        let zwd = delays
//            .iter()
//            .filter_map(|(obs, value)| {
//                if obs == &Observable::ZenithWetDelay {
//                    Some(*value)
//                } else {
//                    None
//                }
//            })
//            .reduce(|k, _| k)
//            .unwrap();
//
//        Some((zwd, zdd))
//    }
//}

/// Returns a [KbModel]
pub fn kb_model(nav: &Rinex, t: Epoch) -> Option<KbModel> {
    let (nav_key, model) = nav
        .nav_klobuchar_models_iter()
        .min_by_key(|(k_i, _)| (k_i.epoch - t).abs())?;

    Some(KbModel {
        h_km: {
            match nav_key.sv.constellation {
                Constellation::BeiDou => 375.0,
                // we only expect GPS or BDS here,
                // badly formed RINEX will generate errors in the solutions
                _ => 350.0,
            }
        },
        alpha: model.alpha,
        beta: model.beta,
    })
}

// /*
//  * Grabs nearest BD model (in time)
//  */
// pub fn bd_model(nav: &Rinex, t: Epoch) -> Option<BdModel> {
//     let (_, model) = nav
//         .nav_bdgim_models_iter()
//         .min_by_key(|(k_i, _)| (k_i.epoch - t).abs())?;

//     Some(BdModel { alpha: model.alpha })
// }

// /*
//  * Grabs nearest NG model (in time)
//  */
// pub fn ng_model(nav: &Rinex, t: Epoch) -> Option<NgModel> {
//     let (_, model) = nav
//         .nav_nequickg_models_iter()
//         .min_by_key(|(k_i, _)| (k_i.epoch - t).abs())?;

//     Some(NgModel { a: model.a })
// }

pub fn precise_positioning(
    _cli: &Cli,
    ctx: &Context,
    is_rtk: bool,
    matches: &ArgMatches,
) -> Result<QcExtraPage, Error> {
    // Load custom configuration script, or Default
    let cfg = match matches.get_one::<String>("cfg") {
        Some(fp) => {
            let content = read_to_string(fp)
                .unwrap_or_else(|e| panic!("failed to read configuration: {}", e));

            let mut cfg: Config = serde_json::from_str(&content)
                .unwrap_or_else(|e| panic!("failed to parse configuration: {}", e));

            /*
             * CGGTTS special case
             */
            #[cfg(not(feature = "cggtts"))]
            if matches.get_flag("cggtts") {
                panic!("--cggtts option not available: compile with cggtts option");
            }

            info!("Using custom solver configuration: {:#?}", cfg);
            cfg
        },
        None => {
            let method = Method::default();
            let mut cfg = Config::static_ppp_preset(method);

            /*
             * CGGTTS special case
             */
            #[cfg(not(feature = "cggtts"))]
            if matches.get_flag("cggtts") {
                panic!("--cggtts option not available: compile with cggtts option");
            }

            info!("Using {:?} default preset: {:#?}", method, cfg);
            cfg
        },
    };

    /* Verify requirements and print helpful comments */
    assert!(
        ctx.data.observation().is_some(),
        "Positioning requires Observation RINEX"
    );

    if !is_rtk {
        assert!(
            ctx.data.brdc_navigation().is_some(),
            "Positioning requires Navigation RINEX"
        );
    }

    if let Some(obs_rinex) = ctx.data.observation() {
        if let Some(obs_header) = &obs_rinex.header.obs {
            if let Some(time_of_first_obs) = obs_header.timeof_first_obs {
                if let Some(clk_rinex) = ctx.data.clock() {
                    if let Some(clk_header) = &clk_rinex.header.clock {
                        if let Some(time_scale) = clk_header.timescale {
                            if time_scale == time_of_first_obs.time_scale {
                                info!("Temporal PPP compliancy");
                            } else {
                                error!("Working with different timescales in OBS/CLK RINEX is not PPP compatible and will generate tiny errors");
                                warn!("Consider using OBS/CLK RINEX files expressed in the same timescale for optimal results");
                            }
                        }
                    }
                } else if let Some(sp3) = ctx.data.sp3() {
                    if ctx.data.sp3_has_clock() {
                        if sp3.header.timescale == time_of_first_obs.time_scale {
                            info!("Temporal PPP compliancy");
                        } else {
                            error!("Working with different timescales in OBS/SP3 is not PPP compatible and will generate tiny errors");
                            if sp3.header.epoch_interval >= Duration::from_seconds(300.0) {
                                warn!("Interpolating clock states from low sample rate SP3 will most likely introduce errors");
                            }
                        }
                    }
                }
            }
        }
    }

    // print config to be used
    info!("Using {:?} method", cfg.method);

    // create data providers
    let eph = RefCell::new(EphemerisSource::from_ctx(ctx));
    let clocks = Clock::new(&ctx, &eph);
    let orbits = Orbits::new(&ctx, &eph);

    // let mut rtk_reference = RemoteRTKReference::from_ctx(&ctx);

    // reference point is mandatory to CGGTTS opmode
    #[cfg(feature = "cggtts")]
    if matches.get_flag("cggtts") {
        if ctx.rx_orbit.is_none() {
            panic!(
                "cggtts needs a reference point (x0, y0, z0).
If your dataset does not describe one, you can manually describe one, see --help."
            );
        }
    }

    let bias_model = BiasModel {};

    let apriori = ctx.rx_orbit;

    let apriori_ecef_m = match apriori {
        Some(apriori) => {
            let pos_vel = apriori.to_cartesian_pos_vel();
            Some((pos_vel[0] * 1.0E3, pos_vel[1] * 1.0E3, pos_vel[2] * 1.0E3))
        },
        None => None,
    };

    let solver = Solver::new_almanac_frame(
        cfg.clone(),
        ctx.data.almanac.clone(),
        ctx.data.earth_cef,
        orbits,
        bias_model,
        apriori_ecef_m,
    );

    #[cfg(feature = "cggtts")]
    if matches.get_flag("cggtts") {
        //* CGGTTS special opmode */
        let tracks = cggtts::resolve(ctx, &eph, clocks, solver, cfg.method, matches)?;
        if !tracks.is_empty() {
            cggtts_post_process(&ctx, &tracks, matches)?;
            let report = CggttsReport::new(&ctx, &tracks);
            return Ok(report.formalize());
        } else {
            error!("solver did not generate a single solution");
            error!("verify your input data and configuration setup");
            return Err(Error::NoSolutions);
        }
    }

    /* PPP */
    let solutions = ppp::resolve(ctx, &eph, clocks, solver);
    if !solutions.is_empty() {
        ppp_post_process(&ctx, &solutions, matches)?;
        let report = PPPReport::new(&cfg, &ctx, &solutions);
        Ok(report.formalize())
    } else {
        error!("solver did not generate a single solution");
        error!("verify your input data and configuration setup");
        Err(Error::NoSolutions)
    }
}
