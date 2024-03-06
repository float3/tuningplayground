#!/usr/bin/env bash
curl https://raw.githubusercontent.com/float3/float3.github.io/master/static/styles.css -o ./src/styles.css

[ -f package.json ] && npm update
[ -f package.json ] && npm install
[ -f package.json ] && npm audit fix
npx tsc
npx eslint . --ext .ts,.tsx --fix
