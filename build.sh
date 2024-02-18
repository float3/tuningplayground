#!/usr/bin/env bash
#cargo build --release
#cargo build --release --features wasm-bindgen
cd ./tuning_systems-wasm
wasm-pack build --target web --profile wasm-release

cd ../ts
npm install
tsc
npm run builddev
