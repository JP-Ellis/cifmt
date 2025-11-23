//! Rustc JSON output messages.

use crate::ci::GitHub;
use crate::message::CiMessage;
use serde::{Deserialize, Serialize};

use super::diagnostic::Diagnostic;

/// A message from rustc's JSON output.
///
/// Rustc can emit various types of messages when running with JSON output.
/// This enum represents all possible message types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "$message_type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum RustcMessage {
    /// Compiler diagnostic (error, warning, etc.).
    Diagnostic(Diagnostic),

    /// Artifact notification.
    Artifact(Artifact),

    /// Future incompatibility report.
    FutureIncompat(FutureIncompat),

    /// Unused extern crate dependencies.
    UnusedExterns(UnusedExterns),

    /// Compilation section timing information.
    SectionTiming(SectionTiming),
}

impl CiMessage for RustcMessage {
    type Platform = GitHub;

    fn format(&self) -> String {
        match self {
            Self::Diagnostic(msg) => msg.format(),
            Self::Artifact(msg) => msg.format(),
            Self::FutureIncompat(msg) => msg.format(),
            Self::UnusedExterns(msg) => msg.format(),
            Self::SectionTiming(msg) => msg.format(),
        }
    }
}

/// Artifact notification emitted when a file artifact has been saved to disk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    /// The filename that was generated.
    pub artifact: String,
    /// The kind of artifact that was generated.
    pub emit: EmitKind,
}

impl CiMessage for Artifact {
    type Platform = GitHub;

    fn format(&self) -> String {
        GitHub::debug(format!(
            "Generated artifact: {} ({})",
            self.artifact, self.emit
        ))
    }
}

/// The kind of artifact that was generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Future incompatibility report for warnings that will become errors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FutureIncompat {
    /// Array of future incompatibility warnings.
    pub future_incompat_report: Vec<FutureIncompatEntry>,
}

impl CiMessage for FutureIncompat {
    type Platform = GitHub;

    fn format(&self) -> String {
        let mut result = String::new();

        if !self.future_incompat_report.is_empty() {
            result.push_str(
                &GitHub::warning("Future incompatibility warnings detected")
                    .title("Future Incompatibility Report")
                    .format(),
            );

            for entry in &self.future_incompat_report {
                result.push_str(&entry.diagnostic.format());
            }
        }

        result
    }
}

/// A single entry in the future incompatibility report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FutureIncompatEntry {
    /// The diagnostic information.
    pub diagnostic: Diagnostic,
}

/// Unused extern crate dependencies report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnusedExterns {
    /// Level of the lint (warn, deny, forbid).
    pub lint_level: String,
    /// Names of unused crates.
    pub unused_names: Vec<String>,
}

impl CiMessage for UnusedExterns {
    type Platform = GitHub;

    fn format(&self) -> String {
        if self.unused_names.is_empty() {
            return String::new();
        }

        let message = format!("Unused dependencies: {}", self.unused_names.join(", "));

        match self.lint_level.as_str() {
            "deny" | "forbid" => GitHub::error(&message)
                .title("Unused Dependencies")
                .format(),
            _ => GitHub::warning(&message)
                .title("Unused Dependencies")
                .format(),
        }
    }
}

/// Compilation section timing information (unstable).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SectionTiming {
    /// Event type ("start" or "end").
    pub event: TimingEvent,
    /// Name of the compilation section.
    pub name: String,
    /// Timestamp in microseconds (relative to compilation start).
    pub time: u64,
}

impl CiMessage for SectionTiming {
    type Platform = GitHub;

    fn format(&self) -> String {
        GitHub::debug(format!(
            "Compilation section {} {}: {} ({}μs)",
            self.name, self.event, self.name, self.time
        ))
    }
}

/// Timing event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimingEvent {
    /// Start of a compilation section.
    Start,
    /// End of a compilation section.
    End,
}

impl std::fmt::Display for TimingEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
        }
    }
}

#[cfg(test)]
pub mod test_data {
    use super::{Artifact, EmitKind, RustcMessage, SectionTiming, TimingEvent, UnusedExterns};
    use crate::tool::cargo_check::diagnostic::test_data as diagnostic_data;
    use serde_json::json;

    /// Test data for rustc message enums.
    pub fn rustc_message_cases()
    -> impl Iterator<Item = (&'static str, serde_json::Value, RustcMessage)> {
        // Get the first diagnostic for the RustcMessage::Diagnostic variant
        let (_diag_desc, diag_json, diag_msg) = diagnostic_data::diagnostic_cases().next().unwrap();

        [
            (
                "rustc_diagnostic",
                diag_json,
                RustcMessage::Diagnostic(diag_msg),
            ),
            (
                "rustc_artifact",
                json!({
                    "$message_type": "artifact",
                    "artifact": "/path/to/target/debug/example.rlib",
                    "emit": "metadata",
                }),
                RustcMessage::Artifact(Artifact {
                    artifact: "/path/to/target/debug/example.rlib".to_string(),
                    emit: EmitKind::Metadata,
                }),
            ),
            (
                "rustc_unused_externs",
                json!({
                    "$message_type": "unused_externs",
                    "lint_level": "warn",
                    "unused_names": ["serde", "tokio"],
                }),
                RustcMessage::UnusedExterns(UnusedExterns {
                    lint_level: "warn".to_string(),
                    unused_names: vec!["serde".to_string(), "tokio".to_string()],
                }),
            ),
            (
                "rustc_section_timing",
                json!({
                    "$message_type": "section_timing",
                    "event": "start",
                    "name": "codegen",
                    "time": 1234567,
                }),
                RustcMessage::SectionTiming(SectionTiming {
                    event: TimingEvent::Start,
                    name: "codegen".to_string(),
                    time: 1234567,
                }),
            ),
        ]
        .into_iter()
    }

    /// Test data for artifact messages.
    pub fn artifact_cases() -> impl Iterator<Item = (&'static str, serde_json::Value, Artifact)> {
        [
            (
                "artifact_link",
                json!({
                    "artifact": "/path/to/target/debug/example",
                    "emit": "link",
                }),
                Artifact {
                    artifact: "/path/to/target/debug/example".to_string(),
                    emit: EmitKind::Link,
                },
            ),
            (
                "artifact_metadata",
                json!({
                    "artifact": "/path/to/target/debug/libexample.rmeta",
                    "emit": "metadata",
                }),
                Artifact {
                    artifact: "/path/to/target/debug/libexample.rmeta".to_string(),
                    emit: EmitKind::Metadata,
                },
            ),
        ]
        .into_iter()
    }

    /// Test data for unused externs messages.
    pub fn unused_externs_cases()
    -> impl Iterator<Item = (&'static str, serde_json::Value, UnusedExterns)> {
        [
            (
                "unused_externs_warn",
                json!({
                    "lint_level": "warn",
                    "unused_names": ["serde"],
                }),
                UnusedExterns {
                    lint_level: "warn".to_string(),
                    unused_names: vec!["serde".to_string()],
                },
            ),
            (
                "unused_externs_deny",
                json!({
                    "lint_level": "deny",
                    "unused_names": ["tokio", "reqwest"],
                }),
                UnusedExterns {
                    lint_level: "deny".to_string(),
                    unused_names: vec!["tokio".to_string(), "reqwest".to_string()],
                },
            ),
        ]
        .into_iter()
    }

    /// Test data for section timing messages.
    pub fn section_timing_cases()
    -> impl Iterator<Item = (&'static str, serde_json::Value, SectionTiming)> {
        [
            (
                "timing_start",
                json!({
                    "event": "start",
                    "name": "codegen",
                    "time": 1000000,
                }),
                SectionTiming {
                    event: TimingEvent::Start,
                    name: "codegen".to_string(),
                    time: 1000000,
                },
            ),
            (
                "timing_end",
                json!({
                    "event": "end",
                    "name": "codegen",
                    "time": 1500000,
                }),
                SectionTiming {
                    event: TimingEvent::End,
                    name: "codegen".to_string(),
                    time: 1500000,
                },
            ),
        ]
        .into_iter()
    }
}
