#!/usr/bin/env bash
cd ./playground
wasm-pack build --target web --dev
cd ../ts
npm install
npx tsc
npm run builddev
