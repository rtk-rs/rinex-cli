use clap::ArgMatches;
use rinex::prelude::RinexType;

use std::path::{Path, PathBuf};

use crate::{
    cli::{Cli, Context},
    fops::{dump_rinex_auto_generated_name, parse_rinex},
    preprocessing::rinex_preprocessing,
    Error,
};

/// Substract and format RINEX=RINEX(A)-RINEX(B)
pub fn diff(ctx: &Context, cli: &Cli, submatches: &ArgMatches) -> Result<(), Error> {
    let ctx_data = &ctx.data;

    let rinex_a = ctx_data
        .observation()
        .expect("RINEX (A) - (B) requires OBS RINEX files");

    let gzip = submatches.get_flag("gzip");
    let forced_short_v2 = submatches.get_flag("short");

    let path_b = submatches.get_one::<PathBuf>("file").unwrap();
    let mut rinex_b = parse_rinex(&path_b);

    assert_eq!(
        rinex_b.header.rinex_type,
        RinexType::ObservationData,
        "only applies to Observation RINEX!"
    );

    if cli.matches.get_flag("rnx2crx") {
        rinex_b.rnx2crnx_mut();
    }

    if cli.matches.get_flag("crx2rnx") {
        rinex_b.crnx2rnx_mut();
    }

    rinex_preprocessing(&mut rinex_b, &cli);

    let rinex_c = rinex_b
        .observations_substract(&rinex_a)
        .unwrap_or_else(|e| {
            panic!("diff failed with: {:?}", e);
        });

    let input_name = rinex_a.standard_filename(forced_short_v2, None, None);
    let input_path = Path::new(&input_name);
    dump_rinex_auto_generated_name(&ctx, input_path, &rinex_c, gzip, None);

    Ok(())
}
