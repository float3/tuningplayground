#!/usr/bin/env bash
cargo build --release
#cargo build --release --features wasm-bindgen
cd tuning_systems-wasm
wasm-pack build --target web --release
cd www
tsc
npm install
npm run build