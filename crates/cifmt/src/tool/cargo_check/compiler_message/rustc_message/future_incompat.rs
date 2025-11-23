//! Future-incompatibility reports from rustc.
//!
//! This module models the `future_incompat` message produced by rustc,
//! which contains warnings about code that will become errors in future
//! compiler releases.
use serde::Deserialize;

use crate::{
    ci::{GitHub, Plain},
    ci_message::CiMessage,
    tool::cargo_check::compiler_message::rustc_message::diagnostic::Diagnostic,
};

/// Future incompatibility report for warnings that will become errors.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FutureIncompat {
    /// Array of future incompatibility warnings.
    pub future_incompat_report: Vec<FutureIncompatEntry>,
}

impl CiMessage<Plain> for FutureIncompat {
    fn format(&self) -> String {
        let mut result = String::new();

        if !self.future_incompat_report.is_empty() {
            result.push_str("Future incompatibility warnings detected:\n");

            for entry in &self.future_incompat_report {
                result.push_str(&CiMessage::<Plain>::format(&entry.diagnostic));
            }
        }

        result
    }
}

impl CiMessage<GitHub> for FutureIncompat {
    fn format(&self) -> String {
        let mut result = String::new();

        if !self.future_incompat_report.is_empty() {
            result.push_str(
                &GitHub::warning("Future incompatibility warnings detected")
                    .title("Future Incompatibility Report")
                    .format(),
            );

            for entry in &self.future_incompat_report {
                result.push_str(&CiMessage::<GitHub>::format(&entry.diagnostic));
            }
        }

        result
    }
}

/// A single entry in the future incompatibility report.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FutureIncompatEntry {
    /// The diagnostic information.
    pub diagnostic: Diagnostic,
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::tool::cargo_check::compiler_message::rustc_message::diagnostic;

    use super::{FutureIncompat, FutureIncompatEntry};
    use serde_json::json;

    /// Test data for future incompatibility messages.
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, FutureIncompat)> {
        [(
            "future_incompat_empty".to_owned(),
            json!({
                "$message_type": "future_incompat",
                "future_incompat_report": [],
            }),
            FutureIncompat {
                future_incompat_report: vec![],
            },
        )]
        .into_iter()
        .chain(diagnostic::tests::cases().map(
            |(diagnostic_desc, diagnostic_value, diagnostic_msg)| {
                (
                    format!("future_incompat_with_warning_{diagnostic_desc}"),
                    json!({
                        "$message_type": "future_incompat",
                        "future_incompat_report": [{
                            "diagnostic": diagnostic_value,
                        }],
                    }),
                    FutureIncompat {
                        future_incompat_report: vec![FutureIncompatEntry {
                            diagnostic: diagnostic_msg,
                        }],
                    },
                )
            },
        ))
    }
}
