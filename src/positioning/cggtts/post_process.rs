//! CGGTTS track formation and post processing
use crate::cli::Context;
use clap::ArgMatches;
use std::io::Write;

use itertools::Itertools;

use cggtts::prelude::{Hardware, Header, ReferenceTime, Track, CGGTTS};

/// CGGTTS solutions post processing
pub fn post_process(
    ctx: &Context,
    tracks: &Vec<Track>,
    matches: &ArgMatches,
) -> std::io::Result<()> {
    let obs_data = ctx.data.observation().unwrap();

    let mut header = Header::default();

    header.nb_channels = 1; // TODO
    header.reference_frame = "WGS84".to_string();

    header = header.with_comment(&format!(
        "rinex-cli v{} - https://github.com/rtk-rs",
        env!("CARGO_PKG_VERSION")
    ));

    // TODO
    // .reference_time({
    //     if let Some(utck) = matches.get_one::<String>("utck") {
    //         ReferenceTime::UTCk(utck.clone())
    //     } else if let Some(clock) = matches.get_one::<String>("clock") {
    //         ReferenceTime::Custom(clock.clone())
    //     } else {
    //         ReferenceTime::Custom("Unknown".to_string())
    //     }
    // })

    // agency customization
    if let Some(custom) = matches.get_one::<String>("agency") {
        header = header.with_station(custom);
    } else {
        let stem = Context::context_stem(&ctx.data);
        let value = if let Some(index) = stem.find('_') {
            stem[..index].to_string()
        } else {
            "LAB".to_string()
        };
        header = header.with_station(&value);
    };

    // TODO
    // header = header.with_receiver_hardware(rx);

    let constellations = tracks.iter().map(|trk| trk.sv.constellation).unique();

    for constellation in constellations {
        let mut cggtts = CGGTTS::default().with_header(header.clone());

        for trk in tracks.iter() {
            if trk.sv.constellation == constellation {
                cggtts.tracks.push(trk.clone());
            }
        }

        // TODO
        let name = cggtts.standardized_file_name(None, None);

        cggtts
            .to_file(&name)
            .unwrap_or_else(|e| panic!("CGGTTS formatting error: {}", e));
    }

    Ok(())
}
