#!/usr/bin/env bash

cargo update --workspace --verbose
cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features --workspace --verbose -- -D warnings
cargo fix --allow-dirty --allow-staged --all-targets --all-features --workspace --verbose
cargo fmt --all --verbose
cargo check --all-targets --all-features --workspace --verbose
cargo test --all-targets --all-features --workspace --verbose

cd ./tuningplayground
wasm-pack build --target web --dev

cd ../music21-rs/music21
git pull origin master

cd ../
python3 -m venv venv
. venv/bin/activate
pip install -r music21/requirements.txt
python -m test
python -m generate_chords

cd ../ts
yarn upgrade
yarn npm audit fix
yarn prettier . --write
yarn eslint . --fix --ext .ts
