//! Supported CI platforms
//!
//! This module defines a number of [new
//! types](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
//! for representing the different CI platforms supported by this library.

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

pub fn from_env() -> Box<dyn Platform> {
    debug!("Detecting CI platform from environment variables");
    if let Some(env) = GitHub::from_env() {
        Box::new(env)
    } else {
        unimplemented!()
        // Box::new(Generic)
    }
}
