//! CI message formatter CLI.

use clap::Parser;
use std::process::ExitCode;

pub(crate) mod commands;
mod logging;
pub mod version;

#[derive(clap::Parser, Debug)]
#[command(author, name = "cifmt")]
#[command(about = "CI message formatter")]
#[command(version = version::Version::default())]
struct GlobalArgs {
    /// Increase verbosity level: -v: info, -vv: debug with timing
    #[clap(short, long, global = true, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[command(subcommand)]
    command: commands::Command,
}

fn main() -> ExitCode {
    let args = GlobalArgs::parse();

    logging::setup_tracing(args.verbosity);

    match args.command.execute() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            tracing::error!("Error executing command: {}", e);
            ExitCode::FAILURE
        }
    }
}
