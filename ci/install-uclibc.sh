#!/usr/bin/bash
#
# Installs the appropriate uclibc toolchain into /toolchain

set -eux

time64="$1"

if [ "${time64:-0}" != "0" ]; then
    version='bleeding-edge-2025.08-1'
else
    version='bleeding-edge-2024.05-1'  # last version with 32-bit time_t
fi

mkdir /toolchain

curl --retry 5 -L "https://toolchains.bootlin.com/downloads/releases/toolchains/armv7-eabihf/tarballs/armv7-eabihf--uclibc--${version}.tar.xz" | \
tar xjf - -C /toolchain --strip-components=1

/toolchain/relocate-sdk.sh
