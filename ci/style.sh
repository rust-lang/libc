#!/usr/bin/env sh

set -ex

rustc ci/style.rs && ./style src

rustup toolchain install nightly -c rustfmt --allow-downgrade
rustup override set nightly
command -v rustfmt
rustfmt -V
cargo fmt --all -- --check

if shellcheck --version ; then
    # GHA's shellcheck is too old (0.4.6) and cannot handle SC2153 correctly.
    shellcheck -e SC2103 -e SC2153 ci/*.sh
else
    echo "shellcheck not found"
    exit 1
fi

