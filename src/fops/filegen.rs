use crate::cli::Context;
use crate::fops::{custom_prod_attributes, output_filename};
use crate::Error;
use clap::ArgMatches;
use gnss_qc::prelude::ProductType;

#[cfg(feature = "csv")]
use crate::fops::csv::{
    write_joint_nav_obs_rinex as write_joint_nav_obs_rinex_csv,
    write_obs_rinex as write_obs_rinex_csv, write_raw_nav_rinex as write_raw_nav_rinex_csv,
};

/// Dumps current (possibly preprocessed) context
/// into either RINEX / SP3 format (maintaining consistent format) or CSV
pub fn filegen(ctx: &Context, matches: &ArgMatches, submatches: &ArgMatches) -> Result<(), Error> {
    #[cfg(feature = "csv")]
    if submatches.get_flag("csv") {
        write_csv(ctx, matches, submatches)?;
        return Ok(());
    }

    #[cfg(not(feature = "csv"))]
    if submatches.get_flag("csv") {
        panic!("Not available. Activate `csv` feature first.")
    }

    write(ctx, matches, submatches)?;

    Ok(())
}

#[cfg(feature = "csv")]
fn write_csv(ctx: &Context, matches: &ArgMatches, submatches: &ArgMatches) -> Result<(), Error> {
    let ctx_data = &ctx.data;

    // OBS RINEX
    if let Some(rinex) = ctx_data.rinex(ProductType::Observation) {
        ctx.workspace.create_subdir("OBSERVATIONS");

        let prod = custom_prod_attributes(rinex, submatches);

        let output = ctx
            .workspace
            .root
            .join("OBSERVATIONS")
            .join(output_filename(rinex, matches, submatches, prod));

        write_obs_rinex_csv(rinex, &output)?;

        info!(
            "{} dumped in {}",
            ProductType::Observation,
            output.display()
        );
    }

    // NAV RINEX
    if let Some(brdc) = ctx_data.rinex(ProductType::BroadcastNavigation) {
        ctx.workspace.create_subdir("BRDC");

        let prod = custom_prod_attributes(brdc, submatches);
        let output_name = output_filename(brdc, matches, submatches, prod);

        let output = ctx.workspace.root.join("BRDC").join(&output_name);

        write_raw_nav_rinex_csv(brdc, &output)?;

        info!(
            "{} dumped in {}",
            ProductType::BroadcastNavigation,
            output.display()
        );

        if let Some(obs) = ctx_data.rinex(ProductType::Observation) {
            ctx.workspace.create_subdir("BRDC+OBS");

            let output = ctx.workspace.root.join("BRDC+OBS").join(&output_name);

            write_joint_nav_obs_rinex_csv(brdc, obs, &output)?;

            info!(
                "{} dumped in {}",
                ProductType::BroadcastNavigation,
                output.display()
            );
        }
    }

    Ok(())
}

fn write(ctx: &Context, matches: &ArgMatches, submatches: &ArgMatches) -> Result<(), Error> {
    let ctx_data = &ctx.data;
    for (product, dir) in [
        (ProductType::DORIS, "DORIS"),
        (ProductType::Observation, "OBSERVATIONS"),
        (ProductType::MeteoObservation, "METEO"),
        (ProductType::BroadcastNavigation, "BRDC"),
        (ProductType::HighPrecisionClock, "CLOCK"),
        (ProductType::HighPrecisionOrbit, "SP3"),
        (ProductType::IONEX, "IONEX"),
        (ProductType::ANTEX, "ANTEX"),
    ] {
        if let Some(rinex) = ctx_data.rinex(product) {
            ctx.workspace.create_subdir(dir);
            let prod = custom_prod_attributes(rinex, submatches);
            let filename = output_filename(rinex, matches, submatches, prod);

            let output_path = ctx
                .workspace
                .root
                .join(dir)
                .join(filename)
                .to_string_lossy()
                .to_string();

            if submatches.get_flag("gzip") {
                rinex.to_gzip_file(&output_path).unwrap_or_else(|_| {
                    panic!("failed to generate {} RINEX \"{}\"", product, output_path)
                });
            } else {
                rinex.to_file(&output_path).unwrap_or_else(|_| {
                    panic!("failed to generate {} RINEX \"{}\"", product, output_path)
                });
            }

            info!("{} RINEX \"{}\" has been generated", product, output_path);
        }
    }
    Ok(())
}
