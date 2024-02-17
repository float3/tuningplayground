#!/usr/bin/env bash
cargo build --release
cd tuning_systems-wasm
wasm-pack build --target web --release