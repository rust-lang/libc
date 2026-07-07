#!/usr/bin/env bash

set -eux

# Extract the bionic linker and libraries from a current Android build into
# /system, so the test binaries can run under qemu-user with no Android
# system booted (see the aarch64-linux-android Dockerfile). This is a
# stopgap until GitHub's arm64 runners expose KVM, at which point this
# target can move to Cuttlefish like x86_64-linux-android.
#
# The target_files zip is used because it contains the system partition as
# plain files, unlike the flashable image zips.
#
# The pinned build (currently Android 17) matches ci/cuttlefish-setup.sh;
# bump both together with the android* artifact-tags in ci.yaml.

build="${ANDROID_SYSROOT_BUILD:-15660610}"
target="${ANDROID_SYSROOT_TARGET:-aosp_cf_arm64_only_phone-userdebug}"

zip_name="${target%-userdebug}-target_files-${build}.zip"
url="https://ci.android.com/builds/submitted/${build}/${target}/latest/${zip_name}"

td="$(mktemp -d)"

# The ci.android.com URL serves an HTML page embedding a short-lived signed
# Cloud Storage URL (JSON-escaped '&'); scrape it to get the artifact.
signed_url="$(wget -q -O - "${url}" |
    grep -o 'https://storage.googleapis.com/android-build/[^"]*' |
    head -1 |
    sed 's/\\u0026/\&/g')"
wget -q --tries=5 -O "${td}/target_files.zip" "${signed_url}"

# Take the full lib64 rather than cherry-picking libc/libm/libdl: the
# linker pulls in supporting libraries that vary between releases.
# Extraction is best-effort; the sanity checks below decide.
unzip -q "${td}/target_files.zip" \
    'SYSTEM/apex/com.android.runtime.apex' \
    'SYSTEM/bin/linker64' \
    'SYSTEM/bin/bootstrap/*' \
    'SYSTEM/build.prop' \
    'SYSTEM/etc/ld.config*' \
    'SYSTEM/lib64/*' \
    -d "${td}" || true

mkdir -p /system
mv "${td}/SYSTEM/bin" /system/bin
mv "${td}/SYSTEM/lib64" /system/lib64
mv "${td}/SYSTEM/build.prop" /system/build.prop
if [ -d "${td}/SYSTEM/etc" ]; then
    mv "${td}/SYSTEM/etc" /system/etc
fi

# Since Android 10 bionic lives in the Runtime APEX and the /system paths
# are symlinks into /apex/com.android.runtime; unpack the apex payload
# there so they resolve.
unzip -q "${td}/SYSTEM/apex/com.android.runtime.apex" apex_payload.img \
    -d "${td}"
mkdir -p /apex/com.android.runtime
case "$(file -b "${td}/apex_payload.img")" in
    *EROFS*)
        fsck.erofs "--extract=/apex/com.android.runtime" \
            "${td}/apex_payload.img"
        ;;
    *ext[24]*)
        debugfs -R "rdump / /apex/com.android.runtime" \
            "${td}/apex_payload.img"
        ;;
    *)
        echo "unrecognized apex payload filesystem" >&2
        exit 1
        ;;
esac

rm -rf "${td}"

# Sanity-check the files the tests need (following the /apex symlinks, so
# this also verifies the apex extraction) and log the Android version.
test -s /system/bin/linker64
test -s /system/lib64/libc.so
test -s /system/lib64/libm.so
test -s /system/lib64/libdl.so
grep ro.build.version.release /system/build.prop
