#!/usr/bin/env sh

# Checks that libc builds properly for all supported targets on a particular
# Rust version:
# The FILTER environment variable can be used to select which target(s) to build.
# For example: set FILTER to vxworks to select the targets that has vxworks in name

set -eux

: "${TOOLCHAIN?The TOOLCHAIN environment variable must be set.}"

rust="$TOOLCHAIN"
filter="${FILTER:-}"
host_target=$(rustc -vV | awk '/^host/ { print $2 }')

case "$(uname -s)" in
    Linux*) os=linux ;;
    Darwin*) os=macos ;;
    MINGW*) os=windows ;;
    *)
        echo "Unknown system $(uname -s)"
        exit 1
        ;;
esac

echo "Testing Rust $rust on $os"

if [ "$TOOLCHAIN" = "nightly" ]; then
    # For build-std
    rustup component add rust-src
fi

# Print GHA workflow commands
echo_if_ci() {
    # Discard stderr so the "set -x" trace doesn't show up
    { [ -n "${CI:-}" ] && echo "$1"; } 2> /dev/null
}

# Run the tests for a specific target
test_target() {
    target="$1"
    no_dist="$2"

    RUSTFLAGS="${RUSTFLAGS:-}"

    # The basic command that is run each time
    cmd="cargo +$rust build --target $target"

    if [ "${no_dist}" != "0" ]; then
        # If we can't download a `core`, we need to build it
        cmd="$cmd -Zbuild-std=core"

        # FIXME: With `build-std` feature, `compiler_builtins` emits a lof of lint warnings.
        RUSTFLAGS="${RUSTFLAGS:-} -Aimproper_ctypes_definitions"
        export RUSTFLAGS
    else
        # Otherwise it is available for download; fetch it:

        # FIXME: rustup often fails to download some artifacts due to network
        # issues, so we retry this N times.
        N=5
        n=0
        until [ $n -ge $N ]; do
            if rustup target add "$target" --toolchain "$rust"; then
                break
            fi
            n=$((n + 1))
            sleep 1
        done
    fi

    # Test with expected combinations of features
    $cmd
    $cmd --features extra_traits

    if [ "$os" = "linux" ]; then
        # Test with the equivalent of __USE_TIME_BITS64
        RUST_LIBC_UNSTABLE_LINUX_TIME_BITS64=1 $cmd
        case "$target" in
            arm*-gnu* | i*86*-gnu | powerpc-*-gnu* | mips*-gnu | sparc-*-gnu | thumb-*gnu*)
                # Test with the equivalent of _FILE_OFFSET_BITS=64
                RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS=64 $cmd
                # Test with the equivalent of _TIME_BITS=64
                RUST_LIBC_UNSTABLE_GNU_TIME_BITS=64 $cmd
                ;;
        esac
    fi

    # Test again without default features, i.e. without "std"
    $cmd --no-default-features
    $cmd --no-default-features --features extra_traits

    # Ensure the crate will build when used with `std`
    if [ "$rust" = "nightly" ]; then
        $cmd --no-default-features --features rustc-dep-of-std
    fi

    # For tier 2 freebsd targets, check with the different versions we support
    # if on nightly or stable
    case "$rust-$target" in
        stable-x86_64-*freebsd*) do_freebsd_checks=1 ;;
        nightly-i686*freebsd*) do_freebsd_checks=1 ;;
    esac

    if [ -n "${do_freebsd_checks:-}" ]; then
        for version in $freebsd_versions; do
            export RUST_LIBC_UNSTABLE_FREEBSD_VERSION="$version"
            $cmd
            $cmd --no-default-features
        done
    fi

    # FIXME(semver): can't pass `--target` to `cargo-semver-checks`
    if [ "$rust" = "stable" ] && [ "$target" = "$host_target" ]; then
        # Run semver checks on the stable channel
        cargo semver-checks --only-explicit-features \
            --features std,extra_traits
    fi
}

freebsd_versions="\
11 \
12 \
13 \
14 \
15 \
"

rust_linux_targets="\
aarch64-linux-android \
aarch64-unknown-linux-gnu \
aarch64-unknown-linux-musl \
arm-linux-androideabi \
arm-unknown-linux-gnueabi \
arm-unknown-linux-gnueabihf \
arm-unknown-linux-musleabi \
arm-unknown-linux-musleabihf \
armv7-linux-androideabi \
armv7-unknown-linux-gnueabihf \
armv7-unknown-linux-musleabihf \
i586-unknown-linux-gnu \
i586-unknown-linux-musl \
i686-linux-android \
i686-unknown-freebsd \
i686-unknown-linux-gnu \
i686-unknown-linux-musl \
powerpc-unknown-linux-gnu \
powerpc64-unknown-linux-gnu \
powerpc64le-unknown-linux-gnu \
s390x-unknown-linux-gnu \
sparc64-unknown-linux-gnu \
sparcv9-sun-solaris \
wasm32-unknown-emscripten \
wasm32-unknown-unknown \
wasm32-wasip1 \
wasm32-wasip2 \
x86_64-linux-android \
x86_64-unknown-freebsd \
x86_64-unknown-linux-gnu \
x86_64-unknown-linux-musl \
x86_64-unknown-netbsd \
"

# FIXME(powerpc64le): powerpc64le-unknown-linux-musl is tier 2 since 1.85 and
# can be moved to rust_linux_targets once MSRV is increased
rust_nightly_linux_targets="\
aarch64-unknown-fuchsia \
armv5te-unknown-linux-gnueabi \
armv5te-unknown-linux-musleabi \
i686-pc-windows-gnu \
powerpc64le-unknown-linux-musl \
riscv64gc-unknown-linux-gnu \
x86_64-fortanix-unknown-sgx \
x86_64-pc-solaris \
x86_64-pc-windows-gnu \
x86_64-unknown-fuchsia \
x86_64-unknown-illumos \
x86_64-unknown-linux-gnux32 \
x86_64-unknown-redox \
"

rust_apple_targets="\
aarch64-apple-darwin \
aarch64-apple-ios \
"

rust_nightly_apple_targets="\
"

# Must start with `x86_64-pc-windows-msvc` first.
rust_nightly_windows_targets="\
x86_64-pc-windows-msvc \
x86_64-pc-windows-gnu \
i686-pc-windows-msvc \
"

# Targets which are not available via rustup and must be built with -Zbuild-std
# FIXME(hexagon): hexagon-unknown-linux-musl should be tested but currently has
# duplicate symbol errors from `compiler_builtins`.
rust_linux_no_dist_targets="\
aarch64-pc-windows-msvc \
aarch64-unknown-freebsd \
aarch64-unknown-hermit \
aarch64-unknown-netbsd \
aarch64-unknown-openbsd \
aarch64-wrs-vxworks \
armebv7r-none-eabi \
armebv7r-none-eabihf \
armv7-wrs-vxworks-eabihf \
armv7r-none-eabi \
armv7r-none-eabihf \
i686-pc-windows-msvc \
i686-unknown-haiku \
i686-unknown-netbsd \
i686-unknown-openbsd \
i686-wrs-vxworks \
mips-unknown-linux-gnu \
mips-unknown-linux-musl \
mips64-unknown-linux-gnuabi64 \
mips64-unknown-linux-muslabi64 \
mips64el-unknown-linux-gnuabi64 \
mips64el-unknown-linux-muslabi64 \
mipsel-unknown-linux-gnu \
mipsel-unknown-linux-musl \
nvptx64-nvidia-cuda \
powerpc-unknown-linux-gnuspe \
powerpc-unknown-netbsd \
powerpc-wrs-vxworks \
powerpc-wrs-vxworks-spe \
powerpc64-unknown-freebsd \
powerpc64-wrs-vxworks \
riscv32i-unknown-none-elf \
riscv32imac-unknown-none-elf \
riscv32imc-unknown-none-elf \
riscv32gc-unknown-linux-gnu \
riscv32-wrs-vxworks \
riscv64gc-unknown-freebsd \
riscv64gc-unknown-hermit \
riscv64gc-unknown-linux-musl \
riscv64gc-unknown-none-elf \
riscv64imac-unknown-none-elf \
riscv64-wrs-vxworks \
s390x-unknown-linux-musl \
sparc-unknown-linux-gnu \
sparc64-unknown-netbsd \

thumbv6m-none-eabi \
thumbv7em-none-eabi \
thumbv7em-none-eabihf \
thumbv7m-none-eabi \
thumbv7neon-linux-androideabi \
thumbv7neon-unknown-linux-gnueabihf \
thumbv8m.main-none-eabi \
x86_64-pc-windows-msvc \
x86_64-unknown-dragonfly \
x86_64-unknown-haiku \
x86_64-unknown-hermit \
x86_64-unknown-l4re-uclibc \
x86_64-unknown-openbsd \
x86_64-wrs-vxworks \
"

rust_apple_no_dist_targets="\
armv7s-apple-ios \
i686-apple-darwin \
i386-apple-ios \
"

# The targets are listed here alphabetically
if [ "$os" = "linux" ]; then
    targets="$rust_linux_targets"
    nightly_targets="$rust_nightly_linux_targets"
    no_dist_targets="$rust_linux_no_dist_targets"
elif [ "$os" = "macos" ]; then
    targets="$rust_apple_targets"
    nightly_targets="$rust_nightly_apple_targets"
    no_dist_targets="$rust_apple_no_dist_targets"
elif [ "$os" = "windows" ]; then
    targets=${rust_nightly_windows_targets}
else
    exit 1
fi

if [ "$rust" = "nightly" ]; then
    targets="$targets ${nightly_targets:-}"
else
    # build-std requires nightly
    no_dist_targets=""
fi

case "$rust" in
    "stable") supports_wasi_pn=1 ;;
    "beta") supports_wasi_pn=1 ;;
    "nightly") supports_wasi_pn=1 ;;
    *) supports_wasi_pn=0 ;;
esac

some_tests_run=0

# Apply the `FILTER` variable, do OS-specific tasks, and run a target
filter_and_run() {
    target="$1"
    no_dist="${2:-0}"

    if echo "$target" | grep -q "$filter"; then
        if [ "$os" = "windows" ]; then
            TARGET="$target" ./ci/install-rust.sh
        fi

        # `wasm32-wasip1` was renamed from `wasm32-wasi`
        if [ "$target" = "wasm32-wasip1" ] && [ "$supports_wasi_pn" = "0" ]; then
            target="wasm32-wasi"
        fi

        # `wasm32-wasip2` only exists in recent versions of Rust
        if [ "$target" = "wasm32-wasip2" ] && [ "$supports_wasi_pn" = "0" ]; then
            return
        fi

        test_target "$target" "$no_dist"
        some_tests_run=1
    fi
}

for target in $targets; do
    echo_if_ci "::group::Target: $target"
    filter_and_run "$target"
    echo_if_ci "::endgroup::"
done

for target in ${no_dist_targets:-}; do
    echo_if_ci "::group::Target: $target"
    filter_and_run "$target" 1
    echo_if_ci "::endgroup::"
done

# Make sure we didn't accidentally filter everything
if [ "$some_tests_run" != 1 ]; then
    echo "No tests were run"
    exit 1
fi
