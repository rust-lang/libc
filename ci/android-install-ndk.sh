#!/usr/bin/env sh

set -ex

NDK=android-ndk-r21d
wget --tries=20 -q https://dl.google.com/android/repository/${NDK}-linux-x86_64.zip
unzip -q ${NDK}-linux-x86_64.zip

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

python3 ${NDK}/build/tools/make_standalone_toolchain.py \
        --install-dir "/android/ndk-${1}" \
        --arch "${arch}" \
        --api ${api}

rm -rf ./${NDK}-linux-x86_64.zip ./${NDK}
