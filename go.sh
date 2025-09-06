#!/bin/bash

set -e

cargo build \
  --release \
  --target wasm32-unknown-unknown

rm -rf ./out/

wasm-bindgen \
  --out-dir ./out/ \
  --target web \
  ./target/wasm32-unknown-unknown/release/cargo-pong.wasm

http-server -o