#!/bin/sh

echo ">> Building contract"

RUSTFLAGS="-C link-args=-s" cargo build --target wasm32-unknown-unknown --release
cargo build --all --target wasm32-unknown-unknown --release