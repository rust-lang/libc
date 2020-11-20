#!/usr/bin/env bash

set -ex

EMSDK_VERSION=1.39.20

hide_output() {
  set +x
  on_err="
echo ERROR: An error was encountered with the build.
cat /tmp/build.log
exit 1
"
  trap '$on_err' ERR
  bash -c "while true; do sleep 30; echo \$(date) - building ...; done" &
  PING_LOOP_PID=$!
  "${@}" &> /tmp/build.log
  trap - ERR
  kill $PING_LOOP_PID
  rm -f /tmp/build.log
  set -x
}

git clone https://github.com/emscripten-core/emsdk.git /emsdk-portable
cd /emsdk-portable
hide_output ./emsdk install "${EMSDK_VERSION}"
./emsdk activate "${EMSDK_VERSION}"

# Compile and cache libc
# shellcheck disable=SC1091
source ./emsdk_env.sh
echo "main(){}" > a.c
HOME=/emsdk-portable/ emcc a.c
rm -f a.*

# Make emsdk usable by any user
chmod a+rxw -R /emsdk-portable

# node 8 is required to run wasm
# NOTE: Do not forget to sync Node.js version with `emscripten-entry.sh`!
cd /
curl --retry 5 -L https://nodejs.org/dist/v12.18.3/node-v12.18.3-linux-x64.tar.xz | \
    tar -xJ
