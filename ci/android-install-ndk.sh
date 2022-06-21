#!/usr/bin/env sh

set -ex

NDK=android-ndk-r23c
wget --tries=20 -q https://dl.google.com/android/repository/${NDK}-linux.zip
unzip -q ${NDK}-linux.zip

case "$1" in
  arm)
    arch=arm
    api=31
    ;;
  armv7)
    arch=arm
    api=31
    ;;
  aarch64)
    arch=arm64
    api=31
    ;;
  i686)
    arch=x86
    api=31
    ;;
  x86_64)
    arch=x86_64
    api=31
    ;;
  *)
    echo "invalid arch: $1"
    exit 1
    ;;
esac;

STANDALONE_NDK="/android/ndk-${1}"
python3 ${NDK}/build/tools/make_standalone_toolchain.py \
        --install-dir "${STANDALONE_NDK}" \
        --arch "${arch}" \
        --api ${api}

# FIXME: workaround of ndk23 https://github.com/rust-lang/rust/pull/85806#issuecomment-1096266946
find "${STANDALONE_NDK}" -name "libunwind.a" -exec sh -c \
        'printf "INPUT(-lunwind)" > $(dirname $1)/libgcc.a' shell {} \;

rm -rf ./${NDK}-linux-x86_64.zip ./${NDK}
