//! Core message types and traits.
//!
//! This module defines the fundamental types and traits for working with
//! messages from different tools and formatting them for CI platforms.

use crate::ci::Platform;

/// Trait for types that can be formatted as CI messages.
///
/// This trait allows different message types to be formatted for specific
/// CI platforms.
///
/// # Example
///
/// ```rust
/// use cifmt::{prelude::*, ci::GitHub};
///
/// struct MyMessage {
///     text: String,
///     file: String,
///     line: u32
/// }
///
/// impl CiMessage<GitHub> for MyMessage {
///     fn format(&self) -> String {
///        Self::Platform::notice(&self.text)
///            .file(&self.file)
///            .line(self.line)
///            .format()
///     }
/// }
/// ```
pub trait CiMessage<T: Platform> {
    /// Formats this message for output.
    ///
    /// # Returns
    ///
    /// A formatted string suitable for the associated CI platform.
    fn format(&self) -> String;
}
