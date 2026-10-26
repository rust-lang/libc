#!/bin/sh
# Prepare BSD environments in vmactions containers, which don't have many tools
# installed by default.

set -eux

target="$1"

uname -a
case "$(uname -s)" in
    FreeBSD) deps="pkg install -y libnghttp2 curl" ;;
    NetBSD) deps="/usr/sbin/pkg_add curl" ;;
    # Solaris and Illumos already have curl installed
    *) deps="true" ;;
esac

# Installs have been flaky so give them a retry
count=0
success=false
while [ $count -lt 3 ]; do
    $deps && success=true || true
    [ $success = true ] && break
    sleep 3
    count=$(( count + 1))
done

if [ "$success" != true ]; then
    echo "failed to install dependencies"
    exit 1
fi

if [ "$target" = x86_64-pc-solaris ]; then
    # rustup 1.29.2 uses File::lock, which is unsupported on Solaris since Rust 1.98.
    # https://github.com/rust-lang/rust/pull/157509
    # Remove this pin once rustup supports Solaris file locking again.
    export RUSTUP_VERSION=1.29.1
fi

curl --proto '=https' --tlsv1.2 -sSf --retry 5 https://sh.rustup.rs | sh -s -- \
    --profile minimal \
    --default-toolchain nightly \
    --target "$target" \
    -y
