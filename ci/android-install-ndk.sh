#!/usr/bin/env sh

set -ex

NDK=android-ndk-r25b
wget --tries=20 -q https://dl.google.com/android/repository/${NDK}-linux.zip
unzip -q ${NDK}-linux.zip
mv ${NDK} "/android/ndk-${1}"

rm -rf ./${NDK}-linux.zip
