//! Cargo test JSON output format (libtest).
//!
//! Support for parsing and formatting messages from `cargo test --format json`.
//!
//! The libtest JSON format is documented in the Rust standard library:
//! <https://github.com/rust-lang/rust/blob/master/library/test/src/formatters/json.rs>

mod bench_message;
mod report_message;
mod suite_message;
mod test_message;

use std::io::BufRead;

use crate::{
    ci::{GitHub, Plain},
    ci_message::CiMessage,
    tool::{
        Detect, Tool,
        cargo_libtest::{
            bench_message::BenchMessage, report_message::ReportMessage,
            suite_message::SuiteMessage, test_message::TestMessage,
        },
    },
};
use serde::Deserialize;

/// A message from libtest's JSON formatter.
///
/// These messages are emitted when running `cargo test -- --format json -Z
/// unstable-options` on nightly Rust.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum LibTestMessage {
    /// Test suite event (started, completed, ok, failed).
    Suite(SuiteMessage),

    /// Individual test event (started, ok, failed, ignored, timeout).
    Test(TestMessage),

    /// Benchmark result.
    Bench(BenchMessage),

    /// Doctest timing report.
    Report(ReportMessage),
}

impl CiMessage<Plain> for LibTestMessage {
    fn format(&self) -> String {
        match self {
            Self::Test(test_msg) => <TestMessage as CiMessage<Plain>>::format(test_msg),
            Self::Suite(suite_msg) => <SuiteMessage as CiMessage<Plain>>::format(suite_msg),
            Self::Bench(bench_msg) => <BenchMessage as CiMessage<Plain>>::format(bench_msg),
            Self::Report(report_msg) => <ReportMessage as CiMessage<Plain>>::format(report_msg),
        }
    }
}

impl CiMessage<GitHub> for LibTestMessage {
    fn format(&self) -> String {
        match self {
            Self::Test(test_msg) => <TestMessage as CiMessage<GitHub>>::format(test_msg),
            Self::Suite(suite_msg) => <SuiteMessage as CiMessage<GitHub>>::format(suite_msg),
            Self::Bench(bench_msg) => <BenchMessage as CiMessage<GitHub>>::format(bench_msg),
            Self::Report(report_msg) => <ReportMessage as CiMessage<GitHub>>::format(report_msg),
        }
    }
}

/// Tool implementation for parsing cargo test (libtest) JSON output.
#[derive(Debug, Clone, Default)]
pub struct CargoLibtest {
    /// Buffer for incomplete JSON lines.
    buffer: Vec<u8>,
}

impl Detect for CargoLibtest {
    type Tool = Self;

    #[inline]
    fn detect(sample: &[u8]) -> Option<Self::Tool> {
        let (oks, errs) = sample
            .lines()
            .map_while(Result::ok)
            .map(|line| serde_json::from_str::<LibTestMessage>(&line))
            .fold((0_u8, 0_u8), |(oks, errs), res| match res {
                Ok(_) => (oks.saturating_add(1), errs),
                Err(_) => (oks, errs.saturating_add(1)),
            });

        (oks > errs).then(Self::default)
    }
}

impl Tool for CargoLibtest {
    type Message = LibTestMessage;
    type Error = serde_json::Error;

    #[inline]
    fn name(&self) -> &'static str {
        "cargo-libtest"
    }

    #[inline]
    fn parse(&mut self, buf: &[u8]) -> Vec<Result<Self::Message, Self::Error>> {
        let mut results = Vec::new();

        // Append new data to buffer
        self.buffer.extend_from_slice(buf);

        // Process complete lines
        while let Some(newline_pos) = self.buffer.iter().position(|&b| b == b'\n') {
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
            match serde_json::from_slice::<LibTestMessage>(line) {
                Ok(msg) => results.push(Ok(msg)),
                Err(e) => {
                    // Only report error if it looks like JSON (starts with '{')
                    if line.first() == Some(&b'{') {
                        results.push(Err(e));
                    }
                    // Otherwise skip non-JSON lines (like rust output)
                }
            }
        }

        results
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use pretty_assertions::assert_eq;

    use crate::ci_message::CiMessage;
    use crate::{
        ci::{GitHub, Plain},
        tool::cargo_libtest::LibTestMessage,
    };

    macro_rules! set_snapshot_suffix {
        ($($expr:expr),*) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_suffix(format!($($expr,)*));
            let _guard = settings.bind_to_scope();
        }
    }

    fn cases() -> impl Iterator<Item = (String, serde_json::Value, LibTestMessage)> {
        super::suite_message::tests::cases()
            .map(|(desc, json, msg)| (desc, json, LibTestMessage::Suite(msg)))
            .chain(
                super::test_message::tests::cases()
                    .map(|(desc, json, msg)| (desc, json, LibTestMessage::Test(msg))),
            )
            .chain(
                super::bench_message::tests::cases()
                    .map(|(desc, json, msg)| (desc, json, LibTestMessage::Bench(msg))),
            )
            .chain(
                super::report_message::tests::cases()
                    .map(|(desc, json, msg)| (desc, json, LibTestMessage::Report(msg))),
            )
    }

    #[test]
    fn deserialize_all() {
        for (_, json_value, expected) in cases() {
            let msg: LibTestMessage =
                serde_json::from_value(json_value).expect("Failed to deserialize");
            assert_eq!(msg, expected);
        }
    }

    #[test]
    fn format_plain() {
        for (desc, _, message) in cases() {
            set_snapshot_suffix!("{desc}");
            let formatted = <LibTestMessage as CiMessage<Plain>>::format(&message);
            insta::assert_snapshot!(formatted);
        }
    }

    #[test]
    fn format_github() {
        for (desc, _, message) in cases() {
            set_snapshot_suffix!("{desc}");
            let formatted = <LibTestMessage as CiMessage<GitHub>>::format(&message);
            insta::assert_snapshot!(formatted);
        }
    }
}
