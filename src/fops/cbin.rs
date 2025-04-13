use crate::cli::Context;
use crate::fops::custom_prod_attributes;
use crate::fops::output_filename;
use crate::Error;
use clap::ArgMatches;
use gnss_qc::prelude::{Filter, Preprocessing, ProductType};
use rinex::prelude::Duration;
use rinex::prod::DetailedProductionAttributes;

/// Constellation / timescale batch creation
pub fn constell_timescale_binning(
    ctx: &Context,
    matches: &ArgMatches,
    submatches: &ArgMatches,
) -> Result<(), Error> {
    let ctx_data = &ctx.data;

    for (product, dir) in [
        (ProductType::Observation, "OBSERVATIONS"),
        (ProductType::BroadcastNavigation, "BRDC"),
    ] {
        // input data determination
        if let Some(rinex) = ctx_data.rinex(product) {
            // create work dir
            ctx.workspace.create_subdir(dir);

            // production attributes: initialize Batch counter
            let mut prod = custom_prod_attributes(rinex, submatches);
            if let Some(ref mut details) = prod.v3_details {
                details.batch = 0_u8;
            } else {
                let mut details = DetailedProductionAttributes::default();
                details.batch = 0_u8;
                prod.v3_details = Some(details);
            };

            // run binning algorithm
        }
    }
    Ok(())
}
