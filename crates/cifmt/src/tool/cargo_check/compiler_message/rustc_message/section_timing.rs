//! Compilation section timing messages from rustc.
//!
//! This module models timing events emitted by rustc (unstable) that mark the
//! start and end of compilation phases. These are primarily useful for
//! diagnostic and profiling output in CI logs.
use serde::Deserialize;

use crate::{
    ci::{GitHub, Plain},
    ci_message::CiMessage,
};

/// Compilation section timing information (unstable).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SectionTiming {
    /// Event type ("start" or "end").
    pub event: TimingEvent,
    /// Name of the compilation section.
    pub name: String,
    /// Timestamp in microseconds (relative to compilation start).
    pub time: u64,
}

impl CiMessage<Plain> for SectionTiming {
    fn format(&self) -> String {
        format!(
            "Compilation section {} {}: {} ({}μs)",
            self.name, self.event, self.name, self.time
        )
    }
}

impl CiMessage<GitHub> for SectionTiming {
    fn format(&self) -> String {
        GitHub::debug(format!(
            "Compilation section {} {}: {} ({}μs)",
            self.name, self.event, self.name, self.time
        ))
    }
}

/// Timing event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
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
pub(crate) mod tests {
    use super::{SectionTiming, TimingEvent};
    use serde_json::json;

    /// Test data for section timing messages.
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, SectionTiming)> {
        [
            (
                "section_timing_start".to_owned(),
                json!({
                    "$message_type": "section_timing",
                    "event": "start",
                    "name": "codegen",
                    "time": 1_234_567,
                }),
                SectionTiming {
                    event: TimingEvent::Start,
                    name: "codegen".to_owned(),
                    time: 1_234_567,
                },
            ),
            (
                "section_timing_end".to_owned(),
                json!({
                    "$message_type": "section_timing",
                    "event": "end",
                    "name": "codegen",
                    "time": 2_345_678,
                }),
                SectionTiming {
                    event: TimingEvent::End,
                    name: "codegen".to_owned(),
                    time: 2_345_678,
                },
            ),
        ]
        .into_iter()
    }
}
