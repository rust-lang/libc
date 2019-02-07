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

TARGETS="wasm32-unknown-unknown"
case "${OS}" in
    linux*)
        TARGETS="i686-unknown-linux-gnu x86_64-unknown-linux-gnu x86_64-unknown-freebsd x86_64-unknown-netbsd i586-unknown-linux-gnu i686-linux-android i686-unknown-freebsd mipsel-unknown-linux-gnu mips64-unknown-linux-gnuabi64 mips64el-unknown-linux-gnuabi64 mipsel-unknown-linux-gnu mipsel-unknown-linux-musl armv7-linux-androideabi armv7-unknown-linux-gnueabihf  aarch64-linux-android powerpc-unknown-linux-gnu powerpc64-unknown-linux-gnu powerpc64le-unknown-linux-gnu s390x-unknown-linux-gnu sparcv9-sun-solaris x86_64-unknown-linux-musl x86_64-linux-android wasm32-unknown-emscripten"

        if [ "${RUST}" != "1.13.0" ] && \
               [ "${RUST}" != "1.19.0" ] && \
               [ "${RUST}" != "1.24.0" ]; then
            TARGETS="${TARGETS} i586-unknown-linux-musl armv7-unknown-linux-musleabihf x86_64-sun-solaris sparc64-unknown-linux-gnu"
        fi

        if [ "${RUST}" = "nightly" ]; then
            # aarch64-unknown-cloudabi armv7-unknown-cloudabi-eabihf
            # i686-unknown-cloudabi powerpc-unknown-linux-gnuspe
            # sparc-unknown-linux-gnu mips-unknown-unknown-linux-uclib
            # i686-unknown-haiku mipsel-unknown-unknown-linux-uclib
            # sparc64-unknown-netbsd x86_64-unknown-bitrig x86_64-unknown-haiku
            # x86_64-unknown-openbsd i686-unknown-netbsd
            TARGETS="${TARGETS} aarch64-fuchsia  x86_64-fuchsia x86_64-rumprun-netbsd x86_64-unknown-cloudabi x86_64-unknown-redox thumbv6m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf thumbv7m-none-eabi x86_64-fortanix-unknown-sgx"
        fi

        ;;
    osx*)
        TARGETS="i686-apple-darwin x86_64-apple-darwin aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios"
        ;;
    *)
        ;;
esac

for TARGET in $TARGETS; do
    test_target "$TARGET"
done
