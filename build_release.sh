#!/usr/bin/env bash
rm -rf www
cd ./tuningplayground
wasm-pack build --target web --release
cd ../ts
curl https://raw.githubusercontent.com/float3/float3.github.io/master/static/styles.css -o ./src/styles.css
npm install
npx tsc
npm run build