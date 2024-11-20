#!/usr/bin/env sh

set -eux

ndk=android-ndk-r27
wget --tries=20 -q "https://dl.google.com/android/repository/${ndk}-linux.zip"
unzip -q "${ndk}-linux.zip"

mv "./${ndk}/toolchains/llvm/prebuilt/linux-x86_64" /android

rm -rf "./${ndk}-linux.zip" "./${ndk}"
