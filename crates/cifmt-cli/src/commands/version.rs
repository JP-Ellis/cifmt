use crate::{commands::OutputFormat, version::Version};

#[derive(Debug, clap::Args)]
pub(crate) struct Args {
    /// Output format
    #[arg(long, value_enum, default_value = "text")]
    output_format: OutputFormat,
}

/// Execute the version command.
pub(crate) fn execute(Args { output_format }: Args) -> Result<(), Box<dyn std::error::Error>> {
    let version = Version::default();

    match output_format {
        crate::commands::OutputFormat::Text => {
            println!("{}", version);
        }
        crate::commands::OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&version)?;
            println!("{}", json);
        }
    }

    Ok(())
}
