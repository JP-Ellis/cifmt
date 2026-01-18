//! CLI Interface Tests

// TODO: Remove once upstream issue is fixed
// https://github.com/rust-lang/rust-clippy/issues/15764
#![cfg(test)]

use std::{fmt, fmt::Write as _, path::PathBuf};

mod format;
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
    // Filter ISO 8601 timestamps in error logs (e.g., 2026-01-18T04:36:40.825075Z)
    (
        r"\x1b\[2m\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z\x1b\[0m",
        "[2m[TIMESTAMP][0m",
    ),
    // Debug logging with timestamps - filter timestamps with microsecond precision
    (r"\x1b\[2m\s*\d+\.\d+s\x1b\[0m", "[2m[TIME][0m"),
    // Filter time.busy and time.idle values in logs
    (
        r"\x1b\[3mtime\.busy\x1b\[0m\x1b\[2m=\x1b\[0m[\d.]+[µnm]?s",
        "[3mtime.busy[0m[2m=[0m[TIME]",
    ),
    (
        r"\x1b\[3mtime\.idle\x1b\[0m\x1b\[2m=\x1b\[0m[\d.]+[µnm]?s",
        "[3mtime.idle[0m[2m=[0m[TIME]",
    ),
    // Debug logging with timestamps (legacy format)
    (
        r"\x1b\[\d+m\s*\d+\.\d{9}s\x1b\[0m \x1b\[\d+m([A-Z]+)\x1b\[0m \x1b\[\d+m([\w:]+)\x1b\[0m\x1b\[\d+m:\x1b\[0m \x1b\[\d+m(\d+):\x1b\[0m",
        "[RUN_TIME] $1 $2: $3",
    ),
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
    #[must_use]
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Push multiple arguments to the command
    #[must_use]
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Set an environment variable for the command
    #[must_use]
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }

    /// Set multiple environment variables for the command
    #[must_use]
    pub fn envs<I, K, V>(mut self, envs: I) -> Self
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
    #[must_use]
    pub fn filter(mut self, pattern: impl Into<String>, replacement: impl Into<String>) -> Self {
        self.filters.push((pattern.into(), replacement.into()));
        self
    }

    /// Add multiple insta filters to the command
    #[must_use]
    pub fn filters<I, P, R>(mut self, filters: I) -> Self
    where
        I: IntoIterator<Item = (P, R)>,
        P: Into<String>,
        R: Into<String>,
    {
        self.filters
            .extend(filters.into_iter().map(|(p, r)| (p.into(), r.into())));
        self
    }

    /// Run the command and format the output as a snapshot string.
    ///
    /// # Returns
    ///
    /// A formatted string containing the command's stdout, stderr, and exit status.
    ///
    /// # Panics
    ///
    /// Panics if the command fails to execute.
    #[must_use]
    pub fn run_and_format(&self) -> String {
        self.run_and_format_with_stdin(None)
    }

    /// Run the command with stdin input and format the output as a snapshot string.
    ///
    /// # Arguments
    ///
    /// * `stdin_input` - Optional string to write to the command's stdin
    ///
    /// # Returns
    ///
    /// A formatted string containing the command's stdout, stderr, and exit status.
    ///
    /// # Panics
    ///
    /// Panics if the command fails to execute or stdin write fails.
    #[must_use]
    pub fn run_and_format_with_stdin(&self, stdin_input: Option<&str>) -> String {
        let mut cmd = std::process::Command::new(&self.cli);

        cmd.current_dir(&self.cwd);
        cmd.args(&self.args);
        cmd.env_clear();
        cmd.envs([("LANG", "C"), ("LC_ALL", "C"), ("TZ", "UTC")]);
        cmd.envs(self.env.iter().map(|(k, v)| (k.as_str(), v.as_str())));

        let output = if let Some(input) = stdin_input {
            use std::io::Write as _;
            cmd.stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped());

            let mut process = cmd.spawn().unwrap_or_else(|e| {
                panic!("Failed to spawn command '{}': {}", self.cli.display(), e)
            });

            if let Some(mut stdin) = process.stdin.take() {
                stdin
                    .write_all(input.as_bytes())
                    .expect("Failed to write to stdin");
            }

            process.wait_with_output().unwrap_or_else(|e| {
                panic!("Failed to wait for command '{}': {}", self.cli.display(), e)
            })
        } else {
            cmd.output().unwrap_or_else(|e| {
                panic!("Failed to execute command '{}': {}", self.cli.display(), e)
            })
        };

        let mut snapshot = String::new();

        write!(
            snapshot,
            "Success: {}\n\
            Exit Code: {}\n\
            --- STDOUT ---\n\
            {}\n\
            --- STDERR ---\n\
            {}",
            output.status.success(),
            output.status.code().unwrap_or(!0_i32),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        )
        .expect("Failed to write command output to snapshot");

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

/// Adjust snapshot names based on variable names and values.
macro_rules! set_snapshot_suffix {
    ($($expr:expr),+ $(,)?) => {
        let mut settings = ::insta::Settings::clone_current();
        settings.set_snapshot_suffix(
            format!(
                concat!("" $(, stringify!($expr), "={}",)"##"*),
                $($expr),*
            )
        );
        let _guard = settings.bind_to_scope();
    }
}

/// Re-export the macros
///
/// See: <https://stackoverflow.com/a/31749071/3549270>
#[allow(
    clippy::allow_attributes,
    unused_imports,
    reason = "Re-exporting the macro"
)]
pub(crate) use set_snapshot_suffix;
