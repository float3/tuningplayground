#!/usr/bin/env bash
cd ./tuning_systems-wasm
wasm-pack build --target web --dev --profile wasm-release

cd ../ts
npm install
tsc
npm run builddev
