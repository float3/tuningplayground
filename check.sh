#!/usr/bin/env bash
cargo fmt --all

cargo fix --allow-dirty --allow-staged
cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features -- -D warnings

cargo check --release
cargo check --release --features wasm-bindgen
cargo check
cargo check --features wasm-bindgen

cargo test
