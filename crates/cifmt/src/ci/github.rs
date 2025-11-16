//! GitHub CI platform support.
//!
//! This module defines the GitHub platform marker and implements formatting of
//! CI messages for GitHub Actions.

use bon::bon;
use core::fmt;
use tracing::debug;

use crate::ci::Platform;

/// GitHub Action platform marker.
///
/// The GitHub Actions platform supports special workflow commands for
/// annotating files, grouping messages, and more.
///
/// For more information, see:
/// <https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions>
#[derive(Debug, Clone, Copy)]
pub struct GitHub;

impl Platform for GitHub {
    fn from_env() -> Option<Self>
    where
        Self: Sized,
    {
        if std::env::var("GITHUB_ACTIONS").is_ok() {
            debug!("Detected GitHub Actions environment");
            Some(GitHub)
        } else {
            None
        }
    }
}

impl fmt::Display for GitHub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GitHub Actions")
    }
}

/// Parameters for file annotations (error, warning, notice).
///
/// Used to specify optional location and metadata for annotations.
#[derive(Debug, Clone, Default)]
struct AnnotationParams<'a> {
    /// The file path to annotate.
    file: Option<&'a str>,
    /// The starting line number (1-indexed).
    line: Option<u32>,
    /// The starting column number (1-indexed).
    col: Option<u32>,
    /// The ending line number.
    end_line: Option<u32>,
    /// The ending column number.
    end_column: Option<u32>,
    /// Custom title for the annotation.
    title: Option<&'a str>,
}

impl fmt::Display for AnnotationParams<'_> {
    #[expect(
        unused_assignments,
        reason = "Last assignment of `needs_separator` is unused"
    )]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut needs_separator = false;

        macro_rules! write_param {
            ($format:expr, $value:expr) => {
                if let Some(v) = $value {
                    if needs_separator {
                        write!(f, ",")?;
                    }
                    write!(f, $format, v)?;
                    needs_separator = true;
                }
            };
        }

        write_param!("file={}", self.file);
        write_param!("line={}", self.line);
        write_param!("col={}", self.col);
        write_param!("endLine={}", self.end_line);
        write_param!("endColumn={}", self.end_column);
        write_param!("title={}", self.title);
        Ok(())
    }
}

#[bon]
impl GitHub {
    /// Formats a debug message for GitHub Actions.
    ///
    /// These messages are only visible when the workflow is run in debug mode.
    ///
    /// # Arguments
    ///
    /// * `message` - The debug message to format.
    ///
    /// # Returns
    ///
    /// A formatted debug message string, suitable for printing to stdout. The
    /// string includes a trailing newline.
    ///
    /// # Example
    ///
    /// ```
    /// use cifmt::ci::GitHub;
    ///
    /// let debug_message = GitHub::debug("This is a debug message.");
    /// ```
    pub fn debug(message: impl AsRef<str>) -> String {
        format!("::debug::{}\n", message.as_ref())
    }

    /// Creates a builder for a notice message.
    ///
    /// Notice messages create annotations which can optionally be associated
    /// with a specific file location.
    ///
    /// # Arguments
    ///
    /// * `message` - The notice message to display.
    /// * `file` - Optional file path for the annotation.
    /// * `line` - Optional starting line number (1-indexed).
    /// * `col` - Optional starting column number (1-indexed).
    /// * `end_line` - Optional ending line number.
    /// * `end_column` - Optional ending column number.
    /// * `title` - Optional custom title for the annotation.
    ///
    /// # Returns
    ///
    /// A builder that can be used to set optional parameters and format the
    /// notice.
    ///
    /// # Example
    ///
    /// ```
    /// use cifmt::ci::GitHub;
    ///
    /// // Simple notice
    /// let notice = GitHub::notice("Build completed successfully").format();
    ///
    /// // Notice with file annotation
    /// let notice = GitHub::notice("Consider refactoring")
    ///     .file("src/main.rs")
    ///     .line(42)
    ///     .title("Code Quality")
    ///     .format();
    /// ```
    #[builder(finish_fn = format)]
    pub fn notice(
        #[builder(start_fn)] message: impl AsRef<str>,
        file: Option<&str>,
        line: Option<u32>,
        col: Option<u32>,
        end_line: Option<u32>,
        end_column: Option<u32>,
        title: Option<&str>,
    ) -> String {
        let params = AnnotationParams {
            file,
            line,
            col,
            end_line,
            end_column,
            title,
        };
        format!("::notice {params}::{}\n", message.as_ref())
    }

    /// Creates a builder for a warning message.
    ///
    /// Warning messages create annotations which can optionally be associated
    /// with a specific file location.
    ///
    /// # Arguments
    ///
    /// * `message` - The warning message to display.
    /// * `file` - Optional file path for the annotation.
    /// * `line` - Optional starting line number (1-indexed).
    /// * `col` - Optional starting column number (1-indexed).
    /// * `end_line` - Optional ending line number.
    /// * `end_column` - Optional ending column number.
    /// * `title` - Optional custom title for the annotation.
    ///
    /// # Returns
    ///
    /// A builder that can be used to set optional parameters and format the
    /// warning.
    ///
    /// # Example
    ///
    /// ```
    /// use cifmt::ci::GitHub;
    ///
    /// // Simple warning
    /// let warning = GitHub::warning("Deprecated function used").format();
    ///
    /// // Warning with file annotation
    /// let warning = GitHub::warning("This function will be removed")
    ///     .file("src/lib.rs")
    ///     .line(100)
    ///     .col(5)
    ///     .title("Deprecation")
    ///     .format();
    /// ```
    #[builder(finish_fn = format)]
    pub fn warning(
        #[builder(start_fn)] message: impl AsRef<str>,
        file: Option<&str>,
        line: Option<u32>,
        col: Option<u32>,
        end_line: Option<u32>,
        end_column: Option<u32>,
        title: Option<&str>,
    ) -> String {
        let params = AnnotationParams {
            file,
            line,
            col,
            end_line,
            end_column,
            title,
        };
        format!("::warning {params}::{}\n", message.as_ref())
    }

    /// Creates a builder for an error message.
    ///
    /// Error messages create annotations which can optionally be associated
    /// with a specific file location.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to display.
    /// * `file` - Optional file path for the annotation.
    /// * `line` - Optional starting line number (1-indexed).
    /// * `col` - Optional starting column number (1-indexed).
    /// * `end_line` - Optional ending line number.
    /// * `end_column` - Optional ending column number.
    /// * `title` - Optional custom title for the annotation.
    ///
    /// # Returns
    ///
    /// A builder that can be used to set optional parameters and format the
    /// error.
    ///
    /// # Example
    ///
    /// ```
    /// use cifmt::ci::GitHub;
    ///
    /// // Simple error
    /// let error = GitHub::error("Build failed").format();
    ///
    /// // Error with file annotation
    /// let error = GitHub::error("Expected semicolon")
    ///     .file("src/main.rs")
    ///     .line(50)
    ///     .col(10)
    ///     .end_column(20)
    ///     .title("Compilation Error")
    ///     .format();
    /// ```
    #[builder(finish_fn = format)]
    pub fn error(
        #[builder(start_fn)] message: impl AsRef<str>,
        file: Option<&str>,
        line: Option<u32>,
        col: Option<u32>,
        end_line: Option<u32>,
        end_column: Option<u32>,
        title: Option<&str>,
    ) -> String {
        let params = AnnotationParams {
            file,
            line,
            col,
            end_line,
            end_column,
            title,
        };
        format!("::error {params}::{}\n", message.as_ref())
    }

    /// Starts a collapsible group in the workflow log.
    ///
    /// All output between this command and `endgroup()` will be nested inside
    /// an expandable group in the logs.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the group to display.
    ///
    /// # Returns
    ///
    /// A formatted group command string, suitable for printing to stdout. The
    /// string includes a trailing newline.
    ///
    /// # Example
    ///
    /// ```
    /// use cifmt::ci::GitHub;
    ///
    /// print!("{}", GitHub::group("Build Steps"));
    /// println!("Running build...");
    /// println!("Compiling...");
    /// print!("{}", GitHub::endgroup());
    /// ```
    pub fn group(title: impl AsRef<str>) -> String {
        format!("::group::{}\n", title.as_ref())
    }

    /// Ends a collapsible group in the workflow log.
    ///
    /// # Returns
    ///
    /// A formatted endgroup command string, suitable for printing to stdout.
    /// The string includes a trailing newline.
    ///
    /// # Example
    ///
    /// ```
    /// use cifmt::ci::GitHub;
    ///
    /// print!("{}", GitHub::group("Test Results"));
    /// println!("Running tests...");
    /// print!("{}", GitHub::endgroup());
    /// ```
    pub fn endgroup() -> String {
        "::endgroup::\n".to_string()
    }

    /// Masks a value in the workflow logs.
    ///
    /// After calling this command, any occurrence of the specified value in
    /// subsequent log output will be replaced with `***`. This is useful for
    /// preventing secrets from being displayed in logs.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to mask in logs.
    ///
    /// # Returns
    ///
    /// A formatted add-mask command string, suitable for printing to stdout.
    /// The string includes a trailing newline.
    ///
    /// # Example
    ///
    /// ```
    /// use cifmt::ci::GitHub;
    ///
    /// let secret = "my-secret-token";
    /// print!("{}", GitHub::add_mask(secret));
    /// println!("The secret is: {}", secret); // Will print: The secret is: ***
    /// ```
    pub fn add_mask(value: impl AsRef<str>) -> String {
        format!("::add-mask::{}\n", value.as_ref())
    }

    /// Stops processing workflow commands.
    ///
    /// This allows you to log anything without accidentally running a workflow
    /// command. You must provide a unique token that will be used to resume
    /// command processing later.
    ///
    /// # Arguments
    ///
    /// * `token` - A unique token used to resume command processing.
    ///
    /// # Returns
    ///
    /// A formatted stop-commands command string, suitable for printing to
    /// stdout. The string includes a trailing newline.
    ///
    /// # Example
    ///
    /// ```
    /// use cifmt::ci::GitHub;
    ///
    /// let token = "my-unique-token-12345";
    /// print!("{}", GitHub::stop_commands(token));
    /// println!("::warning:: This will NOT be processed as a command");
    /// print!("{}", GitHub::resume_commands(token));
    /// println!("::warning:: This WILL be processed as a command");
    /// ```
    pub fn stop_commands(token: impl AsRef<str>) -> String {
        format!("::stop-commands::{}\n", token.as_ref())
    }

    /// Resumes processing workflow commands.
    ///
    /// This must be called with the same token that was used in
    /// `stop_commands()`.
    ///
    /// # Arguments
    ///
    /// * `token` - The same unique token used in the corresponding
    ///   `stop_commands()` call.
    ///
    /// # Returns
    ///
    /// A formatted resume command string, suitable for printing to stdout. The
    /// string includes a trailing newline.
    ///
    /// # Example
    ///
    /// ```
    /// use cifmt::ci::GitHub;
    ///
    /// let token = "my-unique-token-12345";
    /// print!("{}", GitHub::stop_commands(token));
    /// println!("Commands are disabled here");
    /// print!("{}", GitHub::resume_commands(token));
    /// ```
    pub fn resume_commands(token: impl AsRef<str>) -> String {
        format!("::{}::\n", token.as_ref())
    }

    /// Enables or disables echoing of workflow commands.
    ///
    /// When enabled, workflow commands will be echoed to the log. When
    /// disabled, they will be processed silently.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable (`true`) or disable (`false`) command
    ///   echoing.
    ///
    /// # Returns
    ///
    /// A formatted echo command string, suitable for printing to stdout. The
    /// string includes a trailing newline.
    ///
    /// # Example
    ///
    /// ```
    /// use cifmt::ci::GitHub;
    ///
    /// // Disable command echoing
    /// print!("{}", GitHub::echo(false));
    /// println!("Commands will not be echoed");
    ///
    /// // Re-enable command echoing
    /// print!("{}", GitHub::echo(true));
    /// ```
    pub fn echo(enable: bool) -> String {
        let value = if enable { "on" } else { "off" };
        format!("::echo::{value}\n")
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::ci::GitHub;
    use crate::ci::Platform;

    #[rstest]
    fn debug() {
        let result = GitHub::debug("This is a debug message");
        insta::assert_snapshot!(result, @"::debug::This is a debug message\n");
    }

    #[rstest]
    fn notice_simple() {
        let result = GitHub::notice("Build completed").format();
        insta::assert_snapshot!(result, @"::notice ::Build completed\n");
    }

    #[rstest]
    fn notice_with_full_params() {
        let result = GitHub::notice("Full annotation")
            .file("src/main.rs")
            .line(42)
            .col(10)
            .end_line(45)
            .end_column(20)
            .title("Test Title")
            .format();
        insta::assert_snapshot!(result, @"::notice file=src/main.rs,line=42,col=10,endLine=45,endColumn=20,title=Test Title::Full annotation\n");
    }

    #[rstest]
    fn warning_simple() {
        let result = GitHub::warning("Deprecated API").format();
        insta::assert_snapshot!(result, @"::warning ::Deprecated API\n");
    }

    #[rstest]
    fn warning_with_params() {
        let result = GitHub::warning("This will be removed")
            .file("src/main.rs")
            .line(50)
            .col(5)
            .title("Deprecation Warning")
            .format();
        insta::assert_snapshot!(result, @"::warning file=src/main.rs,line=50,col=5,title=Deprecation Warning::This will be removed\n");
    }

    #[rstest]
    fn error_simple() {
        let result = GitHub::error("Build failed").format();
        insta::assert_snapshot!(result, @"::error ::Build failed\n");
    }

    #[rstest]
    fn error_with_params() {
        let result = GitHub::error("Unsupported syntax")
            .file("src/main.rs")
            .line(10)
            .col(1)
            .end_line(10)
            .end_column(15)
            .title("Syntax Error")
            .format();
        insta::assert_snapshot!(result, @"::error file=src/main.rs,line=10,col=1,endLine=10,endColumn=15,title=Syntax Error::Unsupported syntax");
    }

    #[rstest]
    fn group() {
        let result = GitHub::group("Build Steps");
        insta::assert_snapshot!(result, @"::group::Build Steps\n");
    }

    #[rstest]
    fn endgroup() {
        let result = GitHub::endgroup();
        insta::assert_snapshot!(result, @"::endgroup::\n");
    }

    #[rstest]
    fn add_mask() {
        let result = GitHub::add_mask("my-secret-token");
        insta::assert_snapshot!(result, @"::add-mask::my-secret-token\n");
    }

    #[rstest]
    fn stop_commands() {
        let result = GitHub::stop_commands("pause-token-123");
        insta::assert_snapshot!(result, @"::stop-commands::pause-token-123\n");
    }

    #[rstest]
    fn resume_commands() {
        let result = GitHub::resume_commands("pause-token-123");
        insta::assert_snapshot!(result, @"::pause-token-123::\n");
    }

    #[rstest]
    fn echo_enable() {
        let result = GitHub::echo(true);
        insta::assert_snapshot!(result, @"::echo::on\n");
    }

    #[rstest]
    fn echo_disable() {
        let result = GitHub::echo(false);
        insta::assert_snapshot!(result, @"::echo::off\n");
    }

    #[rstest]
    fn github_from_env_present() {
        unsafe {
            std::env::set_var("GITHUB_ACTIONS", "true");
        }
        let result = GitHub::from_env();
        assert!(result.is_some());
        unsafe {
            std::env::remove_var("GITHUB_ACTIONS");
        }
    }

    #[rstest]
    fn github_from_env_absent() {
        unsafe {
            std::env::remove_var("GITHUB_ACTIONS");
        }
        let result = GitHub::from_env();
        assert!(result.is_none());
    }
}
