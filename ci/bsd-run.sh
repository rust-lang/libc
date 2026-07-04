#!/bin/sh

set -eux

target="$1"

# shellcheck source=/dev/null
. "$HOME/.cargo/env"

# System info
uname -a
which rustc
rustc -Vv

./ci/run.sh "$target"
