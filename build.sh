#!/usr/bin/env bash
cd ./playground
wasm-pack build --target web --dev
cd ../ts
npm install
tsc
npm run builddev
