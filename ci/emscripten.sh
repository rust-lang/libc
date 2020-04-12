#!/usr/bin/env bash

set -ex

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
# FIXME: switch to an upstream install once
# https://github.com/rust-lang/rust/pull/63649 lands
hide_output ./emsdk install 1.39.12
./emsdk activate 1.39.12

# Compile and cache libc
# shellcheck disable=SC1091
source ./emsdk_env.sh
echo "main(){}" > a.c
HOME=/emsdk-portable/ emcc a.c
rm -f a.*

# Make emsdk usable by any user
cp /root/.emscripten /emsdk-portable
chmod a+rxw -R /emsdk-portable

# node 8 is required to run wasm
cd /
curl --retry 5 -L https://nodejs.org/dist/v12.16.2/node-v12.16.2-linux-x64.tar.xz | \
    tar -xJ
