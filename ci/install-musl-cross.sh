#!/bin/sh
#
# Install musl cross toolchain

set -ex

MUSL_CROSS_VER=20241103
MUSL_CROSS_URL=https://github.com/musl-cross/musl-cross/releases/download/$MUSL_CROSS_VER/$1.tar.xz

curl -L --retry 5 "$MUSL_CROSS_URL" | tar -xJf - -C /
