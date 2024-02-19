#!/usr/bin/env bash
cd ./tuning_systems-wasm
wasm-pack build --target web --release # try --out-dir
cd ../ts
npm install
tsc
npm run build
