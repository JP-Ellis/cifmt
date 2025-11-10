//! Tool-specific message format catalog.
//!
//! This module contains submodules for different tools that can output
//! structured messages (typically JSON). Each submodule defines the message
//! formats for that tool and implements conversion to CI messages.

pub mod cargo_test;

// Placeholder for future tool support
// pub mod mypy;
// pub mod pytest;
// pub mod eslint;
// pub mod clippy;

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
    /// The tool name.
    const TOOL_NAME: &'static str;

    type Message: crate::message::CiMessage;
    type Error: std::error::Error;

    /// Infer the tool from a sample of its output.
    ///
    /// Returns `Some(Self)` if the sample matches this tool, otherwise returns
    /// `None`.
    ///
    /// Implementations may use heuristics such as checking for specific JSON
    /// keys, patterns, or other identifiable markers in the sample.
    fn detect(&self, sample: &[u8]) -> Option<Self>
    where
        Self: Sized;

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
