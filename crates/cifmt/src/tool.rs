//! Tool-specific message format catalog.
//!
//! This module contains submodules for different tools that can output
//! structured messages (typically JSON). Each submodule defines the message
//! formats for that tool and implements conversion to CI messages.

use thiserror::Error;

pub mod cargo_libtest;

/// Trait for tool detection.
///
/// This trait defines a method for detecting if a given buffer of input
/// corresponds to the tool's output format. If the tool is detected, it
/// returns an instance of the tool.
pub trait ToolDetect {
    /// The tool type associated with this detection.
    ///
    /// In most cases, this will be the implementor type itself.
    type Tool: Tool;

    /// Detect if the given buffer corresponds to this tool's output format.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A sample of the input data.
    ///
    /// # Returns
    ///
    /// Returns `Some(tool_instance)` if detection succeeds, otherwise `None`.
    fn detect(buffer: &[u8]) -> Option<Self::Tool>
    where
        Self: Sized;
}

/// Trait for tool.
///
/// We assume that each tool has a well-defined set of message formats that can
/// be parsed and converted into CI messages. In many instances, this set may
/// be a single message format, but support for multiple formats can be added
/// by implementing this into the `Message` associated type.
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
    type Message: crate::message::CiMessage;
    type Error: std::error::Error;

    /// Get the tool name as a string.
    fn name(&self) -> &'static str;

    /// Parse messages from the tool's output.
    ///
    /// The parser is expected to read the entire buffer and extract all
    /// possible messages, returning them as a vector of results.
    ///
    /// If the buffer ends with incomplete data, the parser must be able to
    /// store any necessary state to continue parsing when more data is
    /// provided in subsequent calls.
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

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("No tool format detected")]
    NoToolDetected,
}

pub fn detect<M, E>(buffer: &[u8]) -> Result<cargo_libtest::CargoLibtest, ToolError> {
    if let Some(tool) = cargo_libtest::CargoLibtest::detect(buffer) {
        tracing::info!("Detected tool format: {}", tool.name());
        return Ok(tool);
    }

    Err(ToolError::NoToolDetected)
}
