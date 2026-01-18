# Test Data

This directory contains static test data used in E2E tests. This data is generated once and committed to the repository to ensure test stability.

## Regenerating Test Data

Use the `generate` script to regenerate test data files:

```bash
# From this directory
./generate <filename>
```

## Files

### cargo-check.in

Example JSON output from `cargo check --message-format json`.

**To regenerate:**

```bash
./generate cargo-check.in
```

This creates a temporary Rust project with intentional warnings and errors, runs `cargo check`, and captures the JSON output with all paths normalized to placeholders.

### cargo-libtest.in

Example JSON output from `cargo test --message-format json -- -Z unstable-options --format json`.

**To regenerate:**

```bash
./generate cargo-libtest.in
```

This creates a temporary Rust project with intentional passing, failing, and ignored tests, runs `cargo test`, and captures the JSON output with all paths normalized to placeholders.
