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

mod build_finished;
mod build_script_executed;
mod common;
mod compiler_artifact;
mod compiler_message;

use crate::{
    ci::{GitHub, Plain},
    ci_message::CiMessage,
    tool::{
        Detect, Tool,
        cargo_check::{
            build_finished::BuildFinished, build_script_executed::BuildScriptExecuted,
            compiler_artifact::CompilerArtifact, compiler_message::CompilerMessage,
        },
    },
};
use serde::Deserialize;
use std::io::BufRead;

/// A message from cargo's JSON output.
///
/// These messages are emitted when running cargo commands with
/// `--message-format=json`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum CargoMessage {
    /// Compiler diagnostic (error, warning, etc.).
    CompilerMessage(CompilerMessage),

    /// Artifact produced by the build.
    CompilerArtifact(CompilerArtifact),

    /// Build script output.
    BuildScriptExecuted(BuildScriptExecuted),

    /// Build finished.
    BuildFinished(BuildFinished),
}

impl CiMessage<Plain> for CargoMessage {
    fn format(&self) -> String {
        match self {
            Self::CompilerMessage(msg) => <CompilerMessage as CiMessage<Plain>>::format(msg),
            Self::CompilerArtifact(msg) => <CompilerArtifact as CiMessage<Plain>>::format(msg),
            Self::BuildScriptExecuted(msg) => {
                <BuildScriptExecuted as CiMessage<Plain>>::format(msg)
            }
            Self::BuildFinished(msg) => <BuildFinished as CiMessage<Plain>>::format(msg),
        }
    }
}

impl CiMessage<GitHub> for CargoMessage {
    fn format(&self) -> String {
        match self {
            Self::CompilerMessage(msg) => <CompilerMessage as CiMessage<GitHub>>::format(msg),
            Self::CompilerArtifact(msg) => <CompilerArtifact as CiMessage<GitHub>>::format(msg),
            Self::BuildScriptExecuted(msg) => {
                <BuildScriptExecuted as CiMessage<GitHub>>::format(msg)
            }
            Self::BuildFinished(msg) => <BuildFinished as CiMessage<GitHub>>::format(msg),
        }
    }
}

/// Tool implementation for parsing cargo JSON output.
#[derive(Debug, Clone, Default)]
pub struct CargoCheck {
    /// Buffer for incomplete JSON lines.
    buffer: Vec<u8>,
}

impl Detect for CargoCheck {
    type Tool = Self;
    #[inline]
    fn detect(sample: &[u8]) -> Option<Self::Tool> {
        let (oks, errs) = sample
            .lines()
            .map_while(Result::ok)
            .map(|line| serde_json::from_str::<CargoMessage>(&line))
            .fold((0_u8, 0_u8), |(oks, errs), res| match res {
                Ok(_) => (oks.saturating_add(1), errs),
                Err(_) => (oks, errs.saturating_add(1)),
            });

        (oks > errs).then(CargoCheck::default)
    }
}

impl Tool for CargoCheck {
    type Message = CargoMessage;
    type Error = serde_json::Error;

    #[inline]
    fn name(&self) -> &'static str {
        "cargo-check"
    }

    #[inline]
    fn parse(&mut self, buf: &[u8]) -> Vec<Result<Self::Message, Self::Error>> {
        let mut results = Vec::new();

        // Append new data to buffer
        self.buffer.extend_from_slice(buf);

        // Process complete lines
        while let Some(newline_pos) = self.buffer.iter().position(|&b| b == b'\n') {
            // Extract line bytes (including newline)
            let mut line_bytes = self.buffer.drain(..=newline_pos).collect::<Vec<u8>>();
            if line_bytes.last() == Some(&b'\n') {
                line_bytes.pop();
            }
            let line = line_bytes.as_slice();

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
pub(crate) mod tests {
    use super::CargoMessage;
    use crate::{
        ci::{GitHub, Plain},
        ci_message::CiMessage,
    };
    use pretty_assertions::assert_eq;

    macro_rules! set_snapshot_suffix {
        ($($expr:expr),*) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_suffix(format!($($expr,)*));
            let _guard = settings.bind_to_scope();
        }
    }

    fn cases() -> impl Iterator<Item = (String, serde_json::Value, CargoMessage)> {
        super::compiler_message::tests::cases()
            .map(|(desc, json, msg)| (desc, json, CargoMessage::CompilerMessage(msg)))
            .chain(
                super::compiler_artifact::tests::cases()
                    .map(|(desc, json, msg)| (desc, json, CargoMessage::CompilerArtifact(msg))),
            )
            .chain(
                super::build_script_executed::tests::cases()
                    .map(|(desc, json, msg)| (desc, json, CargoMessage::BuildScriptExecuted(msg))),
            )
            .chain(
                super::build_finished::tests::cases()
                    .map(|(desc, json, msg)| (desc, json, CargoMessage::BuildFinished(msg))),
            )
    }

    #[test]
    fn deserialize_all() {
        for (_, json_value, expected) in cases() {
            let msg: CargoMessage =
                serde_json::from_value(json_value).expect("Failed to deserialize");
            assert_eq!(msg, expected);
        }
    }

    #[test]
    fn format_plain() {
        for (desc, _, message) in cases() {
            set_snapshot_suffix!("{desc}");
            let formatted = <CargoMessage as CiMessage<Plain>>::format(&message);
            insta::assert_snapshot!(formatted);
        }
    }

    #[test]
    fn format_github() {
        for (desc, _, message) in cases() {
            set_snapshot_suffix!("{desc}");
            let formatted = <CargoMessage as CiMessage<GitHub>>::format(&message);
            insta::assert_snapshot!(formatted);
        }
    }
}
