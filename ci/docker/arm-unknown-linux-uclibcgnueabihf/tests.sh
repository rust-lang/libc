#!/usr/bin/env bash
set -e

PREFIX=/x-tools/arm-unknown-linux-uclibcgnueabihf/bin/arm-unknown-linux-uclibcgnueabihf
RUST_TARGET=arm-unknown-linux-uclibcgnueabihf

export AR_arm_unknown_linux_uclibcgnueabihf=${PREFIX}-ar
export CC_arm_unknown_linux_uclibcgnueabihf=${PREFIX}-gcc

echo -ne "[target.${RUST_TARGET}]\nlinker = \"${PREFIX}-gcc\"\n" > /root/.cargo/config
echo -ne "[target.${RUST_TARGET}.dependencies.std]\nfeatures = [\"jemalloc\"]\n" > Xargo.toml

rustup run nightly xargo build --manifest-path libc-test/Cargo.toml --target ${RUST_TARGET}
