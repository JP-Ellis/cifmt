//! Format command implementation.
//!
//! This module handles the formatting of tool output for CI platforms.

use anyhow::Result;
use cifmt::ci::{GitHub, Plain, Platform};
use cifmt::tool::{self, DynTool};
use std::io::{self, Read, Write};

/// Size of each read chunk from stdin.
const CHUNK_SIZE: usize = 16 * 1024;

/// Arguments for the format command.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The tool format to use.
    ///
    /// If not specified, the tool will be automatically detected from the
    /// input.
    #[arg(value_enum, group = "tool_selection")]
    pub tool: Option<ToolFormat>,

    /// Automatically detect the tool format from the input.
    #[arg(long, group = "tool_selection")]
    pub detect: bool,
}

/// Supported tool formats.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum ToolFormat {
    /// Cargo test (libtest) JSON format.
    CargoLibtest,
    /// Cargo check/build JSON format.
    CargoCheck,
}

impl ToolFormat {
    /// Convert the tool format to a dynamic tool instance for the specified platform.
    ///
    /// # Returns
    ///
    /// A boxed dynamic tool that can parse and format messages for the platform.
    fn into_dyn_tool<P: Platform + 'static>(self) -> Box<dyn DynTool<P>>
    where
        tool::CargoCheck: DynTool<P>,
        tool::CargoLibtest: DynTool<P>,
    {
        match self {
            Self::CargoLibtest => Box::new(tool::CargoLibtest::default()),
            Self::CargoCheck => Box::new(tool::CargoCheck::default()),
        }
    }
}

/// Execute the format command.
///
/// This function reads from stdin as a stream, parses the input according to
/// the specified or detected tool format, and writes the formatted output to
/// stdout.
///
/// # Arguments
///
/// * `args` - The command-line arguments for the format command.
///
/// # Returns
///
/// Returns `Ok(())` if the command executes successfully, otherwise returns an
/// error.
///
/// # Errors
///
/// This function will return an error if:
/// - Reading from stdin fails
/// - Auto-detection is enabled but no tool format could be detected
/// - Parsing the input fails
/// - Writing to stdout fails
#[tracing::instrument(skip(args))]
#[expect(
    clippy::needless_pass_by_value,
    reason = "follows common pattern for command execution functions"
)]
pub(crate) fn execute(args: Args) -> Result<()> {
    let mut reader = io::stdin().lock();
    let mut writer = io::stdout().lock();
    let mut buffer = Vec::with_capacity(CHUNK_SIZE);

    // Detect platform and dispatch to the appropriate typed handler
    if GitHub::from_env().is_some() {
        execute_with_platform::<GitHub>(&args, &mut reader, &mut writer, &mut buffer)
    } else {
        execute_with_platform::<Plain>(&args, &mut reader, &mut writer, &mut buffer)
    }
}

/// Execute the format command with a specific platform type.
fn execute_with_platform<P: Platform + 'static>(
    args: &Args,
    reader: &mut impl Read,
    writer: &mut impl Write,
    buffer: &mut Vec<u8>,
) -> Result<()>
where
    tool::CargoCheck: DynTool<P>,
    tool::CargoLibtest: DynTool<P>,
{
    let platform = P::from_env().ok_or_else(|| anyhow::anyhow!("Failed to detect platform"))?;
    tracing::info!("Using platform: {}", platform);

    // Get tool (either detected or specified)
    let mut tool: Box<dyn DynTool<P>> = if args.detect {
        // Read initial buffer for detection
        buffer.resize(CHUNK_SIZE, 0);
        let n = reader.read(buffer)?;
        buffer.truncate(n);
        tool::detect::<P>(buffer)?
    } else if let Some(tool_format) = args.tool {
        tool_format.into_dyn_tool::<P>()
    } else {
        anyhow::bail!("Either --detect or a tool format must be specified");
    };

    tracing::info!("Using tool: {}", tool.name());

    // Process the initial buffer if we read it for detection
    if args.detect && !buffer.is_empty() {
        for output in tool.parse_and_format(buffer) {
            writeln!(writer, "{output}")?;
        }
    }

    // Stream remaining input
    loop {
        buffer.clear();
        buffer.resize(CHUNK_SIZE, 0);
        let n = reader.read(buffer)?;

        if n == 0 {
            break;
        }

        buffer.truncate(n);

        for output in tool.parse_and_format(buffer) {
            writeln!(writer, "{output}")?;
        }
    }

    Ok(())
}
