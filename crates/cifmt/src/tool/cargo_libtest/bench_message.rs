//! Benchmark result messages from cargo test.

use crate::ci::{GitHub, Plain};
use crate::ci_message::CiMessage;
use serde::Deserialize;

/// Benchmark result message.
#[derive(Debug, Clone, PartialEq, Deserialize)]
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

impl CiMessage<Plain> for BenchMessage {
    fn format(&self) -> String {
        let throughput = self
            .mib_per_second
            .map(|mb| format!(" ({mb} MiB/s)"))
            .unwrap_or_default();
        format!(
            "BENCH: {}: {} ns/iter (± {}){}",
            self.name, self.median, self.deviation, throughput
        )
    }
}

impl CiMessage<GitHub> for BenchMessage {
    fn format(&self) -> String {
        let throughput = self
            .mib_per_second
            .map(|mb| format!(" ({mb} MiB/s)"))
            .unwrap_or_default();
        GitHub::notice(&format!(
            "{}: {} ns/iter (± {}){}",
            self.name, self.median, self.deviation, throughput
        ))
        .title("Benchmark Result")
        .format()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::BenchMessage;
    use serde_json::json;

    /// Test data for bench messages: (JSON value, message instance, description)
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, BenchMessage)> {
        [(
            "bench".to_owned(),
            json!({
                "type": "bench",
                "name": "bench_example",
                "median": 1234,
                "deviation": 56,
            }),
            BenchMessage {
                name: "bench_example".to_owned(),
                median: 1234,
                deviation: 56,
                mib_per_second: None,
            },
        )]
        .into_iter()
    }
}
