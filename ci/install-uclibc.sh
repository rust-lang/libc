#!/bin/bash
#
# Builds a buildroot uclibc toolchain into /buildroot/output/host/
#
# usage: install-uclibc.sh BUILDROOT_DEFCONFIG USE_TIME64

set -eux

defconfig="$1"
time64="$2"  # TODO: honor this flag, adjusting UCLIBC_USE_TIME64 in config

mkdir /toolchain
URL=https://github.com/skrap/libc-downstream-ci/releases/download/v0.1-toolchain/arm-buildroot-linux-uclibcgnueabihf_sdk-buildroot.tar.gz
curl --retry 5 -L "$URL" | tar xzf - -C /toolchain --strip-components=1
/toolchain/relocate-sdk.sh
