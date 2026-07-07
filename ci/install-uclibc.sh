#!/bin/bash
#
# Builds a buildroot uclibc toolchain into /buildroot/output/host/
#
# usage: install-uclibc.sh BUILDROOT_DEFCONFIG USE_TIME64

set -eux

defconfig="$1"
# shellcheck disable=SC2034
time64="$2"  # TODO: honor this flag, adjusting UCLIBC_USE_TIME64 in config

mkdir /buildroot
BR_URL=https://buildroot.org/downloads/buildroot-2026.05.tar.xz
curl --retry 5 -L "$BR_URL" | tar xJf - -C /buildroot --strip-components=1

cd /buildroot
make defconfig "BR2_DEFCONFIG=$defconfig"
make
