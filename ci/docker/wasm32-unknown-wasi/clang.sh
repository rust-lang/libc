#!/usr/bin/env sh
exec /wasmcc/bin/clang --target=wasm32-unknown-wasi --sysroot /wasi-sysroot "$@"
