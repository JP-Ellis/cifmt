//! Build finished messages for Cargo's JSON output.
//!
//! This module defines the `BuildFinished` type which represents the
//! `"build-finished"` JSON message emitted by Cargo when a build completes.
use serde::Deserialize;

use crate::{
    ci::{GitHub, Plain},
    ci_message::CiMessage,
};

/// Build finished message.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BuildFinished {
    /// Whether the build succeeded.
    pub success: bool,
}

impl CiMessage<Plain> for BuildFinished {
    fn format(&self) -> String {
        if self.success {
            "Build finished successfully".to_owned()
        } else {
            "Build failed".to_owned()
        }
    }
}

impl CiMessage<GitHub> for BuildFinished {
    fn format(&self) -> String {
        if self.success {
            GitHub::notice("Build finished successfully")
                .title("Build Complete")
                .format()
        } else {
            GitHub::error("Build failed").title("Build Failed").format()
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::BuildFinished;
    use serde_json::json;

    /// Test data for build finished messages.
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, BuildFinished)> {
        [
            (
                "build_finished_success".to_owned(),
                json!({
                    "reason": "build-finished",
                    "success": true,
                }),
                BuildFinished { success: true },
            ),
            (
                "build_finished_failure".to_owned(),
                json!({
                    "reason": "build-finished",
                    "success": false,
                }),
                BuildFinished { success: false },
            ),
        ]
        .into_iter()
    }
}
