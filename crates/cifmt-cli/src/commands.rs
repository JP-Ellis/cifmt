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

pub(crate) mod format;
pub(crate) mod version;

use anyhow::Result;

/// Available subcommands for the CLI.
#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Format tool output for CI platforms
    Format(format::Args),

    /// Show version information
    Version(version::Args),
}

impl Default for Command {
    fn default() -> Self {
        Command::Format(format::Args {
            tool: None,
            detect: true,
        })
    }
}

/// Output format for command results.
#[derive(Debug, clap::ValueEnum, Copy, Clone, Default)]
pub enum OutputFormat {
    /// Plain text output format.
    #[default]
    Text,
    /// JSON output format.
    Json,
}

impl Command {
    /// Execute the command.
    pub(crate) fn execute(self) -> Result<()> {
        match self {
            Command::Format(args) => format::execute(args),
            Command::Version(args) => version::execute(args),
        }
    }
}
