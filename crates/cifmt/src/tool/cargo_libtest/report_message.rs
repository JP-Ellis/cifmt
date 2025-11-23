//! Doctest timing report messages from cargo test.

use crate::ci::{GitHub, Plain};
use crate::ci_message::CiMessage;
use serde::Deserialize;

/// Doctest timing report.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ReportMessage {
    /// Total execution time in seconds.
    pub total_time: f64,
    /// Compilation time in seconds.
    pub compilation_time: f64,
}

impl CiMessage<Plain> for ReportMessage {
    fn format(&self) -> String {
        format!(
            "REPORT: Total: {:.2}s, Compilation: {:.2}s",
            self.total_time, self.compilation_time
        )
    }
}

impl CiMessage<GitHub> for ReportMessage {
    fn format(&self) -> String {
        GitHub::notice(&format!(
            "Total: {:.2}s, Compilation: {:.2}s",
            self.total_time, self.compilation_time
        ))
        .title("Doctest Report")
        .format()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ReportMessage;
    use serde_json::json;

    /// Test data for report messages: (JSON value, message instance, description)
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, ReportMessage)> {
        [(
            "report".to_owned(),
            json!({
                "type": "report",
                "total_time": 10.5,
                "compilation_time": 8.2,
            }),
            ReportMessage {
                total_time: 10.5,
                compilation_time: 8.2,
            },
        )]
        .into_iter()
    }
}
