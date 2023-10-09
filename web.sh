#!/bin/sh
cargo b --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/school-shooter.wasm docs
python -m http.server -d docs
