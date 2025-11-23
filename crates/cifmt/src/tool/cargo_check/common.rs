//! Common types shared across cargo messages.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Cargo target information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    /// Array of target kinds.
    pub kind: Vec<String>,
    /// Array of crate types.
    pub crate_types: Vec<String>,
    /// Target name.
    pub name: String,
    /// Absolute path to the root source file.
    pub src_path: PathBuf,
    /// Rust edition.
    pub edition: String,
    /// Required features.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
    /// Whether the target should be documented.
    pub doc: bool,
    /// Whether the target has doc tests enabled.
    pub doctest: bool,
    /// Whether the target should be built and run with --test.
    pub test: bool,
}

/// Build profile information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    /// Optimization level.
    pub opt_level: String,
    /// Debug level (can be null, integer, or string).
    pub debuginfo: Option<serde_json::Value>,
    /// Whether debug assertions are enabled.
    pub debug_assertions: bool,
    /// Whether overflow checks are enabled.
    pub overflow_checks: bool,
    /// Whether the --test flag is used.
    pub test: bool,
}
