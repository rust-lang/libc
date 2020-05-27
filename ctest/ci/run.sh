#!/usr/bin/env sh

# Builds and runs tests for a particular target passed as an argument to this
# script.

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

mkdir -p target
rm -rf target/libc || true
git clone --depth=1 https://github.com/rust-lang/libc target/libc
mkdir -p target/libc/target/ctest2

case $TARGET in
    *linux*)
        sed -i 's@ctest2 = "0.2"@ctest2 = { path = "../../.." }@g' target/libc/libc-test/Cargo.toml
        ;;
    *apple*)
        sed -i '' 's@ctest2 = "0.2"@ctest2 = { path = "../../.." }@g' target/libc/libc-test/Cargo.toml
        ;;
esac

cargo test --release --manifest-path target/libc/libc-test/Cargo.toml --target $TARGET
