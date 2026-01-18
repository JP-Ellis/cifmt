#![cfg(test)]

use rstest::{fixture, rstest};

use crate::{TestCommand, set_snapshot_suffix};

/// Get cargo check JSON output for testing from static test data.
///
/// This uses pre-generated test data instead of running `cargo check`
/// dynamically to ensure test stability across code changes.
///
/// # Returns
///
/// Static JSON output representing `cargo check --message-format json`
///
/// # Panics
///
/// Panics if the test data file cannot be read
///
/// # Regeneration
///
/// To regenerate this test data, run:
///
/// ```bash
/// cd crates/cifmt-cli/tests/cli/test_data
/// ./generate cargo-check.in
/// ```
///
/// See `test_data/README.md` for more details.
#[fixture]
fn output() -> String {
    std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/cli/test_data/cargo-check.in"
    ))
    .expect("Failed to read test data file")
}

#[rstest]
fn format_manual(output: String) {
    let cmd = TestCommand::default().arg("format").arg("cargo-check");
    insta::assert_snapshot!(cmd.run_and_format_with_stdin(Some(&output)));
}

#[rstest]
fn format_detect(output: String) {
    let cmd = TestCommand::default().arg("format").arg("--detect");
    insta::assert_snapshot!(cmd.run_and_format_with_stdin(Some(&output)));
}

#[rstest]
#[case("plain", None)]
#[case("github", Some("true"))]
fn format_platform(
    #[case] platform_name: &str,
    #[case] github_actions_env: Option<&str>,
    output: String,
) {
    set_snapshot_suffix!(platform_name, github_actions_env.is_some());

    let mut cmd = TestCommand::default().arg("format").arg("--detect");
    if let Some(val) = github_actions_env {
        cmd = cmd.env("GITHUB_ACTIONS", val);
    }

    insta::assert_snapshot!(cmd.run_and_format_with_stdin(Some(&output)));
}

#[rstest]
fn format_no_tool_specified(output: String) {
    let cmd = TestCommand::default().arg("format");
    insta::assert_snapshot!(cmd.run_and_format_with_stdin(Some(&output)));
}
