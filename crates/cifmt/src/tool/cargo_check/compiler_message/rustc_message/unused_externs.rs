//! Unused extern crate diagnostics from rustc.
//!
//! This module represents the `unused_externs` message which rustc emits to
//! report unused extern crate dependencies.
use serde::Deserialize;

use crate::{
    ci::{GitHub, Plain},
    ci_message::CiMessage,
};

/// Unused extern crate dependencies report.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UnusedExterns {
    /// Level of the lint (warn, deny, forbid).
    pub lint_level: String,
    /// Names of unused crates.
    pub unused_names: Vec<String>,
}

impl CiMessage<Plain> for UnusedExterns {
    fn format(&self) -> String {
        if self.unused_names.is_empty() {
            return String::new();
        }

        let message = format!("Unused dependencies: {}", self.unused_names.join(", "));

        match self.lint_level.as_str() {
            "deny" | "forbid" => format!("error: {message}"),
            _ => format!("warning: {message}"),
        }
    }
}

impl CiMessage<GitHub> for UnusedExterns {
    fn format(&self) -> String {
        if self.unused_names.is_empty() {
            return String::new();
        }

        let message = format!("Unused dependencies: {}", self.unused_names.join(", "));

        match self.lint_level.as_str() {
            "deny" | "forbid" => GitHub::error(&message)
                .title("Unused Dependencies")
                .format(),
            _ => GitHub::warning(&message)
                .title("Unused Dependencies")
                .format(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::UnusedExterns;
    use serde_json::json;

    /// Test data for unused extern messages.
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, UnusedExterns)> {
        [
            (
                "unused_externs_warn".to_owned(),
                json!({
                    "$message_type": "unused_externs",
                    "lint_level": "warn",
                    "unused_names": ["serde", "tokio"],
                }),
                UnusedExterns {
                    lint_level: "warn".to_owned(),
                    unused_names: vec!["serde".to_owned(), "tokio".to_owned()],
                },
            ),
            (
                "unused_externs_deny".to_owned(),
                json!({
                    "$message_type": "unused_externs",
                    "lint_level": "deny",
                    "unused_names": ["unused_crate"],
                }),
                UnusedExterns {
                    lint_level: "deny".to_owned(),
                    unused_names: vec!["unused_crate".to_owned()],
                },
            ),
            (
                "unused_externs_empty".to_owned(),
                json!({
                    "$message_type": "unused_externs",
                    "lint_level": "warn",
                    "unused_names": [],
                }),
                UnusedExterns {
                    lint_level: "warn".to_owned(),
                    unused_names: vec![],
                },
            ),
        ]
        .into_iter()
    }
}
