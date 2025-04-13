mod cbin;
mod diff;
mod filegen;
mod merge;
mod split;
mod tbin;

#[cfg(feature = "csv")]
pub mod csv;

pub use cbin::constell_timescale_binning;
pub use diff::diff;
pub use filegen::filegen;
pub use merge::merge;
pub use split::split;
pub use tbin::time_binning;

use std::path::Path;

use clap::ArgMatches;

use rinex::{
    prelude::Rinex,
    prod::{DataSource, DetailedProductionAttributes, ProductionAttributes, FFU, PPU},
};

use sp3::SP3;

use crate::Context;

/// Shared method to parse a RINEX file
pub fn parse_rinex(path: &Path) -> Rinex {
    let extension = path
        .extension()
        .unwrap_or_else(|| panic!("failed to determine file extension: {}", path.display()))
        .to_string_lossy()
        .to_string();

    let rinex = if extension == "gz" {
        Rinex::from_gzip_file(&path)
            .unwrap_or_else(|e| panic!("Failed to parse gzip compressed RINEX: {}", e))
    } else {
        Rinex::from_file(&path).unwrap_or_else(|e| panic!("Failed to parse RINEX: {}", e))
    };

    rinex
}

/// Shared method to dump a RINEX file into the workspace
pub fn dump_rinex_auto_generated_name(
    ctx: &Context,
    input_path: &Path,
    rinex: &Rinex,
    gzip: bool,
    custom_subdir: Option<String>,
) {
    let suffix = input_path
        .file_name()
        .expect("failed to determine output filename")
        .to_string_lossy()
        .to_string();

    let mut output_path = ctx.workspace.root.clone();

    if let Some(subdir) = custom_subdir {
        output_path = output_path.join(subdir);
    }

    output_path = output_path.join(suffix);

    let mut output_path = output_path.to_string_lossy().to_string();

    if gzip {
        output_path.push_str(".gz");
    }

    if gzip {
        rinex
            .to_gzip_file(&output_path)
            .unwrap_or_else(|e| panic!("Failed to format {}: {}", output_path, e));
    } else {
        rinex
            .to_file(&output_path)
            .unwrap_or_else(|e| panic!("Failed to format {}: {}", output_path, e));
    }

    info!("\"{}\" has been generated", output_path);
}

/*
 * Parses share RINEX production attributes.
 * This helps accurate file production,
 * and also allows customization from files that did not originally follow
 * standard naming conventions
 */
fn custom_prod_attributes(rinex: &Rinex, matches: &ArgMatches) -> ProductionAttributes {
    // Start from smartly guessed attributes and replace
    // manually customized fields
    let mut opts = rinex.guess_production_attributes();

    if let Some(agency) = matches.get_one::<String>("agency") {
        opts.name = agency.to_string();
    }

    if let Some(country) = matches.get_one::<String>("country") {
        if let Some(ref mut details) = opts.v3_details {
            details.country = country[..3].to_string();
        } else {
            let mut default = DetailedProductionAttributes::default();
            default.country = country[..3].to_string();
            opts.v3_details = Some(default);
        }
    }

    if let Some(batch) = matches.get_one::<u8>("batch") {
        if let Some(ref mut details) = opts.v3_details {
            details.batch = *batch;
        } else {
            let mut default = DetailedProductionAttributes::default();
            default.batch = *batch;
            opts.v3_details = Some(default);
        }
    }

    if let Some(src) = matches.get_one::<DataSource>("source") {
        if let Some(ref mut details) = opts.v3_details {
            details.data_src = *src;
        } else {
            let mut default = DetailedProductionAttributes::default();
            default.data_src = *src;
            opts.v3_details = Some(default);
        }
    }

    if let Some(ppu) = matches.get_one::<PPU>("ppu") {
        if let Some(ref mut details) = opts.v3_details {
            details.ppu = *ppu;
        } else {
            let mut default = DetailedProductionAttributes::default();
            default.ppu = *ppu;
            opts.v3_details = Some(default);
        }
    }

    if let Some(ffu) = matches.get_one::<FFU>("ffu") {
        if let Some(ref mut details) = opts.v3_details {
            details.ffu = Some(*ffu);
        } else {
            let mut default = DetailedProductionAttributes::default();
            default.ffu = Some(*ffu);
            opts.v3_details = Some(default);
        }
    }

    opts
}

/// Returns output filename to be generated, for this kind of product.
fn output_filename(
    rinex: &Rinex,
    matches: &ArgMatches,
    submatches: &ArgMatches,
    prod: ProductionAttributes,
) -> String {
    // Parse possible custom opts
    let short = submatches.get_flag("short");
    let csv = submatches.get_flag("csv");

    let suffix = if csv {
        if submatches.get_flag("gzip") {
            Some(".csv.gz")
        } else {
            Some(".csv")
        }
    } else {
        if submatches.get_flag("gzip") {
            Some(".gz")
        } else {
            None
        }
    };

    // Prefer manual user input, otherwise, use smart determination.
    if let Some(custom) = matches.get_one::<String>("output-name") {
        if let Some(suffix) = suffix {
            format!("{}{}", custom, suffix)
        } else {
            custom.to_string()
        }
    } else {
        debug!("{:?}", prod);
        rinex.standard_filename(short, suffix, Some(prod))
    }
}
