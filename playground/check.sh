#!/usr/bin/env bash
cargo fmt --all

cargo fix --allow-dirty --allow-staged
cargo clippy --all-targets --all-features -- -D warnings

cargo check --release
cargo check --release --features console_error_panic_hook
cargo check
cargo check --features console_error_panic_hook

cargo test