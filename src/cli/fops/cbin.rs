use clap::{value_parser, Arg, ArgAction, Command};
use rinex::prelude::Duration;

use super::{SHARED_DATA_ARGS, SHARED_GENERAL_ARGS};

pub fn subcommand() -> Command {
    Command::new("cbin")
        .arg_required_else_help(false)
        .about("Constellation / NAV binning. Split files into a batch of individual Constellation/Timescales.")
        .arg(
            Arg::new("tsbin")
                .long("tsbin")
                .action(ArgAction::SetTrue)
        )
        .next_help_heading("Production Environment")
        .args(SHARED_GENERAL_ARGS.iter())
        .next_help_heading("Data context")
        .args(SHARED_DATA_ARGS.iter())
}
