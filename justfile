setup:
    cargo install cargo-binstall
    cargo binstall cargo-nextest
    cargo binstall cargo-insta

test:
    cargo nextest run --workspace --all-targets || cargo insta review
    cargo test --workspace --doc
