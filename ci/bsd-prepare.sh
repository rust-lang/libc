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
    sleep 3s
    count=$(( count + 1))
done

if [ "$success" != true ]; then
    echo "failed to install dependencies"
    exit 1
fi

curl --proto '=https' --tlsv1.2 -sSf --retry 5 https://sh.rustup.rs | sh -s -- \
    --profile minimal \
    --default-toolchain nightly \
    --target "$target" \
    -y
