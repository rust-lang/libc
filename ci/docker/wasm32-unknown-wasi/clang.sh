#!/bin/sh
exec /wasmcc/bin/clang --target=wasm32-unknown-wasi --sysroot /wasm-sysroot "$@"
