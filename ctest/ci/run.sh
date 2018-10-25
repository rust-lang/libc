#!/bin/sh

# Builds and runs tests for a particular target passed as an argument to this
# script.

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

mkdir -p target
git clone https://github.com/rust-lang/libc target/libc
mkdir -p target/libc/target/ctest
sed -i 's@ctest = "0.2.3"@ctest = { path = "../../.." }@g' target/libc/libc-test/Cargo.toml

cargo test --manifest-path target/libc/libc-test/Cargo.toml --target $TARGET
