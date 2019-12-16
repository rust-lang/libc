#!/usr/bin/env sh
exec /usr/bin/clang --target=wasm32-wasi --sysroot /wasi-libc/sysroot "$@"
