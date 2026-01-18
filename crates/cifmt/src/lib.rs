//! CI message formatter library.
//!
//! `cifmt` provides types and utilities for parsing structured output from
//! various development tools (like test runners, linters, etc.) and formatting
//! them for different CI platforms (GitHub Actions, GitLab CI, etc.), with a
//! focus on rich annotations, groupings, severity levels, file locations, etc.
//!
//! # Overview
//!
//! The library is organized around three main concepts:
//!
//! 1. **CI Platforms** ([`ci`]): Contains platform-specific implementations
//!    that implement the [`ci::Platform`] trait for formatting messages
//!    according to the conventions of each CI system.
//!
//! 2. **Tool Formats** ([`tool`]): Parsers for specific tool output formats
//!    (cargo test, nextest, mypy, etc.).
//!
//! 3. **Messages** ([`CiMessage`]): A trait for types that can be formatted as
//!    CI messages.
//!

pub mod ci;
pub mod ci_message;
pub mod tool;

pub mod prelude {
    //! A prelude module for convenient imports.
    //!
    //! This module provides convenient re-exports of commonly used types and
    //! traits from the `cifmt` crate, allowing for easier and more concise
    //! imports.
    #![expect(clippy::pub_use, reason = "convenience re-exports")]
    pub use crate::ci::Platform;
    pub use crate::ci_message::CiMessage;
    pub use crate::tool::{Detect, DynTool, Tool};
}
