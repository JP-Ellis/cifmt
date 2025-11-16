//! CLI Interface Tests

// TODO: Remove once upstream issue is fixed
// https://github.com/rust-lang/rust-clippy/issues/15764
#![cfg(test)]

use std::{fmt, path::PathBuf};

mod version;

/// Default replacements when formatting command output
///
/// These are applied in order, so more specific filters should go later. The
/// first filter is the regex to match, and the second is the replacement
/// string.
const DEFAULT_FILTERS: &[(&str, &str)] = &[
    // Replace version
    (
        r"\d+\.\d+\.\d+(\.dev\d+)? \([a-z0-9]+ \d{4}-\d{2}-\d{2}\)",
        "[VERSION] ([HASH] [DATE])",
    ),
    (r"\d+\.\d+\.\d+(\.dev\d+)?", "[VERSION]"),
];
/// A test context for CLI tests
///
/// This is used to set up a consistent environment to execute the CLI commands.
pub struct TestCommand {
    cli: PathBuf,
    cwd: assert_fs::TempDir,
    args: Vec<String>,
    env: Vec<(String, String)>,
    filters: Vec<(String, String)>,
}

impl TestCommand {
    /// Push a new argument to the command
    pub fn arg(&mut self, arg: impl Into<String>) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    /// Push multiple arguments to the command
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Set an environment variable for the command
    pub fn env(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.env.push((key.into(), value.into()));
        self
    }

    /// Set multiple environment variables for the command
    pub fn envs<I, K, V>(&mut self, envs: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.env
            .extend(envs.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Add an insta filter to the command
    pub fn filter(
        &mut self,
        pattern: impl Into<String>,
        replacement: impl Into<String>,
    ) -> &mut Self {
        self.filters.push((pattern.into(), replacement.into()));
        self
    }

    /// Add multiple insta filters to the command
    pub fn filters<I, P, R>(&mut self, filters: I) -> &mut Self
    where
        I: IntoIterator<Item = (P, R)>,
        P: Into<String>,
        R: Into<String>,
    {
        self.filters
            .extend(filters.into_iter().map(|(p, r)| (p.into(), r.into())));
        self
    }

    pub fn run_and_format(&self) -> String {
        let mut cmd = std::process::Command::new(&self.cli);

        cmd.current_dir(&self.cwd);

        cmd.args(&self.args);

        cmd.env_clear();
        cmd.envs([("LANG", "C"), ("LC_ALL", "C"), ("TZ", "UTC")]);
        cmd.envs(self.env.iter().map(|(k, v)| (k.as_str(), v.as_str())));

        let output = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to execute command '{}': {}", self.cli.display(), e)
        });

        let mut snapshot = String::new();

        snapshot.push_str(&format!("Success: {}\n", output.status.success()));
        snapshot.push_str(&format!(
            "Exit Code: {}\n",
            output.status.code().unwrap_or(!0_i32)
        ));
        snapshot.push_str("--- STDOUT ---\n");
        snapshot.push_str(&String::from_utf8_lossy(&output.stdout));
        snapshot.push_str("\n--- STDERR ---\n");
        snapshot.push_str(&String::from_utf8_lossy(&output.stderr));

        for (pattern, replacement) in self
            .filters
            .iter()
            .map(|(p, r)| (p.as_str(), r.as_str()))
            .chain(DEFAULT_FILTERS.iter().copied())
        {
            snapshot = regex::Regex::new(pattern)
                .expect("Invalid regex pattern in filter")
                .replace_all(&snapshot, replacement)
                .to_string();
        }

        snapshot
    }
}

impl Default for TestCommand {
    fn default() -> Self {
        Self {
            cli: env!("CARGO_BIN_EXE_cifmt").into(),
            cwd: assert_fs::TempDir::new().expect("Failed to create temp dir"),
            args: Vec::new(),
            env: Vec::new(),
            filters: Vec::new(),
        }
    }
}

/// Implement `Display` so that insta can execute the command and verify the
/// output
impl fmt::Display for TestCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.run_and_format())
    }
}
