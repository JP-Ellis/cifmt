//! Cargo test JSON output format (libtest).
//!
//! Support for parsing and formatting messages from `cargo test --format json`.
//!
//! The libtest JSON format is documented in the Rust standard library:
//! <https://github.com/rust-lang/rust/blob/master/library/test/src/formatters/json.rs>

use crate::ci::GitHub;
use crate::message::CiMessage;
use serde::{Deserialize, Serialize};

/// A message from libtest's JSON formatter.
///
/// These messages are emitted when running `cargo test -- --format json -Z
/// unstable-options` on nightly Rust.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl CiMessage for LibTestMessage {
    type Platform = GitHub;

    fn format(&self) -> String {
        match self {
            Self::Test(test_msg) => test_msg.format(),
            Self::Suite(suite_msg) => suite_msg.format(),
            Self::Bench(bench_msg) => bench_msg.format(),
            Self::Report(report_msg) => report_msg.format(),
        }
    }
}

/// Suite-level events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
#[non_exhaustive]
pub enum SuiteMessage {
    /// Test discovery started.
    Discovery,

    /// Test discovery completed.
    Completed {
        /// Number of tests discovered.
        tests: usize,
        /// Number of benchmarks discovered.
        benchmarks: usize,
        /// Total tests and benchmarks.
        total: usize,
        /// Number of ignored tests.
        ignored: usize,
    },

    /// Test suite started.
    Started {
        /// Number of tests to run.
        test_count: usize,
        /// Optional shuffle seed.
        #[serde(skip_serializing_if = "Option::is_none")]
        shuffle_seed: Option<u64>,
    },

    /// Test suite passed.
    Ok {
        /// Number of tests passed.
        passed: usize,
        /// Number of tests failed.
        failed: usize,
        /// Number of tests ignored.
        ignored: usize,
        /// Number of benchmarks measured.
        measured: usize,
        /// Number of tests filtered out.
        filtered_out: usize,
        /// Optional execution time in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        exec_time: Option<f64>,
    },

    /// Test suite failed.
    Failed {
        /// Number of tests passed.
        passed: usize,
        /// Number of tests failed.
        failed: usize,
        /// Number of tests ignored.
        ignored: usize,
        /// Number of benchmarks measured.
        measured: usize,
        /// Number of tests filtered out.
        filtered_out: usize,
        /// Optional execution time in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        exec_time: Option<f64>,
    },
}

impl CiMessage for SuiteMessage {
    type Platform = GitHub;

    fn format(&self) -> String {
        match self {
            &Self::Discovery => GitHub::group("Test Discovery"),

            Self::Completed {
                tests,
                benchmarks,
                total,
                ignored,
            } => {
                let mut parts = Vec::new();

                parts.push(GitHub::endgroup());
                parts.push(GitHub::notice(&format!(
                    "Discovered {total} items: {tests} tests, {benchmarks} benchmarks, {ignored} ignored"
                ))
                .title("Test Discovery")
                .format());

                parts.join("")
            }

            &Self::Started {
                test_count,
                shuffle_seed: _,
            } => {
                // We don't start a group here because the individual tests will
                // create their own groups.
                GitHub::notice(&format!("Running {test_count} tests"))
                    .title("Test Suite Started")
                    .format()
            }

            Self::Failed {
                passed,
                failed,
                ignored,
                measured,
                filtered_out,
                exec_time,
            } => {
                let time_info = exec_time
                    .map(|t| format!(" in {:.2}s", t))
                    .unwrap_or_default();
                GitHub::error(&format!(
                        "{failed} failed, {passed} passed, {ignored} ignored, {measured} measured, {filtered_out} filtered out{time_info}"
                    ))
                    .title("Test Suite Failed")
                    .format()
            }

            Self::Ok {
                passed,
                failed,
                ignored,
                measured,
                filtered_out,
                exec_time,
            } => {
                let time_info = exec_time
                    .map(|t| format!(" in {:.2}s", t))
                    .unwrap_or_default();
                GitHub::notice(&format!(
                        "{passed} passed, {failed} failed, {ignored} ignored, {measured} measured, {filtered_out} filtered out{time_info}"
                    ))
                    .title("Test Suite Passed")
                    .format()
            }
        }
    }
}

/// Individual test events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl CiMessage for TestMessage {
    type Platform = GitHub;

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

                if let Some(stdout) = stdout.as_ref().filter(|s| s.is_empty()) {
                    parts.push(stdout.clone() + "\n");
                }

                parts.push(
                    GitHub::notice(
                        &exec_time
                            .map(|t| format!("Executed in {:.2}s", t))
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

                if let Some(stdout) = stdout.as_ref().filter(|s| s.is_empty()) {
                    parts.push(stdout.clone() + "\n");
                }

                parts.push(GitHub::endgroup());

                let time_info = exec_time
                    .map(|t| format!(" (executed in {:.2}s)", t))
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
                    .filter(|s| s.is_empty())
                    .map(|s| s.replace('\n', " "))
                    .unwrap_or_default(),
            )
            .title(&format!("Test Ignored: {name}"))
            .format(),
        }
    }
}

/// Benchmark result message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchMessage {
    /// Benchmark name.
    pub name: String,
    /// Median time in nanoseconds.
    pub median: u64,
    /// Deviation (max - min) in nanoseconds.
    pub deviation: u64,
    /// Optional throughput in MiB/s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mib_per_second: Option<u64>,
}

impl CiMessage for BenchMessage {
    type Platform = GitHub;

    fn format(&self) -> String {
        let throughput = self
            .mib_per_second
            .map(|mb| format!(" ({} MiB/s)", mb))
            .unwrap_or_default();
        GitHub::notice(&format!(
            "{}: {} ns/iter (± {}){}",
            self.name, self.median, self.deviation, throughput
        ))
        .title("Benchmark Result")
        .format()
    }
}

/// Doctest timing report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportMessage {
    /// Total execution time in seconds.
    pub total_time: f64,
    /// Compilation time in seconds.
    pub compilation_time: f64,
}

impl CiMessage for ReportMessage {
    type Platform = GitHub;

    fn format(&self) -> String {
        GitHub::notice(&format!(
            "Total: {:.2}s, Compilation: {:.2}s",
            self.total_time, self.compilation_time
        ))
        .title("Doctest Report")
        .format()
    }
}

/// Tool implementation for parsing cargo test (libtest) JSON output.
#[derive(Debug, Clone, Default)]
pub struct CargoTest {
    /// Buffer for incomplete JSON lines.
    buffer: Vec<u8>,
}

impl CargoTest {
    /// Creates a new CargoTest parser.
    pub fn new() -> Self {
        Self::default()
    }
}

impl crate::tool::Tool for CargoTest {
    const TOOL_NAME: &'static str = "cargo-test";

    type Message = LibTestMessage;
    type Error = serde_json::Error;

    fn detect(&self, sample: &[u8]) -> Option<Self> {
        // Try to parse a few lines and check if they match libtest JSON format
        let sample_str = std::str::from_utf8(sample).ok()?;

        for line in sample_str.lines().take(5) {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(line) {
                // Check for libtest-specific message types
                if let Some(msg_type) = value.get("type").and_then(|v| v.as_str()) {
                    match msg_type {
                        "suite" | "test" | "bench" | "report" => {
                            return Some(Self::new());
                        }
                        _ => continue,
                    }
                }
            }
        }

        None
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
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::message::CiMessage;
    use crate::tool::cargo_test::{LibTestMessage, SuiteMessage};

    macro_rules! set_snapshot_suffix {
        ($($expr:expr),*) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_suffix(format!($($expr,)*));
            let _guard = settings.bind_to_scope();
        }
    }

    #[rstest]
    #[case::suite_discovery(
        r#"{"type":"suite","event":"discovery"}"#,
        LibTestMessage::Suite(SuiteMessage::Discovery)
    )]
    #[case::suite_completed(
        r#"{"type":"suite","event":"completed","tests":42,"benchmarks":5,"total":47,"ignored":3}"#,
        LibTestMessage::Suite(SuiteMessage::Completed {
            tests: 42,
            benchmarks: 5,
            total: 47,
            ignored: 3,
        })
    )]
    #[case::suite_started(
        r#"{"type":"suite","event":"started","test_count":42}"#,
        LibTestMessage::Suite(SuiteMessage::Started {
            test_count: 42,
            shuffle_seed: None,
        })
    )]
    #[case::suite_ok(
        r#"{"type":"suite","event":"ok","passed":40,"failed":0,"ignored":2,"measured":0,"filtered_out":5,"exec_time":1.234}"#,
        LibTestMessage::Suite(SuiteMessage::Ok {
            passed: 40,
            failed: 0,
            ignored: 2,
            measured: 0,
            filtered_out: 5,
            exec_time: Some(1.234),
        })
    )]
    #[case::suite_failed(
        r#"{"type":"suite","event":"failed","passed":38,"failed":2,"ignored":2,"measured":0,"filtered_out":5,"exec_time":1.567}"#,
        LibTestMessage::Suite(SuiteMessage::Failed {
            passed: 38,
            failed: 2,
            ignored: 2,
            measured: 0,
            filtered_out: 5,
            exec_time: Some(1.567),
        })
    )]
    #[case::test_discovered(
        r#"{"type":"test","event":"discovered","name":"test_example","ignore":false,"source_path":"src/lib.rs","start_line":10,"start_col":4,"end_line":15,"end_col":5}"#,
        LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Discovered {
            name: "test_example".to_string(),
            ignore: false,
            ignore_message: None,
            source_path: "src/lib.rs".to_string(),
            start_line: 10,
            start_col: 4,
            end_line: 15,
            end_col: 5,
        })
    )]
    #[case::test_started(
        r#"{"type":"test","event":"started","name":"test_example"}"#,
        LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Started {
            name: "test_example".to_string(),
        })
    )]
    #[case::test_ok(
        r#"{"type":"test","event":"ok","name":"test_example","exec_time":0.001}"#,
        LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Ok {
            name: "test_example".to_string(),
            exec_time: Some(0.001),
            stdout: None,
        })
    )]
    #[case::test_failed(
        r#"{"type":"test","event":"failed","name":"test_failing","exec_time":0.003,"message":"assertion failed"}"#,
        LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Failed {
            name: "test_failing".to_string(),
            exec_time: Some(0.003),
            stdout: None,
            message: Some("assertion failed".to_string()),
        })
    )]
    #[case::test_timeout(
        r#"{"type":"test","event":"timeout","name":"test_hanging"}"#,
        LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Timeout {
            name: "test_hanging".to_string(),
        })
    )]
    #[case::test_ignored(
        r#"{"type":"test","event":"ignored","name":"test_ignored"}"#,
        LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Ignored {
            name: "test_ignored".to_string(),
            message: None,
        })
    )]
    #[case::bench(
        r#"{"type":"bench","name":"bench_example","median":1234,"deviation":56}"#,
        LibTestMessage::Bench(crate::tool::cargo_test::BenchMessage {
            name: "bench_example".to_string(),
            median: 1234,
            deviation: 56,
            mib_per_second: None,
        })
    )]
    #[case::report(
        r#"{"type":"report","total_time":10.5,"compilation_time":8.2}"#,
        LibTestMessage::Report(crate::tool::cargo_test::ReportMessage {
            total_time: 10.5,
            compilation_time: 8.2,
        })
    )]
    fn deserialize(#[case] json: &str, #[case] expected: LibTestMessage) {
        let msg: LibTestMessage = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(msg, expected);
    }

    macro_rules! platform_format {
        ($fn:ident, $platform:expr) => {
            #[rstest]
            #[case(LibTestMessage::Suite(SuiteMessage::Discovery), "suite_discovery")]
            #[case(LibTestMessage::Suite(SuiteMessage::Completed {
                    tests: 42,
                    benchmarks: 5,
                    total: 47,
                    ignored: 3,
                }),
                "suite_completed"
            )]
            #[case(LibTestMessage::Suite(SuiteMessage::Started {
                    test_count: 42,
                    shuffle_seed: None,
                }),
                "suite_started"
            )]
            #[case(LibTestMessage::Suite(SuiteMessage::Ok {
                    passed: 40,
                    failed: 0,
                    ignored: 2,
                    measured: 0,
                    filtered_out: 5,
                    exec_time: Some(1.234),
                }),
                "suite_ok"
            )]
            #[case(LibTestMessage::Suite(SuiteMessage::Failed {
                    passed: 38,
                    failed: 2,
                    ignored: 2,
                    measured: 0,
                    filtered_out: 5,
                    exec_time: Some(1.567),
                }),
                "suite_failed"
            )]
            #[case(LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Discovered {
                    name: "test_example".to_string(),
                    ignore: false,
                    ignore_message: None,
                    source_path: "src/lib.rs".to_string(),
                    start_line: 10,
                    start_col: 4,
                    end_line: 15,
                    end_col: 5,
                }),
                "test_discovered"
            )]
            #[case(
                            LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Started {
                                name: "test_example".to_string(),
                            }),
                            "test_started"
                        )]
            #[case(
                            LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Ok {
                                name: "test_example".to_string(),
                                exec_time: Some(0.001),
                                stdout: None,
                            }),
                            "test_ok"
                        )]
            #[case(
                            LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Failed {
                                name: "test_failing".to_string(),
                                exec_time: Some(0.003),
                                stdout: None,
                                message: Some("assertion failed".to_string()),
                            }),
                            "test_failed"
                        )]
            #[case(
                            LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Timeout {
                                name: "test_hanging".to_string(),
                            }),
                            "test_timeout"
                        )]
            #[case(
                            LibTestMessage::Test(crate::tool::cargo_test::TestMessage::Ignored {
                                name: "test_ignored".to_string(),
                                message: None,
                            }),
                            "test_ignored"
                        )]
            #[case(
                            LibTestMessage::Bench(crate::tool::cargo_test::BenchMessage {
                                name: "bench_example".to_string(),
                                median: 1234,
                                deviation: 56,
                                mib_per_second: None,
                            }),
                            "bench"
                        )]
            #[case(
                            LibTestMessage::Report(crate::tool::cargo_test::ReportMessage {
                                total_time: 10.5,
                                compilation_time: 8.2,
                            }),
                            "report"
                        )]
            fn $fn(#[case] message: LibTestMessage, #[case] suffix: &str) {
                set_snapshot_suffix!("{}-{suffix}", $platform);
                let formatted = message.format();
                insta::assert_snapshot!(formatted);
            }
        };
    }

    platform_format!(format_github, crate::ci::GitHub);
}
