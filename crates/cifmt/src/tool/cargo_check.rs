//! Cargo JSON output format.
//!
//! Support for parsing and formatting messages from `cargo check
//! --message-format json`, `cargo clippy --message-format json`, and certain
//! other cargo commands that emit JSON messages.
//!
//! The JSON message format is documented in the Cargo book:
//! <https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages>
//!
//! It is also closely linked to the `rustc` JSON diagnostic format:
//! <https://doc.rust-lang.org/rustc/json.html>

mod cargo_message;
mod common;
mod diagnostic;
mod rustc_message;

use std::io::BufRead;

pub use cargo_message::{
    BuildFinished, BuildScriptExecuted, CargoMessage, CompilerArtifact, CompilerMessage,
};
pub use common::{Profile, Target};
pub use diagnostic::{
    Diagnostic, DiagnosticCode, DiagnosticLevel, DiagnosticSpan, DiagnosticSpanLine,
    DiagnosticSpanMacroExpansion, SuggestionApplicability,
};
pub use rustc_message::{
    Artifact, EmitKind, FutureIncompat, FutureIncompatEntry, RustcMessage, SectionTiming,
    TimingEvent, UnusedExterns,
};

/// Tool implementation for parsing cargo JSON output.
#[derive(Debug, Clone, Default)]
pub struct CargoCheck {
    /// Buffer for incomplete JSON lines.
    buffer: Vec<u8>,
}

impl Tool for CargoCheck {
    type Message = CargoMessage;
    type Error = serde_json::Error;

    fn name(&self) -> &'static str {
        "cargo-check"
    }

    fn detect(sample: &[u8]) -> bool {
        let (oks, errs) = sample
            .lines()
            .map_while(Result::ok)
            .map(|line| serde_json::from_str::<Self::Message>(&line))
            .fold((0_u8, 0_u8), |(oks, errs), res| match res {
                Ok(_) => (oks + 1, errs),
                Err(_) => (oks, errs + 1),
            });

        oks > errs
    }

    fn parse(&mut self, buf: &[u8]) -> Vec<Result<Self::Message, Self::Error>> {
        let mut results = Vec::new();

        // Append new data to buffer
        self.buffer.extend_from_slice(buf);

        // Process complete lines
        while let Some(newline_pos) = self.buffer.iter().position(|&b| b == b'\n') {
            // Extract line (excluding newline)
            let line = self.buffer.drain(..=newline_pos).collect::<Vec<u8>>();
            let line = &line[..line.len() - 1]; // Remove newline

            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            // Try to parse as JSON
            match serde_json::from_slice::<CargoMessage>(line) {
                Ok(msg) => results.push(Ok(msg)),
                Err(e) => {
                    // Only report error if it looks like JSON (starts with '{')
                    if line.first() == Some(&b'{') {
                        results.push(Err(e));
                    }
                    // Otherwise skip non-JSON lines (like plain text output)
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::message::CiMessage;
    use crate::tool::cargo_check::CargoMessage;

    macro_rules! set_snapshot_suffix {
        ($($expr:expr),*) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_suffix(format!($($expr,)*));
            let _guard = settings.bind_to_scope();
        }
    }

    fn all_cases() -> impl Iterator<Item = (&'static str, serde_json::Value, CargoMessage)> {
        super::cargo_message::test_data::build_finished_cases()
            .map(|(desc, json, msg)| (desc, json, CargoMessage::BuildFinished(msg)))
            .chain(
                super::cargo_message::test_data::build_script_cases()
                    .map(|(desc, json, msg)| (desc, json, CargoMessage::BuildScriptExecuted(msg))),
            )
            .chain(
                super::cargo_message::test_data::compiler_artifact_cases()
                    .map(|(desc, json, msg)| (desc, json, CargoMessage::CompilerArtifact(msg))),
            )
            .chain(
                super::cargo_message::test_data::compiler_message_cases()
                    .map(|(desc, json, msg)| (desc, json, CargoMessage::CompilerMessage(msg))),
            )
    }

    #[test]
    fn deserialize_all() {
        for (_, json_value, expected) in all_cases() {
            let msg: CargoMessage =
                serde_json::from_value(json_value).expect("Failed to deserialize");
            assert_eq!(msg, expected);
        }
    }

    #[test]
    fn format_github() {
        for (desc, _, message) in all_cases() {
            set_snapshot_suffix!("{}-{desc}", crate::ci::GitHub);
            let formatted = message.format();
            insta::assert_snapshot!(formatted);
        }
    }
}
