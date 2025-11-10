//! CLI Interface Tests

// TODO: Remove once upstream issue is fixed
// https://github.com/rust-lang/rust-clippy/issues/15764
#![cfg(test)]

use std::{path::PathBuf, process};

/// Path to the cifmt binary.
///
/// See [Cargo environment
/// variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates)
/// for more details on the `CARGO_BIN_EXE_*` variables.
#[must_use]
pub fn get_bin() -> PathBuf {
    env!("CARGO_BIN_EXE_cifmt").into()
}

#[test]
fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    assert!(process::Command::new(get_bin()).spawn()?.wait()?.success());
    Ok(())
}
