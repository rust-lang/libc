#!/bin/sh

set -ex

TARGET=$1
cargo build --manifest-path libc-test/Cargo.toml --target $TARGET

if [ "$TARGET" = "arm-linux-androideabi" ]; then
    emulator @test -no-window &
    adb wait-for-device
    adb push /root/target/$TARGET/debug/libc-test /data/libc-test
    adb shell /data/libc-test
elif [ "$TARGET" = "arm-unknown-linux-gnueabihf" ]; then
    qemu-arm -L /usr/arm-linux-gnueabihf libc-test/target/$TARGET/debug/libc-test
elif [ "$TARGET" = "mips-unknown-linux-gnu" ]; then
    # FIXME: this segfaults on travis, passes locally?
    #qemu-mips -L /usr/mips-linux-gnu libc-test/target/$TARGET/debug/all-*
    echo skip
elif [ "$TARGET" = "aarch64-unknown-linux-gnu" ]; then
    qemu-aarch64 -L /usr/aarch64-linux-gnu/ libc-test/target/$TARGET/debug/libc-test
else
    cargo run --manifest-path libc-test/Cargo.toml --target $TARGET
fi
