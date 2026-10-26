#!/bin/bash
#
# Builds a buildroot uclibc toolchain into /buildroot/output/host/
#
# usage: install-uclibc.sh BUILDROOT_TOOLCHAIN_URL

set -eux

toolchain_url="$1"

mkdir /toolchain
curl --retry 5 -L "$toolchain_url" | tar xzf - -C /toolchain --strip-components=1
if ! [ -f /toolchain/relocate-sdk.sh ]; then
    echo "ERROR: toolchain does not contain relocate-sdk.sh, expecting a buildroot SDK."
    exit 1
fi
/toolchain/relocate-sdk.sh

