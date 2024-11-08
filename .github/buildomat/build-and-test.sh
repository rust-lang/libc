#!/bin/env bash

set -o errexit
set -o pipefail
set -o xtrace

export TARGET="$1"

# Enable ANSI colors in Cargo output.
export CARGO_TERM_COLOR=always

banner install
# Note: we use ci/install-rust.sh rather than buildomat to install Rust, for
# consistency with other CI jobs.
TOOLCHAIN=stable INSTALL_RUSTUP=1 ptime -m sh ci/install-rust.sh

banner run.sh
ptime -m sh ci/run.sh "$TARGET"
