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

    # Test that libc builds with the `extra_traits` feature if Rust >= 1.25.0
    if [ "${RUST}" != "+1.13.0" ] && \
           [ "${RUST}" != "+1.19.0" ] && \
           [ "${RUST}" != "+1.24.0" ]; then
        cargo "+${RUST}" build -vv $opt --no-default-features --target "${TARGET}" \
              --features extra_traits

        # Also test that it builds with `extra_traits` and default features:
        if [ "$NO_STD" != "1" ]; then
            cargo "+${RUST}" build -vv $opt --target "${TARGET}" \
                  --features extra_traits
        fi
    fi
}

TARGETS="wasm32-unknown-unknown"
case "${OS}" in
    linux*)
        TARGETS="i686-unknown-linux-gnu x86_64-unknown-linux-gnu x86_64-unknown-freebsd x86_64-unknown-netbsd "

        if [ "${RUST}" = "nightly" ]; then
            # aarch64-unknown-cloudabi armv7-unknown-cloudabi-eabihf i686-unknown-cloudabi powerpc-unknown-linux-gnuspe sparc-unknown-linux-gnu mips-unknown-unknown-linux-uclib i686-unknown-haiku i686-unknown-netbsd mipsel-unknown-unknown-linux-uclib sparc64-unknown-netbsd x86_64-unknown-bitrig x86_64-unknown-haiku x86_64-unknown-openbsd
            TARGETS="${TARGETS} aarch64-fuchsia armv7-linux-androideabi armv7-unknown-linux-gnueabihf armv7-unknown-linux-musleabihf i586-unknown-linux-gnu i586-unknown-linux-musl i686-linux-android i686-unknown-freebsd mipsel-unknown-linux-gnu x86_64-fuchsia x86_64-rumprun-netbsd x86_64-sun-solaris x86_64-unknown-cloudabi x86_64-unknown-redox thumbv6m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf thumbv7m-none-eabi x86_64-fortanix-unknown-sgx"
        fi

        ;;
    osx*)
        TARGETS="i686-apple-darwin x86_64-apple-darwin"

        if [ "${RUST}" = "nightly" ]; then
            TARGETS="${TARGETS} aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios"
        fi

        ;;
    *)
        ;;
esac

for TARGET in $TARGETS; do
    test_target "$TARGET"
done
