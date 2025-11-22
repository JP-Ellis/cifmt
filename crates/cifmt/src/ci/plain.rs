//! Fallback plain text formatter.
//!
//! This formatter is used when no other formatter matches the CI environment.

use std::fmt;

use crate::ci::Platform;

/// Plain text formatter.
#[derive(Debug, Clone, Copy)]
pub struct Plain;

impl Platform for Plain {
    fn from_env() -> Option<Self>
    where
        Self: Sized,
    {
        Some(Plain)
    }
}

impl fmt::Display for Plain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plain Text Formatter")
    }
}
