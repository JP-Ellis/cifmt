//! Build script execution results from Cargo's JSON messages.
//!
//! This module defines the `BuildScriptExecuted` struct which represents the
//! `"build-script-executed"` message emitted by Cargo when a build script
//! (`build.rs`) runs. The type captures common metadata produced by build
//! scripts (linked libraries, search paths, cfg flags, environment
//! variables and output directories) and provides formatting implementations
//! for both plain text and CI-specific renderers.
use serde::Deserialize;

use crate::{
    ci::{GitHub, Plain},
    ci_message::CiMessage,
};

/// Build script execution result.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BuildScriptExecuted {
    /// The Package ID.
    pub package_id: String,
    /// Libraries to link.
    pub linked_libs: Vec<String>,
    /// Library search paths.
    pub linked_paths: Vec<String>,
    /// Cfg values to enable.
    pub cfgs: Vec<String>,
    /// Environment variables to set.
    pub env: Vec<(String, String)>,
    /// Output directory path.
    pub out_dir: String,
}

impl CiMessage<Plain> for BuildScriptExecuted {
    fn format(&self) -> String {
        format!("Build script executed: {}", self.package_id)
    }
}

impl CiMessage<GitHub> for BuildScriptExecuted {
    fn format(&self) -> String {
        GitHub::debug(format!("Build script executed: {}", self.package_id))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::BuildScriptExecuted;
    use serde_json::json;

    /// Test data for build script executed messages.
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, BuildScriptExecuted)> {
        [
            (
                "build_script_executed".to_owned(),
                json!({
                    "reason": "build-script-executed",
                    "package_id": "mypackage 0.1.0 (path+file:///path/to/package)",
                    "linked_libs": ["ssl", "crypto"],
                    "linked_paths": ["/usr/lib", "/usr/local/lib"],
                    "cfgs": ["feature=\"my_feature\""],
                    "env": [["CARGO_FEATURE_MY_FEATURE", "1"]],
                    "out_dir": "/path/to/target/debug/build/mypackage-abc123/out",
                }),
                BuildScriptExecuted {
                    package_id: "mypackage 0.1.0 (path+file:///path/to/package)".to_owned(),
                    linked_libs: vec!["ssl".to_owned(), "crypto".to_owned()],
                    linked_paths: vec!["/usr/lib".to_owned(), "/usr/local/lib".to_owned()],
                    cfgs: vec!["feature=\"my_feature\"".to_owned()],
                    env: vec![("CARGO_FEATURE_MY_FEATURE".to_owned(), "1".to_owned())],
                    out_dir: "/path/to/target/debug/build/mypackage-abc123/out".to_owned(),
                },
            ),
            (
                "build_script_executed_minimal".to_owned(),
                json!({
                    "reason": "build-script-executed",
                    "package_id": "simple 1.0.0",
                    "linked_libs": [],
                    "linked_paths": [],
                    "cfgs": [],
                    "env": [],
                    "out_dir": "/tmp/out",
                }),
                BuildScriptExecuted {
                    package_id: "simple 1.0.0".to_owned(),
                    linked_libs: vec![],
                    linked_paths: vec![],
                    cfgs: vec![],
                    env: vec![],
                    out_dir: "/tmp/out".to_owned(),
                },
            ),
        ]
        .into_iter()
    }
}
