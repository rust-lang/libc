#!/usr/bin/env sh

set -ex

rustc ci/style.rs && ./style src

if rustup component add rustfmt-preview ; then
    command -v rustfmt
    rustfmt -V
    cargo fmt --all -- --check
fi

if shellcheck --version ; then
    # GHA's shellcheck is too old (0.4.6) and cannot handle SC2153 correctly.
    shellcheck -e SC2103 -e SC2153 ci/*.sh
else
    echo "shellcheck not found"
    exit 1
fi

