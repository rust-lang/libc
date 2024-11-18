#!/usr/bin/env bash

set -eux

# Note: keep in sync with:
# https://github.com/rust-lang/rust/blob/master/src/ci/docker/scripts/emscripten.sh
emsdk_version=3.1.68

git clone https://github.com/emscripten-core/emsdk.git /emsdk-portable
cd /emsdk-portable
./emsdk install "$emsdk_version"
./emsdk activate "$emsdk_version"

# Compile and cache libc
# shellcheck disable=SC1091
source ./emsdk_env.sh
echo "int main() {return 0;}" > a.c
HOME=/emsdk-portable/ emcc a.c
rm -f a.*

# Make emsdk usable by any user
chmod a+rxw -R /emsdk-portable
