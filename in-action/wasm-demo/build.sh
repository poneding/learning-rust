#!/bin/bash

# Step 1: cargo build
cargo build --target wasm32-wasi --release

# Step 2: docker build
docker buildx use wasmbuilder || docker buildx create --use --name wasmbuilder
docker buildx build --push --platform wasi/wasm -t poneding/rust-hello .