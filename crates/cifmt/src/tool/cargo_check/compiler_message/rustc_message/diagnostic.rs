//! Diagnostic messages from rustc.

use crate::ci::{GitHub, Plain};
use crate::ci_message::CiMessage;
use serde::{Deserialize, Serialize};

/// A diagnostic message from the compiler.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// The primary message.
    pub message: String,
    /// The diagnostic code.
    pub code: Option<DiagnosticCode>,
    /// Severity level.
    pub level: DiagnosticLevel,
    /// Source code spans.
    pub spans: Vec<DiagnosticSpan>,
    /// Child diagnostics (notes, help, etc.).
    pub children: Vec<Diagnostic>,
    /// Rendered version of the diagnostic.
    pub rendered: Option<String>,
}

impl CiMessage<Plain> for Diagnostic {
    fn format(&self) -> String {
        let mut result = String::new();

        // Format the main diagnostic
        let annotation = match self.level {
            DiagnosticLevel::Error | DiagnosticLevel::InternalCompilerError => {
                let title = if let Some(code) = &self.code {
                    format!("{}: {}", self.level, code.code)
                } else {
                    self.level.to_string()
                };

                format!("error: {} ({})\n", self.message, title)
            }
            DiagnosticLevel::Warning => {
                let title = if let Some(code) = &self.code {
                    format!("{}: {}", self.level, code.code)
                } else {
                    self.level.to_string()
                };

                format!("warning: {} ({})\n", self.message, title)
            }
            DiagnosticLevel::Note | DiagnosticLevel::Help | DiagnosticLevel::FailureNote => {
                format!("{}: {}\n", self.level, self.message)
            }
        };

        result.push_str(&annotation);

        // Format child diagnostics (notes, help messages, etc.)
        for child in &self.children {
            result.push_str(&<Diagnostic as CiMessage<Plain>>::format(child));
        }

        result
    }
}

impl CiMessage<GitHub> for Diagnostic {
    fn format(&self) -> String {
        // Find the primary span for location information
        let primary_span = self.spans.iter().find(|s| s.is_primary);

        let mut result = String::new();

        // Format the main diagnostic
        let annotation = match self.level {
            DiagnosticLevel::Error | DiagnosticLevel::InternalCompilerError => {
                let title = if let Some(code) = &self.code {
                    format!("{}: {}", self.level, code.code)
                } else {
                    self.level.to_string()
                };

                if let Some(span) = primary_span {
                    GitHub::error(&self.message)
                        .file(&span.file_name)
                        .line(span.line_start)
                        .col(span.column_start)
                        .end_line(span.line_end)
                        .end_column(span.column_end)
                        .title(&title)
                        .format()
                } else {
                    GitHub::error(&self.message).title(&title).format()
                }
            }
            DiagnosticLevel::Warning => {
                let title = if let Some(code) = &self.code {
                    format!("{}: {}", self.level, code.code)
                } else {
                    self.level.to_string()
                };

                if let Some(span) = primary_span {
                    GitHub::warning(&self.message)
                        .file(&span.file_name)
                        .line(span.line_start)
                        .col(span.column_start)
                        .end_line(span.line_end)
                        .end_column(span.column_end)
                        .title(&title)
                        .format()
                } else {
                    GitHub::warning(&self.message).title(&title).format()
                }
            }
            DiagnosticLevel::Note | DiagnosticLevel::Help | DiagnosticLevel::FailureNote => {
                // For child diagnostics, format as notice
                if let Some(span) = primary_span {
                    GitHub::notice(&self.message)
                        .file(&span.file_name)
                        .line(span.line_start)
                        .col(span.column_start)
                        .title(&self.level.to_string())
                        .format()
                } else {
                    GitHub::notice(&self.message)
                        .title(&self.level.to_string())
                        .format()
                }
            }
        };

        result.push_str(&annotation);

        // Format child diagnostics (notes, help messages, etc.)
        for child in &self.children {
            result.push_str(&<Diagnostic as CiMessage<GitHub>>::format(child));
        }

        result
    }
}

/// Diagnostic code information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticCode {
    /// Unique code identifying the diagnostic.
    pub code: String,
    /// Optional explanation.
    pub explanation: Option<String>,
}

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticLevel {
    /// Fatal error preventing compilation.
    #[serde(rename = "error")]
    Error,
    /// Possible error or concern.
    #[serde(rename = "warning")]
    Warning,
    /// Additional information.
    #[serde(rename = "note")]
    Note,
    /// Suggestion on how to resolve.
    #[serde(rename = "help")]
    Help,
    /// Additional failure information.
    #[serde(rename = "failure-note")]
    FailureNote,
    /// Internal compiler error.
    #[serde(rename = "error: internal compiler error")]
    InternalCompilerError,
}

impl std::fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Note => write!(f, "note"),
            Self::Help => write!(f, "help"),
            Self::FailureNote => write!(f, "failure-note"),
            Self::InternalCompilerError => write!(f, "error: internal compiler error"),
        }
    }
}

/// Source code span information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSpan {
    /// File path.
    pub file_name: String,
    /// Byte offset start (0-based, inclusive).
    pub byte_start: usize,
    /// Byte offset end (0-based, exclusive).
    pub byte_end: usize,
    /// Starting line number (1-based, inclusive).
    pub line_start: u32,
    /// Ending line number (1-based, inclusive).
    pub line_end: u32,
    /// Starting column number (1-based, inclusive).
    pub column_start: u32,
    /// Ending column number (1-based, exclusive).
    pub column_end: u32,
    /// Whether this is the primary span.
    pub is_primary: bool,
    /// Source text for the span.
    pub text: Vec<DiagnosticSpanLine>,
    /// Optional label for this span.
    pub label: Option<String>,
    /// Suggested replacement text.
    pub suggested_replacement: Option<String>,
    /// Suggestion applicability level.
    pub suggestion_applicability: Option<SuggestionApplicability>,
    /// Macro expansion information.
    pub expansion: Option<Box<DiagnosticSpanMacroExpansion>>,
}

/// A line of source text in a diagnostic span.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSpanLine {
    /// The full text of the line.
    pub text: String,
    /// Starting column of the highlight (1-based, inclusive).
    pub highlight_start: u32,
    /// Ending column of the highlight (1-based, exclusive).
    pub highlight_end: u32,
}

/// Suggestion applicability level.
///
/// Indicates the confidence of a suggested replacement and whether it should
/// be automatically applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestionApplicability {
    /// The suggestion is definitely what the user intended and should be
    /// automatically applied.
    MachineApplicable,
    /// The suggestion may be what the user intended, but it is uncertain.
    /// Should result in valid Rust code if applied.
    MaybeIncorrect,
    /// The suggestion contains placeholders like `(...)` and cannot be applied
    /// automatically because it will not result in valid Rust code.
    HasPlaceholders,
    /// The applicability of the suggestion is unknown.
    Unspecified,
}

/// Macro expansion information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSpanMacroExpansion {
    /// Span of the macro invocation.
    pub span: DiagnosticSpan,
    /// Name of the macro.
    pub macro_decl_name: String,
    /// Optional span where the macro is defined.
    pub def_site_span: Option<DiagnosticSpan>,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::{Diagnostic, DiagnosticCode, DiagnosticLevel, DiagnosticSpan};
    use serde_json::json;

    /// Test data for diagnostic messages.
    #[expect(
        clippy::too_many_lines,
        reason = "Test data with many fields and variants"
    )]
    pub fn cases() -> impl Iterator<Item = (String, serde_json::Value, Diagnostic)> {
        [
            (
                "error_with_code".to_owned(),
                json!({
                    "$message_type": "diagnostic",
                    "message": "unused variable: `x`",
                    "code": {
                        "code": "unused_variables",
                        "explanation": null,
                    },
                    "level": "error",
                    "spans": [{
                        "file_name": "src/main.rs",
                        "byte_start": 50,
                        "byte_end": 51,
                        "line_start": 3,
                        "line_end": 3,
                        "column_start": 9,
                        "column_end": 10,
                        "is_primary": true,
                        "text": [{
                            "text": "    let x = 5;",
                            "highlight_start": 9,
                            "highlight_end": 10,
                        }],
                        "label": "unused variable",
                        "suggested_replacement": null,
                        "suggestion_applicability": null,
                        "expansion": null,
                    }],
                    "children": [],
                    "rendered": null,
                }),
                Diagnostic {
                    message: "unused variable: `x`".to_owned(),
                    code: Some(DiagnosticCode {
                        code: "unused_variables".to_owned(),
                        explanation: None,
                    }),
                    level: DiagnosticLevel::Error,
                    spans: vec![DiagnosticSpan {
                        file_name: "src/main.rs".to_owned(),
                        byte_start: 50,
                        byte_end: 51,
                        line_start: 3,
                        line_end: 3,
                        column_start: 9,
                        column_end: 10,
                        is_primary: true,
                        text: vec![super::DiagnosticSpanLine {
                            text: "    let x = 5;".to_owned(),
                            highlight_start: 9,
                            highlight_end: 10,
                        }],
                        label: Some("unused variable".to_owned()),
                        suggested_replacement: None,
                        suggestion_applicability: None,
                        expansion: None,
                    }],
                    children: vec![],
                    rendered: None,
                },
            ),
            (
                "warning_without_code".to_owned(),
                json!({
                    "$message_type": "diagnostic",
                    "message": "unused import: `std::io`",
                    "code": null,
                    "level": "warning",
                    "spans": [{
                        "file_name": "src/lib.rs",
                        "byte_start": 10,
                        "byte_end": 18,
                        "line_start": 1,
                        "line_end": 1,
                        "column_start": 5,
                        "column_end": 13,
                        "is_primary": true,
                        "text": [{
                            "text": "use std::io;",
                            "highlight_start": 5,
                            "highlight_end": 13,
                        }],
                        "label": null,
                        "suggested_replacement": null,
                        "suggestion_applicability": null,
                        "expansion": null,
                    }],
                    "children": [],
                    "rendered": null,
                }),
                Diagnostic {
                    message: "unused import: `std::io`".to_owned(),
                    code: None,
                    level: DiagnosticLevel::Warning,
                    spans: vec![DiagnosticSpan {
                        file_name: "src/lib.rs".to_owned(),
                        byte_start: 10,
                        byte_end: 18,
                        line_start: 1,
                        line_end: 1,
                        column_start: 5,
                        column_end: 13,
                        is_primary: true,
                        text: vec![super::DiagnosticSpanLine {
                            text: "use std::io;".to_owned(),
                            highlight_start: 5,
                            highlight_end: 13,
                        }],
                        label: None,
                        suggested_replacement: None,
                        suggestion_applicability: None,
                        expansion: None,
                    }],
                    children: vec![],
                    rendered: None,
                },
            ),
        ]
        .into_iter()
    }
}
