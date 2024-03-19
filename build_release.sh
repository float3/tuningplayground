#!/usr/bin/env bash
rm -rf www
cd ./tuningplayground
wasm-pack build --target web --release
cd ../ts
npm install
npx tsc
npx webpack --mode production