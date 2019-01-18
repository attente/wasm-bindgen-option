#!/bin/sh

cargo build --release --target=wasm32-unknown-unknown

wasm-bindgen --nodejs --out-dir client/src target/wasm32-unknown-unknown/release/wasm_bindgen_option.wasm
