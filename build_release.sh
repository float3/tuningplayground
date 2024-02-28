#!/usr/bin/env bash
rm -rf www
cd ./tuningplayground
wasm-pack build --target web --release # try --out-dir
cd ../ts
npm install
npx tsc
npm run build
