//! Tool-specific message format catalog.
//!
//! This module contains submodules for different tools that can output
//! structured messages (typically JSON). Each submodule defines the message
//! formats for that tool and implements conversion to CI messages.

#![expect(clippy::pub_use, reason = "convenience re-exports of tool types")]

use crate::ci::Platform;

mod cargo_check;
mod cargo_libtest;

pub use cargo_check::CargoCheck;
pub use cargo_libtest::CargoLibtest;

/// Trait for types that can detect a tool format from sample output.
pub trait Detect {
    /// The concrete tool type returned when this detector succeeds.
    type Tool: Tool;

    /// Detect if the given sample matches this tool's format.
    ///
    /// # Arguments
    ///
    /// * `sample` - A byte slice containing a sample of the tool's output. This
    ///   should typically contain a few lines of output to allow for detection.
    ///
    /// # Returns
    ///
    /// The associated tool if detected, otherwise `None`.
    fn detect(sample: &[u8]) -> Option<Self::Tool>;
}

/// Trait for tool.
///
/// We assume that each tool has a well-defined set of message formats that can
/// be parsed and converted into CI messages. In many instances, this set may be
/// a single message format, but support for multiple formats can be added by
/// implementing this into the `Message` associated type.
///
/// This trait defines capabilities for detecting, reading, and parsing messages
/// from a specific tool.
pub trait Tool {
    /// The message type produced by this tool.
    ///
    /// If the tool supports multiple message formats, this can be an enum
    /// encapsulating all supported formats; otherwise, it can be a single
    /// message type.
    ///
    /// It must implement the `CiMessage` trait to allow conversion to CI
    /// messages.
    type Message;
    /// Error type returned by the parser for this tool.
    type Error: std::error::Error;

    /// Get the tool name as a string.
    fn name(&self) -> &'static str;

    /// Parse messages from the tool's output.
    ///
    /// The parser is expected to read the entire buffer and extract all
    /// possible messages, returning them as a vector of results.
    ///
    /// If the buffer ends with incomplete data, the parser must be able to
    /// store any necessary state to continue parsing when more data is provided
    /// in subsequent calls.
    ///
    /// You can assume that successive calls to `parse` will provide contiguous
    /// data from the tool's output.
    ///
    /// # Arguments
    ///
    /// * `buf` - A buffer containing the tool's output.
    ///
    /// # Returns
    ///
    /// A vector of results, each being either a successfully parsed message or
    /// an error if parsing failed for that message.
    fn parse(&mut self, buf: &[u8]) -> Vec<Result<Self::Message, Self::Error>>;
}

/// Dynamic tool wrapper that combines parsing and formatting.
///
/// This trait is object-safe and allows working with tools polymorphically at
/// runtime while preserving platform-specific formatting.
#[expect(
    clippy::module_name_repetitions,
    reason = "DynTool is a clear name for a trait that provides dynamic dispatch for tools"
)]
pub trait DynTool<P: Platform> {
    /// Get the tool name.
    fn name(&self) -> &'static str;

    /// Parse and format messages from the tool's output.
    ///
    /// Returns formatted strings ready for output to the specified platform.
    fn parse_and_format(&mut self, buf: &[u8]) -> Vec<String>;
}

/// Errors that can occur during tool detection.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// No tool format detected in the provided buffer.
    #[error("No tool format detected")]
    NoToolDetected,
}

/// Detect which tool format is present in the buffer.
///
/// # Arguments
///
/// * `sample` - A byte slice containing a sample of the tool's output. This
///   should typically contain a few lines of output to allow for detection.
///
/// # Returns
///
/// Returns the detected tool if successful, otherwise returns an error.
///
/// # Errors
///
/// Returns `ToolError::NoToolDetected` if no known tool format is detected.
#[inline]
pub fn detect<P: Platform + 'static>(buffer: &[u8]) -> Result<Box<dyn DynTool<P>>, Error>
where
    cargo_check::CargoCheck: DynTool<P>,
    cargo_libtest::CargoLibtest: DynTool<P>,
{
    if let Some(tool) = cargo_check::CargoCheck::detect(buffer) {
        tracing::info!("Detected tool format: {}", Tool::name(&tool));
        return Ok(Box::new(tool));
    }

    if let Some(tool) = cargo_libtest::CargoLibtest::detect(buffer) {
        tracing::info!("Detected tool format: {}", Tool::name(&tool));
        return Ok(Box::new(tool));
    }

    Err(Error::NoToolDetected)
}
