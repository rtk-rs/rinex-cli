use clap::ArgMatches;
use std::path::Path;

use gnss_qc::prelude::{
    Filter as QcFilter, FilterItem as QcFilterItem, MaskOperand as QcMaskOperand, Preprocessing,
    ProductType,
};

use crate::{cli::Context, fops::dump_rinex_auto_generated_name, Error};

/// Constellation / timescale batch creation
pub fn constell_timescale_binning(ctx: &Context, submatches: &ArgMatches) -> Result<(), Error> {
    let ctx_data = &ctx.data;

    let forced_short_v2 = submatches.get_flag("short");
    let gzip = submatches.get_flag("gzip");

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

                // design this filter
                let filter = QcFilter::mask(
                    QcMaskOperand::Equals,
                    QcFilterItem::ConstellationItem(vec![constellation]),
                );

                // apply this filter
                let mut focused = rinex.filter(&filter);

                // rework
                focused.header.constellation = Some(constellation);

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

    // for product in [ProductType::HighPrecisionOrbit] {

    //     // input data determination
    //     if let Some(sp3) = ctx_data.sp3() {
    //         // split on a constellation basis
    //         for constellation in sp3.constellations_iter() {
    //             // design this filter
    //             let filter = QcFilter::mask(
    //                 QcMaskOperand::Equals,
    //                 QcFilterItem::ConstellationItem(vec![constellation]),
    //             );

    //             // apply this filter
    //             let focused = sp3.filter(&filter);

    //         }
    //     }
    // }

    Ok(())
}
