#!/usr/bin/env sh

# Checks that libc builds properly for all supported targets on a particular
# Rust version:

set -ex

RUST=${TRAVIS_RUST_VERSION}
OS=${TRAVIS_OS_NAME}

echo "Testing Rust ${RUST} on ${OS}"

test_target() {
    TARGET="${1}"

    opt=
    if [ "${TARGET}" = "x86_64-unknown-linux-gnux32" ]; then
        # FIXME: x86_64-unknown-linux-gnux32 fail to compile without
        # --release
        #
        # See https://github.com/rust-lang/rust/issues/45417
        opt="--release"
    fi

    NO_STD=
    case ${TARGET} in
        thumbv*)
            NO_STD=1
            ;;
    esac

    rustup target add "${TARGET}" --toolchain "${RUST}" || true

    # Test that libc builds without any default features (no libstd)
    cargo "+${RUST}" build -vv $opt --no-default-features --target "${TARGET}"

    # Test that libc builds with default features (e.g. libstd)
    # if the target supports libstd
    if [ "$NO_STD" != "1" ]; then
        cargo "+${RUST}" build -vv $opt --target "${TARGET}"
    fi

    # Test that libc builds with the `extra_traits` feature
    cargo "+${RUST}" build -vv $opt --no-default-features --target "${TARGET}" \
          --features extra_traits

    # Also test that it builds with `extra_traits` and default features:
    if [ "$NO_STD" != "1" ]; then
        cargo "+${RUST}" build -vv $opt --target "${TARGET}" \
              --features extra_traits
    fi
}

RUST_LINUX_TARGETS="\
aarch64-linux-android \
aarch64-unknown-linux-gnu \
arm-linux-androideabi \
arm-unknown-linux-gnueabi \
arm-unknown-linux-gnueabihf \
arm-unknown-linux-musleabi \
arm-unknown-linux-musleabihf \
armv7-linux-androideabi \
armv7-unknown-linux-gnueabihf \
i586-unknown-linux-gnu \
i686-linux-android \
i686-unknown-freebsd \
i686-unknown-linux-gnu \
i686-unknown-linux-musl \
mips-unknown-linux-gnu \
mips-unknown-linux-musl \
mips64-unknown-linux-gnuabi64 \
mips64el-unknown-linux-gnuabi64 \
mipsel-unknown-linux-gnu \
mipsel-unknown-linux-gnu \
mipsel-unknown-linux-musl \
powerpc-unknown-linux-gnu \
powerpc64-unknown-linux-gnu \
powerpc64le-unknown-linux-gnu \
s390x-unknown-linux-gnu \
x86_64-unknown-freebsd \
x86_64-unknown-linux-gnu \
x86_64-unknown-linux-musl \
x86_64-unknown-netbsd \
"

RUST_GT_1_13_LINUX_TARGETS="\
armv7-unknown-linux-musleabihf \
i586-unknown-linux-musl  \
sparc64-unknown-linux-gnu \
sparcv9-sun-solaris \
wasm32-unknown-emscripten \
wasm32-unknown-unknown \
x86_64-linux-android \
x86_64-rumprun-netbsd \
x86_64-sun-solaris \
x86_64-unknown-cloudabi \
"
RUST_GT_1_19_LINUX_TARGETS="\
aarch64-unknown-linux-musl \
"
RUST_GT_1_24_LINUX_TARGETS=

RUST_NIGHTLY_LINUX_TARGETS="\
aarch64-fuchsia \
thumbv6m-none-eabi \
thumbv7em-none-eabi \
thumbv7em-none-eabihf \
thumbv7m-none-eabi \
thumbv7neon-linux-androideabi \
thumbv7neon-unknown-linux-gnueabihf \
x86_64-fortanix-unknown-sgx \
x86_64-fuchsia \
x86_64-unknown-linux-gnux32 \
x86_64-unknown-redox \
"
# FIXME: these do not have a rust-std component available
# aarch64-unknown-cloudabi armv7-unknown-cloudabi-eabihf
# i686-unknown-cloudabi powerpc-unknown-linux-gnuspe
# sparc-unknown-linux-gnu mips-unknown-linux-uclib
# i686-unknown-haiku mipsel-unknown-unknown-linux-uclib
# sparc64-unknown-netbsd x86_64-unknown-bitrig x86_64-unknown-haiku
# x86_64-unknown-openbsd i686-unknown-netbsd

RUST_OSX_TARGETS="\
aarch64-apple-ios \
armv7-apple-ios \
armv7s-apple-ios \
i386-apple-ios \
i686-apple-darwin \
x86_64-apple-darwin \
x86_64-apple-ios \
"

# The targets are listed here alphabetically
TARGETS=""
case "${OS}" in
    linux*)
        TARGETS="${RUST_LINUX_TARGETS}"

        if [ "${RUST}" != "1.13.0" ]; then
            TARGETS="${TARGETS} ${RUST_GT_1_13_LINUX_TARGETS}"
            if [ "${RUST}" != "1.19.0" ]; then
                TARGETS="${TARGETS} ${RUST_GT_1_19_LINUX_TARGETS}"
                if [ "${RUST}" != "1.24.0" ]; then
                    TARGETS="${TARGETS} ${RUST_GT_1_24_LINUX_TARGETS}"
                fi
            fi
        fi

        if [ "${RUST}" = "nightly" ]; then
            TARGETS="${TARGETS} ${RUST_NIGHTLY_LINUX_TARGETS}"
        fi

        ;;
    osx*)
        TARGETS="${RUST_OSX_TARGETS}"
        ;;
    *)
        ;;
esac

for TARGET in $TARGETS; do
    test_target "$TARGET"
done
