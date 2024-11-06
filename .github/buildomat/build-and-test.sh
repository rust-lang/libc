#!/bin/env bash

set -o errexit
set -o pipefail
set -o xtrace

# Enable ANSI colors in Cargo output.
export CARGO_TERM_COLOR=always

cargo --version
rustc --version

banner libc
ptime -m cargo build --verbose

banner test
ptime -m cargo test --verbose

banner libctest
ptime -m cargo test --verbose --manifest-path libc-test/Cargo.toml
