# cifmt

<!-- markdownlint-disable no-inline-html -->
<div align="center">
    <span>
        <b>
            A CI formatter library and CLI for pretty formatting JSON messages in CI environments.
        </b>
    </span>
</div>

<div align="center"><table>
    <tr>
        <td>Package</td>
        <td>
            <a href="https://crates.io/crates/cifmt"><img src="https://img.shields.io/crates/v/cifmt.svg" alt="Version"></a>
            <a href="https://crates.io/crates/cifmt"><img src="https://img.shields.io/crates/d/cifmt.svg" alt="Downloads"></a>
            <a href="https://docs.rs/cifmt"><img src="https://docs.rs/cifmt/badge.svg" alt="Documentation"></a>
        </td>
    </tr>
    <tr>
        <td>CI/CD</td>
        <td>
            <a
                href="https://github.com/JP-Ellis/cifmt/actions/workflows/deploy.yml"><img
                src="https://img.shields.io/github/actions/workflow/status/JP-Ellis/cifmt/deploy.yml?branch=main&label=CI"
                alt="CI Status"></a>
            <a
                href="https://github.com/JP-Ellis/cifmt/actions/workflows/test.yml"><img
                src="https://img.shields.io/github/actions/workflow/status/JP-Ellis/cifmt/test.yml?branch=main&label=tests"
                alt="Test Status"></a>
        </td>
    </tr>
    <tr>
        <td>Meta</td>
        <td>
            <a
                href="https://github.com/rust-lang/cargo"><img
                src="https://img.shields.io/badge/🦀-Cargo-blue.svg"
                alt="Cargo project"></a>
            <a href="https://github.com/rust-lang/rustfmt"><img
                src="https://img.shields.io/badge/code%20style-rustfmt-brightgreen.svg"
                alt="Code style - rustfmt"></a>
            <a href="https://github.com/rust-lang/rust-clippy"><img
                src="https://img.shields.io/badge/linting-clippy-blue.svg"
                alt="Linting - Clippy"></a>
            <a
                href="https://opensource.org/licenses/MIT"><img
                src="https://img.shields.io/badge/License-MIT-green.svg"
                alt="License"></a>
        </td>
    </tr>
    <tr>
        <td>Community</td>
        <td>
            <a
                href="https://github.com/JP-Ellis/cifmt/issues"><img
                src="https://img.shields.io/github/issues/JP-Ellis/cifmt.svg"
                alt="Issues"></a>
            <a
                href="https://github.com/JP-Ellis/cifmt/discussions"><img
                src="https://img.shields.io/github/discussions/JP-Ellis/cifmt.svg"
                alt="Discussions"></a>
            <a
                href="https://github.com/JP-Ellis/cifmt"><img
                src="https://img.shields.io/github/stars/JP-Ellis/cifmt.svg?style=social"
                alt="GitHub Stars"></a>
        </td>
    </tr>
</table></div>
<!-- markdownlint-enable no-inline-html -->

## Installation

### As a CLI Tool

Install the CLI globally using cargo:

```bash
cargo install cifmt
```

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
cifmt = "~0.1"
```

## Quick Start

### CLI Usage

Pipe JSON messages to `cifmt` to format them for your CI environment:

```bash
echo '{"type": "error", "message": "Something went wrong"}' | cifmt
```

Format with GitHub Actions syntax:

```bash
cifmt --format github < messages.json
```

### Library Usage

```rust
use cifmt::{Formatter, Message};

// Create a formatter for your CI platform
let formatter = Formatter::github();

// Format a message
let message = Message::error("Build failed");
println!("{}", formatter.format(&message));
```

## Supported CI Platforms

-   **GitHub Actions**: Groups, error annotations, warnings
-   **GitLab CI**: Collapsible sections, error formatting
-   **Generic**: Basic formatting for any CI platform

## Features

-   **Message grouping**: Organize output into collapsible sections
-   **Error highlighting**: Properly annotate errors and warnings
-   **File annotations**: Link messages to specific files and lines
-   **JSON input**: Parse structured JSON messages
-   **Multiple formats**: Support for various CI platforms

## Documentation

-   [API Documentation](https://docs.rs/cifmt)
-   [GitHub Repository](https://github.com/JP-Ellis/cifmt)

## Contributing

We welcome contributions! Please see our [Contributing Guide](./CONTRIBUTING.md) for details on:

-   Setting up the development environment
-   Running tests and examples
-   Code style and formatting guidelines
-   Submitting pull requests

## Testing

Run the test suite:

```bash
# Run tests with nextest (faster)
cargo nextest run

# Run integration tests
cargo test --test integration
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

-   📚 [Documentation](https://docs.rs/cifmt)
-   🐛 [Issue Tracker](https://github.com/JP-Ellis/cifmt/issues)
-   💬 [Discussions](https://github.com/JP-Ellis/cifmt/discussions)
