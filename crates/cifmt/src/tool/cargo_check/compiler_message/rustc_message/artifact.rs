//! Rustc artifact notification details.
//!
//! This module contains the `Artifact` type and the `EmitKind` enum which
//! represent the `artifact` messages emitted by `rustc` when it saves
//! compilation artifacts to disk. These items are primarily used by the
//! `CompilerArtifact` wrapper to provide richer output about built files and
//! their kinds (linkable crates, bitcode, LLVM IR, object files, etc.).
use serde::Deserialize;

use crate::{
    ci::{GitHub, Plain},
    ci_message::CiMessage,
};

/// Artifact notification emitted when a file artifact has been saved to disk.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Artifact {
    /// The filename that was generated.
    pub artifact: String,
    /// The kind of artifact that was generated.
    pub emit: EmitKind,
}

impl CiMessage<Plain> for Artifact {
    fn format(&self) -> String {
        format!("Generated artifact: {} ({})", self.artifact, self.emit)
    }
}

impl CiMessage<GitHub> for Artifact {
    fn format(&self) -> String {
        GitHub::debug(format!(
            "Generated artifact: {} ({})",
            self.artifact, self.emit
        ))
    }
}

/// The kind of artifact that was generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmitKind {
    /// The generated crate as specified by the crate-type.
    Link,
    /// The `.d` file with dependency information in a Makefile-like syntax.
    DepInfo,
    /// The Rust `.rmeta` file containing metadata about the crate.
    Metadata,
    /// The `.s` file with generated assembly.
    Asm,
    /// The `.ll` file with generated textual LLVM IR.
    LlvmIr,
    /// The `.bc` file with generated LLVM bitcode.
    LlvmBc,
    /// The `.mir` file with rustc's mid-level intermediate representation.
    Mir,
    /// The `.o` file with generated native object code.
    Obj,
}

impl std::fmt::Display for EmitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Link => write!(f, "link"),
            Self::DepInfo => write!(f, "dep-info"),
            Self::Metadata => write!(f, "metadata"),
            Self::Asm => write!(f, "asm"),
            Self::LlvmIr => write!(f, "llvm-ir"),
            Self::LlvmBc => write!(f, "llvm-bc"),
            Self::Mir => write!(f, "mir"),
            Self::Obj => write!(f, "obj"),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::{Artifact, EmitKind};
    use serde_json::json;

    /// Test data for artifact messages.
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, Artifact)> {
        [
            (
                "artifact_link".to_owned(),
                json!({
                    "$message_type": "artifact",
                    "artifact": "target/debug/myapp",
                    "emit": "link",
                }),
                Artifact {
                    artifact: "target/debug/myapp".to_owned(),
                    emit: EmitKind::Link,
                },
            ),
            (
                "artifact_metadata".to_owned(),
                json!({
                    "$message_type": "artifact",
                    "artifact": "target/debug/deps/libmylib.rmeta",
                    "emit": "metadata",
                }),
                Artifact {
                    artifact: "target/debug/deps/libmylib.rmeta".to_owned(),
                    emit: EmitKind::Metadata,
                },
            ),
            (
                "artifact_dep_info".to_owned(),
                json!({
                    "$message_type": "artifact",
                    "artifact": "target/debug/myapp.d",
                    "emit": "dep-info",
                }),
                Artifact {
                    artifact: "target/debug/myapp.d".to_owned(),
                    emit: EmitKind::DepInfo,
                },
            ),
        ]
        .into_iter()
    }
}
