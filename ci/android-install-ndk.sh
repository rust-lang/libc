#!/usr/bin/env sh

set -ex

NDK=android-ndk-r26b
wget --tries=20 -q https://dl.google.com/android/repository/${NDK}-linux.zip
unzip -q ${NDK}-linux.zip

mv ./${NDK}/toolchains/llvm/prebuilt/linux-x86_64 /android

rm -rf ./${NDK}-linux.zip ./${NDK}
