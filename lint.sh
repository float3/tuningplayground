#!/usr/bin/env bash

cd ./music21-rs/music21
git pull origin master

cd ../
python3 -m venv venv
. venv/bin/activate
pip3 install -r music21/requirements.txt
pip3 install music21
python3 -m test
python3 -m generate_chords

cd ../
cargo update --workspace
cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features --workspace -- -D warnings
cargo fix --allow-dirty --allow-staged --all-targets --all-features --workspace
cargo fmt --all
cargo check --all-targets --all-features --workspace
cargo test --all-targets --all-features --workspace

cd ./tuningplayground
wasm-pack build --target web --dev

cd ../ts
npm update
npm audit fix --force
npx prettier . --write
# npx eslint . --fix
