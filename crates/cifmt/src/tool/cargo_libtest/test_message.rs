//! Individual test events from cargo test.

use crate::ci::{GitHub, Plain};
use crate::ci_message::CiMessage;
use serde::Deserialize;

/// Individual test events.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
#[non_exhaustive]
pub enum TestMessage {
    /// Test discovered during listing.
    Discovered {
        /// Test name.
        name: String,
        /// Whether the test is ignored.
        ignore: bool,
        /// Optional ignore message.
        #[serde(skip_serializing_if = "Option::is_none")]
        ignore_message: Option<String>,
        /// Source file path.
        source_path: String,
        /// Starting line number.
        start_line: usize,
        /// Starting column number.
        start_col: usize,
        /// Ending line number.
        end_line: usize,
        /// Ending column number.
        end_col: usize,
    },

    /// Test started.
    Started {
        /// Test name.
        name: String,
    },

    /// Test passed.
    Ok {
        /// Test name.
        name: String,
        /// Optional execution time in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        exec_time: Option<f64>,
        /// Optional stdout output.
        #[serde(skip_serializing_if = "Option::is_none")]
        stdout: Option<String>,
    },

    /// Test failed.
    Failed {
        /// Test name.
        name: String,
        /// Optional execution time in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        exec_time: Option<f64>,
        /// Optional stdout output.
        #[serde(skip_serializing_if = "Option::is_none")]
        stdout: Option<String>,
        /// Optional failure message.
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },

    /// Test timed out.
    Timeout {
        /// Test name.
        name: String,
    },

    /// Test ignored.
    Ignored {
        /// Test name.
        name: String,
        /// Optional ignore message.
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
}

impl CiMessage<Plain> for TestMessage {
    fn format(&self) -> String {
        match self {
            Self::Discovered {
                name,
                ignore,
                ignore_message,
                source_path,
                start_line,
                start_col,
                end_line,
                end_col,
            } => format!(
                "TEST DISCOVERED: {name} (ignored: {ignore}, message: {ignore_message:?}, location: {source_path}:{start_line}:{start_col}-{end_line}:{end_col})"
            ),

            Self::Started { name } => format!("TEST STARTED: {name}"),

            Self::Ok {
                name,
                exec_time,
                stdout,
            } => {
                let mut parts = Vec::with_capacity(2);

                if let Some(v) = stdout.as_ref().filter(|s| !s.is_empty()) {
                    parts.push(v.clone());
                }

                parts.push(format!(
                    "TEST OK: {}{}",
                    name,
                    exec_time
                        .map(|t| format!(" (executed in {t:.2}s)"))
                        .unwrap_or_default()
                ));

                parts.join("\n")
            }

            Self::Failed {
                name,
                message,
                stdout,
                exec_time,
            } => {
                let mut parts = Vec::with_capacity(2);

                if let Some(v) = stdout.as_ref().filter(|s| !s.is_empty()) {
                    parts.push(v.clone());
                }

                parts.push(format!(
                    "TEST FAILED: {}{}{}\n",
                    name,
                    exec_time
                        .map(|t| format!(" (executed in {t:.2}s)"))
                        .unwrap_or_default(),
                    message
                        .as_ref()
                        .map(|m| format!(" - {m}"))
                        .unwrap_or_default()
                ));

                parts.join("\n")
            }

            Self::Timeout { name } => format!("TEST TIMEOUT: {name}"),

            Self::Ignored { name, message } => format!(
                "TEST IGNORED: {}{}",
                name,
                message
                    .as_ref()
                    .filter(|s| !s.is_empty())
                    .map(|s| format!(" - {}", s.replace('\n', " ")))
                    .unwrap_or_default()
            ),
        }
    }
}

impl CiMessage<GitHub> for TestMessage {
    fn format(&self) -> String {
        match self {
            Self::Discovered {
                name,
                ignore,
                ignore_message,
                source_path,
                start_line,
                start_col,
                end_line,
                end_col,
            } => GitHub::debug(format!(
                "Discovered test: {name} (ignored: {ignore}, message: {ignore_message:?}, location: {source_path}:{start_line}:{start_col}-{end_line}:{end_col})",
            )),

            Self::Started { name } => GitHub::group(format!("Test: {name}")),

            Self::Ok {
                name,
                exec_time,
                stdout,
            } => {
                let mut parts = Vec::with_capacity(3);

                if let Some(v) = stdout.as_ref().filter(|s| !s.is_empty()) {
                    parts.push(v.clone() + "\n");
                }

                parts.push(
                    GitHub::notice(
                        &exec_time
                            .map(|t| format!("Executed in {t:.2}s"))
                            .unwrap_or_default(),
                    )
                    .title(&format!("Test Passed: {name}"))
                    .format(),
                );

                parts.push(GitHub::endgroup());

                parts.join("")
            }

            Self::Failed {
                name,
                message,
                stdout,
                exec_time,
            } => {
                let mut parts = Vec::with_capacity(3);

                if let Some(v) = stdout.as_ref().filter(|s| !s.is_empty()) {
                    parts.push(v.clone() + "\n");
                }

                parts.push(GitHub::endgroup());

                let time_info = exec_time
                    .map(|t| format!(" (executed in {t:.2}s)"))
                    .unwrap_or_default();

                parts.push(
                    GitHub::notice(message.as_deref().unwrap_or_default())
                        .title(&format!("Test Failed: {name}{time_info}"))
                        .format(),
                );

                parts.join("")
            }

            Self::Timeout { name } => [
                GitHub::endgroup(),
                GitHub::error(name).title("Test Timeout").format(),
            ]
            .join(""),

            Self::Ignored { name, message } => GitHub::notice(
                &message
                    .as_deref()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.replace('\n', " "))
                    .unwrap_or_default(),
            )
            .title(&format!("Test Ignored: {name}"))
            .format(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::TestMessage;
    use serde_json::json;

    /// Test data for test messages: (JSON value, message instance, description)
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, TestMessage)> {
        [
            (
                "test_discovered".to_owned(),
                json!({
                    "type": "test",
                    "event": "discovered",
                    "name": "test_example",
                    "ignore": false,
                    "source_path": "src/lib.rs",
                    "start_line": 10,
                    "start_col": 4,
                    "end_line": 15,
                    "end_col": 5,
                }),
                TestMessage::Discovered {
                    name: "test_example".to_owned(),
                    ignore: false,
                    ignore_message: None,
                    source_path: "src/lib.rs".to_owned(),
                    start_line: 10,
                    start_col: 4,
                    end_line: 15,
                    end_col: 5,
                },
            ),
            (
                "test_started".to_owned(),
                json!({
                    "type": "test",
                    "event": "started",
                    "name": "test_example",
                }),
                TestMessage::Started {
                    name: "test_example".to_owned(),
                },
            ),
            (
                "test_ok".to_owned(),
                json!({
                    "type":"test",
                    "event":"ok",
                    "name":"test_example",
                    "exec_time":0.001,
                }),
                TestMessage::Ok {
                    name: "test_example".to_owned(),
                    exec_time: Some(0.001),
                    stdout: None,
                },
            ),
            (
                "test_failed".to_owned(),
                json!({
                    "type":"test",
                    "event":"failed",
                    "name":"test_failing",
                    "exec_time":0.003,
                    "message":"assertion failed",
                }),
                TestMessage::Failed {
                    name: "test_failing".to_owned(),
                    exec_time: Some(0.003),
                    stdout: None,
                    message: Some("assertion failed".to_owned()),
                },
            ),
            (
                "test_timeout".to_owned(),
                json!({
                    "type":"test",
                    "event":"timeout",
                    "name":"test_hanging",
                }),
                TestMessage::Timeout {
                    name: "test_hanging".to_owned(),
                },
            ),
            (
                "test_ignored".to_owned(),
                json!({
                    "type":"test",
                    "event":"ignored",
                    "name":"test_ignored",
                }),
                TestMessage::Ignored {
                    name: "test_ignored".to_owned(),
                    message: None,
                },
            ),
        ]
        .into_iter()
    }
}
