#!/usr/bin/env sh

# Checks that libc builds properly for all supported targets on a particular
# Rust version:
# The FILTER environment variable can be used to select which target(s) to build.
# For example: set FILTER to vxworks to select the targets that has vxworks in name

set -eux

: "${TOOLCHAIN?The TOOLCHAIN environment variable must be set.}"
: "${OS?The OS environment variable must be set.}"

rust="$TOOLCHAIN"
filter="${FILTER:-}"

echo "Testing Rust $rust on $OS"

if [ "$TOOLCHAIN" = "nightly" ] ; then
    rustup component add rust-src
fi

test_target() {
    build_cmd="${1}"
    target="${2}"
    no_std="${3:-}"

    RUSTFLAGS="${RUSTFLAGS:-}"

    # If there is a std component, fetch it:
    if [ "${no_std}" != "1" ]; then
        # FIXME: rustup often fails to download some artifacts due to network
        # issues, so we retry this N times.
        N=5
        n=0
        until [ $n -ge $N ]; do
            if rustup target add "$target" --toolchain "$rust" ; then
                break
            fi
            n=$((n+1))
            sleep 1
        done

        # FIXME: With `build-std` feature, `compiler_builtins` emits a lof of lint warnings.
        RUSTFLAGS="${RUSTFLAGS:-} -Aimproper_ctypes_definitions"
        export RUSTFLAGS
    fi

    # Test that libc builds without any default features (no std)
    if [ "$no_std" != "1" ]; then
        cargo "+$rust" "$build_cmd" --no-default-features --target "$target"
    else
        cargo "+$rust" "$build_cmd" \
            -Z build-std=core,alloc \
            --no-default-features \
            --target "$target"
    fi

    # Test that libc builds with default features (e.g. std)
    # if the target supports std
    if [ "$no_std" != "1" ]; then
        cargo "+$rust" "$build_cmd" --target "$target"
    else
        cargo "+$rust" "${build_cmd}" \
            -Z build-std=core,alloc \
            --target "$target"
    fi

    # Test that libc builds with the `extra_traits` feature
    if [ "$no_std" != "1" ]; then
        cargo "+$rust" "$build_cmd" \
            --no-default-features \
            --features extra_traits \
            --target "$target"
    else
        cargo "+$rust" "$build_cmd" \
            -Z build-std=core,alloc \
            --no-default-features \
            --features extra_traits \
            --target "$target"
    fi

    # Test the 'const-extern-fn' feature on nightly
    if [ "${rust}" = "nightly" ]; then
        if [ "${no_std}" != "1" ]; then
            cargo "+$rust" "$build_cmd" \
                --no-default-features \
                --features const-extern-fn \
                --target "$target"
        else
            cargo "+$rust" "$build_cmd" \
                -Z build-std=core,alloc \
                --no-default-features \
                --features const-extern-fn \
                --target "$target"
        fi
    fi

    # Also test that it builds with `extra_traits` and default features:
    if [ "$no_std" != "1" ]; then
        cargo "+$rust" "$build_cmd" \
            --target "$target" \
            --features extra_traits
    else
        cargo "+$rust" "$build_cmd" \
            -Z build-std=core,alloc \
            --target "$target" \
            --features extra_traits
    fi
}

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
x86_64-linux-android \
x86_64-unknown-freebsd \
x86_64-unknown-linux-gnu \
x86_64-unknown-linux-musl \
x86_64-unknown-netbsd \
"

rust_nightly_linux_targets="\
aarch64-unknown-fuchsia \
armv5te-unknown-linux-gnueabi \
armv5te-unknown-linux-musleabi \
i686-pc-windows-gnu \
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
x86_64-apple-darwin \
x86_64-apple-ios \
"

rust_nightly_apple_targets="\
"

# Must start with `x86_64-pc-windows-msvc` first.
rust_nightly_windows_targets="\
x86_64-pc-windows-msvc \
x86_64-pc-windows-gnu \
i686-pc-windows-msvc \
"

# The targets are listed here alphabetically
targets=""
case "${OS}" in
    linux*)
        targets="$rust_linux_targets"

        if [ "$rust" = "nightly" ]; then
            targets="$targets $rust_nightly_linux_targets"
        fi

        ;;
    macos*)
        targets="$rust_apple_targets"

        if [ "$rust" = "nightly" ]; then
            targets="$targets $rust_nightly_apple_targets"
        fi

        ;;
    windows*)
        targets=${rust_nightly_windows_targets}
        ;;
    *) ;;
esac

for target in $targets; do
    if echo "$target" | grep -q "$filter"; then
        if [ "${OS}" = "windows" ]; then
            TARGET="$target" ./ci/install-rust.sh
            test_target build "$target"
        else
            test_target build "$target"
        fi

        test_run=1
    fi
done

# Targets which are not available via rustup and must be built with -Zbuild-std
# FIXME(hexagon): hexagon-unknown-linux-musl should be tested but currently has
# duplicate symbol errors from `compiler_builtins`.
rust_linux_no_core_targets="\
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
i586-pc-windows-msvc \
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
mipsel-sony-psp \
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

if [ "${rust}" = "nightly" ] && [ "${OS}" = "linux" ]; then
    for target in $rust_linux_no_core_targets; do
        if echo "$target" | grep -q "$FILTER"; then
            test_target "$target" 1
        fi

        test_run=1
    done
fi

rust_apple_no_core_targets="\
armv7s-apple-ios \
i686-apple-darwin \
i386-apple-ios \
"

if [ "${rust}" = "nightly" ] && [ "${OS}" = "macos" ]; then
    for target in $rust_apple_no_core_targets; do
        if echo "$target" | grep -q "$FILTER"; then
            test_target "$target" 1
        fi

        test_run=1
    done
fi

# Make sure we didn't accidentally filter everything
if [ "${test_run:-}" != 1 ]; then
    echo "No tests were run"
    exit 1
fi
