//! Compiler message wrapper and helpers.
//!
//! This module defines the `CompilerMessage` structure which represents
//! messages emitted by the Rust compiler during a `cargo check` run. These
//! messages include errors, warnings, and informational notes.
//!
//! The `CompilerMessage` wraps the underlying `RustcMessage` along with
//! additional metadata about the package and target that generated the message.
mod rustc_message;

use crate::{
    ci::{GitHub, Plain},
    ci_message::CiMessage,
    tool::cargo_check::{common::Target, compiler_message::rustc_message::RustcMessage},
};
use serde::Deserialize;

/// Compiler message (errors, warnings, notes).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CompilerMessage {
    /// The Package ID.
    pub package_id: String,
    /// Absolute path to the package manifest.
    pub manifest_path: String,
    /// The Cargo target that generated the message.
    pub target: Target,
    /// The rustc message from the compiler.
    pub message: RustcMessage,
}

impl CiMessage<Plain> for CompilerMessage {
    fn format(&self) -> String {
        <RustcMessage as CiMessage<Plain>>::format(&self.message)
    }
}

impl CiMessage<GitHub> for CompilerMessage {
    fn format(&self) -> String {
        <RustcMessage as CiMessage<GitHub>>::format(&self.message)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::CompilerMessage;
    use crate::tool::cargo_check::common;
    use serde_json::json;

    /// Test data for compiler messages.
    ///
    /// This generates all combinations of targets and rustc messages for
    /// comprehensive test coverage.
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, CompilerMessage)> {
        common::tests::target_cases().flat_map(move |(target_desc, target_json, target)| {
            super::rustc_message::tests::cases().map(move |(rustc_desc, rustc_json, rustc_msg)| {
                (
                    format!("compiler_message_{target_desc}_{rustc_desc}"),
                    json!({
                        "reason": "compiler-message",
                        "package_id": "mypackage 0.1.0 (path+file:///path/to/package)",
                        "manifest_path": "/path/to/package/Cargo.toml",
                        "target": target_json.clone(),
                        "message": rustc_json.clone(),
                    }),
                    CompilerMessage {
                        package_id: "mypackage 0.1.0 (path+file:///path/to/package)".to_owned(),
                        manifest_path: "/path/to/package/Cargo.toml".to_owned(),
                        target: target.clone(),
                        message: rustc_msg.clone(),
                    },
                )
            })
        })
    }

    #[test]
    fn deserialize_all() {
        use pretty_assertions::assert_eq;

        for (_, json_value, expected) in cases() {
            let msg: CompilerMessage =
                serde_json::from_value(json_value).expect("Failed to deserialize");
            assert_eq!(msg, expected);
        }
    }
}
