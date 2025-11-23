//! Rustc JSON output messages.

mod artifact;
mod diagnostic;
mod future_incompat;
mod section_timing;
mod unused_externs;

use crate::{
    ci::{GitHub, Plain},
    tool::cargo_check::compiler_message::rustc_message::{
        artifact::Artifact, diagnostic::Diagnostic, section_timing::SectionTiming,
        unused_externs::UnusedExterns,
    },
};
use crate::{
    ci_message::CiMessage,
    tool::cargo_check::compiler_message::rustc_message::future_incompat::FutureIncompat,
};
use serde::Deserialize;

/// A message from rustc's JSON output.
///
/// Rustc can emit various types of messages when running with JSON output.
/// This enum represents all possible message types.
#[derive(Debug, Clone, PartialEq, Deserialize)]
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

impl CiMessage<Plain> for RustcMessage {
    fn format(&self) -> String {
        match self {
            Self::Diagnostic(msg) => CiMessage::<Plain>::format(msg),
            Self::Artifact(msg) => CiMessage::<Plain>::format(msg),
            Self::FutureIncompat(msg) => CiMessage::<Plain>::format(msg),
            Self::UnusedExterns(msg) => CiMessage::<Plain>::format(msg),
            Self::SectionTiming(msg) => CiMessage::<Plain>::format(msg),
        }
    }
}

impl CiMessage<GitHub> for RustcMessage {
    fn format(&self) -> String {
        match self {
            Self::Diagnostic(msg) => CiMessage::<GitHub>::format(msg),
            Self::Artifact(msg) => CiMessage::<GitHub>::format(msg),
            Self::FutureIncompat(msg) => CiMessage::<GitHub>::format(msg),
            Self::UnusedExterns(msg) => CiMessage::<GitHub>::format(msg),
            Self::SectionTiming(msg) => CiMessage::<GitHub>::format(msg),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::tool::cargo_check::compiler_message::rustc_message::diagnostic;

    use super::RustcMessage;

    /// Test data combining all rustc message types.
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, RustcMessage)> {
        diagnostic::tests::cases()
            .map(|(desc, json, msg)| (desc, json, RustcMessage::Diagnostic(msg)))
            .chain(
                super::artifact::tests::cases()
                    .map(|(desc, json, msg)| (desc, json, RustcMessage::Artifact(msg))),
            )
            .chain(
                super::future_incompat::tests::cases()
                    .map(|(desc, json, msg)| (desc, json, RustcMessage::FutureIncompat(msg))),
            )
            .chain(
                super::unused_externs::tests::cases()
                    .map(|(desc, json, msg)| (desc, json, RustcMessage::UnusedExterns(msg))),
            )
            .chain(
                super::section_timing::tests::cases()
                    .map(|(desc, json, msg)| (desc, json, RustcMessage::SectionTiming(msg))),
            )
    }

    #[test]
    fn deserialize_all() {
        use pretty_assertions::assert_eq;

        for (_, json_value, expected) in cases() {
            let msg: RustcMessage =
                serde_json::from_value(json_value).expect("Failed to deserialize");
            assert_eq!(msg, expected);
        }
    }
}
