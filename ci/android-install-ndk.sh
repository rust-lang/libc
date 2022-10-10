#!/usr/bin/env sh

set -ex

NDK=android-ndk-r25b
wget --tries=20 -q https://dl.google.com/android/repository/${NDK}-linux.zip
unzip -q ${NDK}-linux.zip

case "$1" in
  arm)
    arch=arm
    api=28
    ;;
  armv7)
    arch=arm
    api=28
    ;;
  aarch64)
    arch=arm64
    api=28
    ;;
  i686)
    arch=x86
    api=28
    ;;
  x86_64)
    arch=x86_64
    api=28
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
find "/android/ndk-${1}" -name "libunwind.a" -exec sh -c \
        'printf "INPUT(-lunwind)" > $(dirname $1)/libgcc.a' shell {} \;

rm -rf ./${NDK}-linux.zip
