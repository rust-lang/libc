#!/usr/bin/env bash

set -ex

# FIXME: 3.1.21 removed a lot of header files (https://github.com/emscripten-core/emscripten/pull/17704).
#        We have to tweak libc-test (and deprecate unsupported items, maybe) when updating emsdk.
EMSDK_VERSION=3.1.20

git clone https://github.com/emscripten-core/emsdk.git /emsdk-portable
cd /emsdk-portable
./emsdk install "${EMSDK_VERSION}"
./emsdk activate "${EMSDK_VERSION}"

# Compile and cache libc
# shellcheck disable=SC1091
source ./emsdk_env.sh
echo "int main() {return 0;}" > a.c
HOME=/emsdk-portable/ emcc a.c
rm -f a.*

# Make emsdk usable by any user
chmod a+rxw -R /emsdk-portable

# node 8 is required to run wasm
# NOTE: Do not forget to sync Node.js version with `emscripten-entry.sh`!
cd /
curl --retry 5 -L https://nodejs.org/dist/v14.17.0/node-v14.17.0-linux-x64.tar.xz | \
    tar -xJ
