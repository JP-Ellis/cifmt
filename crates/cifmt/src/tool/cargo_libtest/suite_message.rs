//! Test suite-level events from cargo test.

use crate::ci::{GitHub, Plain};
use crate::message::CiMessage;
use serde::Deserialize;

/// Suite-level events.
#[derive(Debug, Clone, PartialEq, Deserialize)]
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

impl CiMessage<Plain> for SuiteMessage {
    fn format(&self) -> String {
        match self {
            &Self::Discovery => "SUITE: Test Discovery Started".to_string(),

            Self::Completed {
                tests,
                benchmarks,
                total,
                ignored,
            } => format!(
                "SUITE: Test Discovery Completed - Discovered {} items: {} tests, {} benchmarks, {} ignored",
                total, tests, benchmarks, ignored
            ),

            &Self::Started {
                test_count,
                shuffle_seed: _,
            } => format!("SUITE: Test Suite Started - Running {} tests", test_count),

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
                format!(
                    "SUITE: Test Suite Failed - {} failed, {} passed, {} ignored, {} measured, {} filtered out{}",
                    failed, passed, ignored, measured, filtered_out, time_info
                )
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
                format!(
                    "SUITE: Test Suite Passed - {} passed, {} failed, {} ignored, {} measured, {} filtered out{}",
                    passed, failed, ignored, measured, filtered_out, time_info
                )
            }
        }
    }
}

impl CiMessage<GitHub> for SuiteMessage {
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

#[cfg(test)]
pub mod test_data {
    use super::SuiteMessage;
    use serde_json::json;

    /// Test data for suite messages: (JSON value, expected message, description)
    pub fn suite_cases() -> impl Iterator<Item = (&'static str, serde_json::Value, SuiteMessage)> {
        [
            (
                "suite_discovery",
                json!({
                    "type": "suite",
                    "event": "discovery",
                }),
                SuiteMessage::Discovery,
            ),
            (
                "suite_completed",
                json!({
                    "type": "suite",
                    "event": "completed",
                    "tests": 42,
                    "benchmarks": 5,
                    "total": 47,
                    "ignored": 3,
                }),
                SuiteMessage::Completed {
                    tests: 42,
                    benchmarks: 5,
                    total: 47,
                    ignored: 3,
                },
            ),
            (
                "suite_started",
                json!({
                    "type": "suite",
                    "event": "started",
                    "test_count": 42,
                }),
                SuiteMessage::Started {
                    test_count: 42,
                    shuffle_seed: None,
                },
            ),
            (
                "suite_ok",
                json!({
                    "type": "suite",
                    "event": "ok",
                    "passed": 40,
                    "failed": 0,
                    "ignored": 2,
                    "measured": 0,
                    "filtered_out": 5,
                    "exec_time": 1.234,
                }),
                SuiteMessage::Ok {
                    passed: 40,
                    failed: 0,
                    ignored: 2,
                    measured: 0,
                    filtered_out: 5,
                    exec_time: Some(1.234),
                },
            ),
            (
                "suite_failed",
                json!({
                    "type": "suite",
                    "event": "failed",
                    "passed": 38,
                    "failed": 2,
                    "ignored": 2,
                    "measured": 0,
                    "filtered_out": 5,
                    "exec_time": 1.567,
                }),
                SuiteMessage::Failed {
                    passed: 38,
                    failed: 2,
                    ignored: 2,
                    measured: 0,
                    filtered_out: 5,
                    exec_time: Some(1.567),
                },
            ),
        ]
        .into_iter()
    }
}
