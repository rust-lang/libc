#!/usr/bin/env sh

set -ex

NDK=android-ndk-r25b
wget --tries=20 -q https://dl.google.com/android/repository/${NDK}-linux.zip
unzip -q ${NDK}-linux.zip

mv ${NDK} "/android/ndk-${1}"

# FIXME: workaround of ndk23 https://github.com/rust-lang/rust/pull/85806#issuecomment-1096266946
find "/android/ndk-${1}" -name "libunwind.a" -exec sh -c \
        'printf "INPUT(-lunwind)" > $(dirname $1)/libgcc.a' shell {} \;

rm -rf ./${NDK}-linux.zip
