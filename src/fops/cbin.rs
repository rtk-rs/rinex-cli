use clap::ArgMatches;
use std::path::Path;

use gnss_qc::prelude::{
    Filter as QcFilter, FilterItem as QcFilterItem, MaskOperand as QcMaskOperand, Preprocessing,
    ProductType, TimeScale, Timeshift,
};

use crate::{cli::Context, fops::dump_rinex_auto_generated_name, Error};

/// Constellation / timescale batch creation
pub fn constell_timescale_binning(ctx: &Context, submatches: &ArgMatches) -> Result<(), Error> {
    let ctx_data = &ctx.data;

    let forced_short_v2 = submatches.get_flag("short");
    let gzip = submatches.get_flag("gzip");

    let ts_binning = submatches.get_flag("tsbin");

    let prefered_ts = if let Some(ts) = submatches.get_one::<TimeScale>("timescale") {
        Some(ts)
    } else {
        None
    };

    if ts_binning && prefered_ts.is_some() {
        panic!("timescale binning (--ts) and prefered timescale (--timescale) are incompatible!");
    }

    for product in [
        ProductType::Observation,
        ProductType::BroadcastNavigation,
        ProductType::HighPrecisionClock,
    ] {
        // input data determination
        if let Some(rinex) = ctx_data.rinex(product) {
            // split on a constellation basis
            for constellation in rinex.constellations_iter() {
                let custom_subdir = format!("{:X}", constellation);
                ctx.workspace.create_subdir(&custom_subdir);

                // design filter
                debug!("{} constellation binning..", constellation);

                let filter = QcFilter::mask(
                    QcMaskOperand::Equals,
                    QcFilterItem::ConstellationItem(vec![constellation]),
                );

                // apply this filter
                let mut focused = rinex.filter(&filter);

                // rework
                focused.header.constellation = Some(constellation);

                // possible timescale shift
                if ts_binning || prefered_ts.is_some() {
                    if constellation.timescale().is_none() {
                        // timescale not supported: abort
                        error!("{} timescale is not supported yet. Skipped.", constellation);
                        continue;
                    }
                }

                // prefered timescale shift
                if let Some(prefered_ts) = prefered_ts {
                    debug!("Transposition to {}..", prefered_ts);
                    focused.timeshift_mut(*prefered_ts);
                }

                // timescale binning
                if ts_binning {
                    let ts = constellation.timescale().unwrap(); // infaillible at this point

                    // transpose
                    debug!("Transposition to {}..", ts);
                    focused.timeshift_mut(ts);
                }

                let standard_name = focused.standard_filename(forced_short_v2, None, None);
                let input_path = Path::new(&standard_name);

                dump_rinex_auto_generated_name(
                    &ctx,
                    input_path,
                    &focused,
                    gzip,
                    Some(custom_subdir),
                );
            }
        }
    }

    if let Some(sp3) = ctx.data.sp3() {
        // split per constellation
        for constellation in sp3.constellations_iter() {
            let custom_subdir = format!("{:X}", constellation);
            ctx.workspace.create_subdir(&custom_subdir);

            // design filter
            debug!("{} constellation binning..", constellation);

            let filter = QcFilter::mask(
                QcMaskOperand::Equals,
                QcFilterItem::ConstellationItem(vec![constellation]),
            );

            //apply
            let mut focused = sp3.filter(&filter);

            // rework
            focused.header.constellation = constellation;

            // possible timescale shift
            if ts_binning || prefered_ts.is_some() {
                if focused.header.constellation.timescale().is_none() {
                    // abort here: timescale not supported
                    error!("{} timescale is not supported yet. Skipped.", constellation);
                    continue;
                }
            }

            // prefered timescale shift
            if let Some(prefered_ts) = prefered_ts {
                debug!("Transposition to {}..", prefered_ts);
                focused.timeshift_mut(*prefered_ts);
            }

            // timescale binning
            if ts_binning {
                let ts = constellation.timescale().unwrap(); // infaillble at this point

                debug!("Transposition to {}..", ts);
                focused.timeshift_mut(ts);
            }

            // filename
            // let standard_name = focused.standard_filename();
            // dump_sp3(&standard_name, gzip, Some(custom_dir));
        }
    }

    Ok(())
}
