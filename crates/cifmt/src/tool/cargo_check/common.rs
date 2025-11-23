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

#[cfg(test)]
pub(crate) mod tests {
    use super::{Profile, Target};
    use serde_json::json;
    use std::path::PathBuf;

    /// Test data for Target structs.
    pub fn target_cases() -> impl Iterator<Item = (String, serde_json::Value, Target)> {
        [
            (
                "target_lib".to_owned(),
                json!({
                    "kind": ["lib"],
                    "crate_types": ["lib"],
                    "name": "mylib",
                    "src_path": "/path/to/src/lib.rs",
                    "edition": "2021",
                    "doc": true,
                    "doctest": true,
                    "test": true,
                }),
                Target {
                    kind: vec!["lib".to_owned()],
                    crate_types: vec!["lib".to_owned()],
                    name: "mylib".to_owned(),
                    src_path: PathBuf::from("/path/to/src/lib.rs"),
                    edition: "2021".to_owned(),
                    required_features: vec![],
                    doc: true,
                    doctest: true,
                    test: true,
                },
            ),
            (
                "target_bin".to_owned(),
                json!({
                    "kind": ["bin"],
                    "crate_types": ["bin"],
                    "name": "myapp",
                    "src_path": "/path/to/src/main.rs",
                    "edition": "2021",
                    "required_features": ["feature1", "feature2"],
                    "doc": true,
                    "doctest": false,
                    "test": false,
                }),
                Target {
                    kind: vec!["bin".to_owned()],
                    crate_types: vec!["bin".to_owned()],
                    name: "myapp".to_owned(),
                    src_path: PathBuf::from("/path/to/src/main.rs"),
                    edition: "2021".to_owned(),
                    required_features: vec!["feature1".to_owned(), "feature2".to_owned()],
                    doc: true,
                    doctest: false,
                    test: false,
                },
            ),
        ]
        .into_iter()
    }

    /// Test data for Profile structs.
    pub fn profile_cases() -> impl Iterator<Item = (String, serde_json::Value, Profile)> {
        [
            (
                "profile_debug".to_owned(),
                json!({
                    "opt_level": "0",
                    "debuginfo": 2,
                    "debug_assertions": true,
                    "overflow_checks": true,
                    "test": false,
                }),
                Profile {
                    opt_level: "0".to_owned(),
                    debuginfo: Some(json!(2)),
                    debug_assertions: true,
                    overflow_checks: true,
                    test: false,
                },
            ),
            (
                "profile_release".to_owned(),
                json!({
                    "opt_level": "3",
                    "debuginfo": null,
                    "debug_assertions": false,
                    "overflow_checks": false,
                    "test": false,
                }),
                Profile {
                    opt_level: "3".to_owned(),
                    debuginfo: None,
                    debug_assertions: false,
                    overflow_checks: false,
                    test: false,
                },
            ),
        ]
        .into_iter()
    }
}
