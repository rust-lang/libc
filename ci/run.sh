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

case "$target" in
    # crash in std::env::tmp_dir (no filesystem on wasm).
    *wasm*) cmd="$cmd --exclude ctest --exclude ctest-test --exclude ctest-next" ;;
    # Loongarch was fixed, but there are new instances of
    # https://github.com/bytecodealliance/rustix/issues/1496
    powerpc64le*musl) cmd="$cmd --exclude ctest --exclude ctest-test --exclude ctest-next" ;;
esac

env="$(rustc --print cfg --target "$target" | sed -n 's/target_env="\(.*\)"/\1/p')"
bits="$(rustc --print cfg --target "$target" | sed -n 's/target_pointer_width="\(.*\)"/\1/p')"

# shellcheck disable=SC2086
$cmd --no-default-features -- $test_flags
