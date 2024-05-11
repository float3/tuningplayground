#!/usr/bin/env bash

if [ "$1" = "dev" ]; then
  MODE="--dev"
  WEBPACK_MODE="development"
  ARGS="--features console_error_panic_hook"
elif [ "$1" = "prod" ]; then
  MODE="--release"
  WEBPACK_MODE="production"
else
  echo "Invalid argument. Use 'dev' for development or 'prod' for production."
  exit 1
fi

rm -rf www
cd ./tuningplayground
wasm-pack build --target web $MODE $ARGS
cd ../ts
npm install
npx tsc
npx webpack --mode $WEBPACK_MODE
