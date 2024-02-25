#!/usr/bin/env bash
rm -rf www
cd ./playground
wasm-pack build --target web --dev --features console_error_panic_hook
cd ../ts
npm install
npx tsc
npm run builddev
