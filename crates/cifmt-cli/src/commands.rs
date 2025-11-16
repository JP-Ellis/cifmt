//! Command definitions for the cifmt application.
//!
//! This module defines the command-line interface (CLI) commands
//! supported by the `cifmt` application using the `clap` crate.

// The general design for commands is to:
//
// - Create a submodule for each command under the `commands` module containing:
//   - An `Args` struct defining the command-line arguments for the command.
//   - An `execute` function that takes the `Args` struct and performs the
//     command's functionality.
// - Add the command to the `Command` enum in this module.

pub(crate) mod version;

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Show version information
    Version(version::Args),
}

#[derive(Debug, clap::ValueEnum, Copy, Clone, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

impl Command {
    /// Execute the command.
    pub(crate) fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Version(args) => {
                version::execute(args)?;
            }
        }
        Ok(())
    }
}
