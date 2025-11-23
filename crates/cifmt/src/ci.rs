//! Supported CI platforms
//!
//! This module defines a number of [new
//! types](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
//! for representing the different CI platforms supported by this library.

#![expect(
    clippy::pub_use,
    reason = "Keeping a flat module structure for CI platforms"
)]

mod github;
mod plain;

use core::fmt;

use tracing::debug;

pub use github::GitHub;
pub use plain::Plain;

/// Platform trait.
pub trait Platform: fmt::Display {
    /// Infer the CI platform from environment variables.
    ///
    /// Returns `Some(Self)` if the current environment matches this platform,
    /// otherwise returns `None`.
    fn from_env() -> Option<Self>
    where
        Self: Sized;
}

/// Detect the CI platform from environment variables.
///
/// Returns a boxed platform implementation. Falls back to `Plain` when no
/// specific platform is detected.
#[inline]
pub fn from_env() -> Box<dyn Platform> {
    debug!("Detecting CI platform from environment variables");
    if let Some(env) = GitHub::from_env() {
        Box::new(env)
    } else {
        // Fall back to the plain formatter when detection fails.
        Box::new(Plain)
    }
}
