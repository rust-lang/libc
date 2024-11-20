#!/usr/bin/env bash

set -eux

apt-get update
apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    clang \
    xz-utils

# Wasmtime is used to execute tests and wasi-sdk is used to compile tests.
# Download appropriate versions here and configure various flags below.
#
# At the time of this writing wasmtime 24.0.0 is the latest release and
# wasi-sdk-24 is the latest release, that these numbers match is just
# coincidence.
wasmtime=24.0.0
wasi_sdk=24

curl -L https://github.com/bytecodealliance/wasmtime/releases/download/v$wasmtime/wasmtime-v$wasmtime-x86_64-linux.tar.xz |
    tar xJf -
mv wasmtime-v$wasmtime-x86_64-linux wasmtime

# The pre-built `*.deb` files for wasi-sdk install to `/opt/wasi-sdk`
curl -LO https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-$wasi_sdk/wasi-sdk-$wasi_sdk.0-x86_64-linux.deb
dpkg -i ./wasi-sdk-*.deb
