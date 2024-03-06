#!/usr/bin/env bash
cargo fmt --all
cargo fix --allow-dirty --allow-staged --all-targets --all-features
cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features -- -D warnings
cargo check --all-targets --all-features
cargo test --all-targets --all-features