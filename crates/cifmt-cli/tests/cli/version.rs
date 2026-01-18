use core::iter;

use rstest::{fixture, rstest};

use crate::{TestCommand, set_snapshot_suffix};

#[fixture]
fn cmd() -> TestCommand {
    TestCommand::default()
        .arg("version")
        .filter(
            r#""version": \[\n(\s+\d+,?\n){3}\s+\]"#,
            r#""version": [<int>, <int>, <int>]"#,
        )
        .filter(
            r#""short_commit_hash": "[a-f0-9]+""#,
            r#""short_commit_hash": "[SHORT_HASH]""#,
        )
        .filter(
            r#""commit_hash": "[a-f0-9]+""#,
            r#""commit_hash": "[HASH]""#,
        )
        .filter(
            r#""build_date": "\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z""#,
            r#""build_date": "[DATETIME]""#,
        )
        .filter(
            r#""build_date": "\d{4}-\d{2}-\d{2}""#,
            r#""build_date": "[DATE]""#,
        )
        .filter(r#""tag": (null|"[a-zA-Z0-9\.-]+")"#, r#""tag": "[TAG]""#)
        .filter(
            r#""tag_distance": (null|\d+)"#,
            r#""tag_distance": "[DISTANCE]""#,
        )
}

#[rstest]
fn version(
    cmd: TestCommand,
    #[values("text", "json")] output_format: &str,
    #[values(0, 1, 2)] verbosity: usize,
) {
    set_snapshot_suffix!(output_format, verbosity);
    insta::assert_snapshot!(
        cmd.arg("--output-format")
            .arg(output_format)
            .args(iter::repeat_n("-v", verbosity))
    );
}
