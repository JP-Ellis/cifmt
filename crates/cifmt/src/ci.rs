//! Supported CI platforms
//!
//! This module defines a number of [new
//! types](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
//! for representing the different CI platforms supported by this library.

mod github;

pub use crate::ci::github::GitHub;

/// Platform trait.
pub trait Platform {
    /// Infer the CI platform from environment variables.
    ///
    /// Returns `Some(Self)` if the current environment matches this platform,
    /// otherwise returns `None`.
    fn from_env() -> Option<Self>
    where
        Self: Sized;
}

pub fn from_env() -> Box<dyn Platform> {
    if let Some(env) = GitHub::from_env() {
        Box::new(env)
    } else {
        unimplemented!()
        // Box::new(Generic)
    }
}
