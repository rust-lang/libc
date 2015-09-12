#!/bin/sh

set -ex

TARGET=$1
cargo test --manifest-path libc-test/Cargo.toml --no-run --target $TARGET

if [ "$TARGET" = "arm-linux-androideabi" ]; then
    emulator @test -no-window &
    adb wait-for-device
    adb push /root/target/$TARGET/debug/all-* /data/test
    adb shell /data/test
else
    cargo test --manifest-path libc-test/Cargo.toml --target $TARGET
fi
