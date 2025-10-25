#!/usr/bin/env sh

# Builds and runs tests for a particular target passed as an argument to this
# script.

set -eux

target="$1"

export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"

# For logging
uname -a

cmd="cargo test --target $target ${LIBC_CI_ZBUILD_STD+"-Zbuild-std"}"
test_flags="--skip check_style"

# Run tests in the `libc` crate
case "$target" in
    # Only run `libc-test`
    # FIXME(android): unit tests fail to start on Android
    *android*) cmd="$cmd --manifest-path libc-test/Cargo.toml" ;;
    *s390x*) cmd="$cmd --manifest-path libc-test/Cargo.toml" ;;
    # For all other platforms, test everything in the workspace
    *) cmd="$cmd --workspace" ;;
esac

# shellcheck disable=SC2086
$cmd --no-default-features -- $test_flags
# shellcheck disable=SC2086
$cmd -- $test_flags
# shellcheck disable=SC2086
$cmd --features extra_traits -- $test_flags
