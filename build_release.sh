#!/usr/bin/env bash
cd ./tuning_systems-wasm
wasm-pack build --target web --dev --profile wasm-release
cp ./target/wasm32-unknown-unknown/wasm-release ./target/wasm32-unknown-unknown/release
wasm-pack build --target web --dev
cd ../ts
npm install
tsc
npm run build
