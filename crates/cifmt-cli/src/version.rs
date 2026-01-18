//! CLI version information.
//!
//! This module relies on `build.rs` to provide version information during
//! compilation, as exposed through various `CARGO_BUILD_*` environment
//! variables.

use std::{fmt, fmt::Write as _};

use serde::Serialize;

/// Version information.
///
/// This consists of a version tuple parsed from git tags, along with optional
/// commit information if available.
#[derive(Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct Version {
    /// The version tuple.
    ///
    /// This is a tuple of three integers representing the major, minor, and
    /// patch version numbers.
    pub version: (u32, u32, u32),

    /// Git commit information.
    ///
    /// This may be `None` if the build was not done from a git repository.
    pub commit_info: Option<CommitInfo>,
}

impl Version {
    /// Format the version as a semantic version string.
    ///
    /// Examples:
    /// - `1.2.3` - at a tag
    /// - `1.2.3-dev5+abcdef0` - 5 commits after tag
    #[must_use]
    pub fn as_semver(&self) -> String {
        let mut semver = format!("{}.{}.{}", self.version.0, self.version.1, self.version.2);

        if let Some(CommitInfo {
            tag_distance: Some(d),
            short_commit_hash,
            ..
        }) = &self.commit_info
            && *d > 0
        {
            #[expect(clippy::expect_used, reason = "Writing to a String should not fail")]
            write!(semver, "-dev{d}+{short_commit_hash}").expect("Failed to write to semver");
        }

        semver
    }
}

impl Default for Version {
    /// Retrieve version information from build-time environment variables.
    ///
    /// This will use the `CARGO_BUILD_TAG` environment variable to determine
    /// the version, falling back to `CARGO_PKG_VERSION` if not available (which
    /// is set by Cargo based on `Cargo.toml`).
    ///
    /// The version is expected to be a tag of the form `cifmt-cli/v?X.Y.Z` or
    /// for the Cargo package version fallback, simply `X.Y.Z`.
    ///
    /// # Panics
    ///
    /// This function will panic if the version information cannot be parsed
    /// (due to an invalid format).
    #[expect(
        clippy::expect_used,
        reason = "Parsing build-time version should not fail"
    )]
    fn default() -> Self {
        // Try to get version from git tag, fallback to CARGO_PKG_VERSION
        let mut version_str = option_env!("CARGO_BUILD_TAG").unwrap_or(env!("CARGO_PKG_VERSION"));

        // Strip the leading `cifmt-cli/` prefix, and `v` if present
        if let Some(stripped) = version_str.strip_prefix("cifmt-cli/") {
            version_str = stripped;
        }
        if let Some(stripped) = version_str.strip_prefix("v") {
            version_str = stripped;
        }

        let mut parts = version_str
            .split('.')
            .map(|s| s.parse::<u32>().expect("Failed to parse version segment"));
        let mut next = || parts.next().expect("Version string is missing segments");

        let version = (next(), next(), next());

        Self {
            version,
            commit_info: CommitInfo::from_build_env(),
        }
    }
}

impl fmt::Display for Version {
    /// Format version as: `X.Y.Z[.devN] (hash date)`
    ///
    /// Examples:
    /// - `1.2.3 (abcdef0 2025-01-15)` - at a tag
    /// - `1.2.3.dev5 (abcdef0 2025-01-15)` - 5 commits after tag
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.version.0, self.version.1, self.version.2
        )?;

        match &self.commit_info {
            None => {}
            Some(CommitInfo {
                tag_distance: Some(d),
                short_commit_hash,
                commit_date,
                ..
            }) if *d > 0 => write!(f, ".dev{d} ({short_commit_hash} {commit_date})")?,
            Some(CommitInfo {
                short_commit_hash,
                commit_date,
                ..
            }) => write!(f, " ({short_commit_hash} {commit_date})")?,
        }

        Ok(())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl From<Version> for clap::builder::Str {
    fn from(v: Version) -> Self {
        clap::builder::Str::from(v.to_string())
    }
}

/// Information about the git repository where the CLI was built.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CommitInfo {
    /// Short commit hash.
    pub short_commit_hash: String,
    /// Full commit hash.
    pub commit_hash: String,
    /// Commit date in YYYY-MM-DD format.
    pub commit_date: String,
    /// Last git tag.
    ///
    /// This may not be present if there are no tags, or if the build is from
    /// a shallow clone.
    pub tag: Option<String>,
    /// Number of commits since the last tag.
    ///
    /// As with `last_tag`, this may not always be present.
    pub tag_distance: Option<u32>,
}

impl CommitInfo {
    /// Read the environment variables set by build.rs and return a new
    /// `CommitInfo` instance.
    ///
    /// If any of the required environment variables are missing, this function
    /// will return `None`.
    pub(crate) fn from_build_env() -> Option<Self> {
        // At a minimum, we need the commit hash and date.
        let commit_hash = option_env!("CARGO_BUILD_COMMIT_HASH")?;
        let commit_date = option_env!("CARGO_BUILD_COMMIT_DATE")?;
        let short_commit_hash = option_env!("CARGO_BUILD_COMMIT_SHORT_HASH").unwrap_or_else(|| {
            // Fallback to first 7 characters of full hash
            let char_index = commit_hash
                .char_indices()
                .nth(7)
                .map_or(commit_hash.len(), |(idx, _)| idx);
            #[expect(clippy::string_slice, reason = "Index is on char boundary")]
            &commit_hash[0..char_index]
        });

        let tag = option_env!("CARGO_BUILD_TAG");
        let tag_distance =
            option_env!("CARGO_BUILD_TAG_DISTANCE").and_then(|s| s.parse::<u32>().ok());

        Some(Self {
            short_commit_hash: short_commit_hash.to_owned(),
            commit_hash: commit_hash.to_owned(),
            commit_date: commit_date.to_owned(),
            tag: tag.map(std::borrow::ToOwned::to_owned),
            tag_distance,
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn version_formatting_without_commit_info() {
        let version = Version {
            version: (1, 2, 3),
            commit_info: None,
        };
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn version_formatting_at_tag() {
        let version = Version {
            version: (1, 2, 3),
            commit_info: Some(CommitInfo {
                short_commit_hash: "abcdef0".to_owned(),
                commit_hash: "abcdef0123456789abcdef0123456789abcdef01".to_owned(),
                commit_date: "2025-01-15".to_owned(),
                tag: Some("v1.2.3".to_owned()),
                tag_distance: Some(0),
            }),
        };
        assert_eq!(version.to_string(), "1.2.3 (abcdef0 2025-01-15)");
    }

    #[test]
    fn version_formatting_with_dev_commits() {
        let version = Version {
            version: (1, 2, 3),
            commit_info: Some(CommitInfo {
                short_commit_hash: "abcdef0".to_owned(),
                commit_hash: "abcdef0123456789abcdef0123456789abcdef01".to_owned(),
                commit_date: "2025-01-15".to_owned(),
                tag: Some("v1.2.3".to_owned()),
                tag_distance: Some(5),
            }),
        };
        assert_eq!(version.to_string(), "1.2.3.dev5 (abcdef0 2025-01-15)");
    }

    #[test]
    fn version_semver_without_dev_commits() {
        let version = Version {
            version: (1, 2, 3),
            commit_info: Some(CommitInfo {
                short_commit_hash: "abcdef0".to_owned(),
                commit_hash: "abcdef0123456789abcdef0123456789abcdef01".to_owned(),
                commit_date: "2025-01-15".to_owned(),
                tag: Some("v1.2.3".to_owned()),
                tag_distance: Some(0),
            }),
        };
        assert_eq!(version.as_semver(), "1.2.3");
    }

    #[test]
    fn version_semver_with_dev_commits() {
        let version = Version {
            version: (1, 2, 3),
            commit_info: Some(CommitInfo {
                short_commit_hash: "abcdef0".to_owned(),
                commit_hash: "abcdef0123456789abcdef0123456789abcdef01".to_owned(),
                commit_date: "2025-01-15".to_owned(),
                tag: Some("v1.2.3".to_owned()),
                tag_distance: Some(5),
            }),
        };
        assert_eq!(version.as_semver(), "1.2.3-dev5+abcdef0");
    }
}
