#!/usr/bin/env sh

set -ex

# Prep the SDK and emulator
#
# Note that the update process requires that we accept a bunch of licenses, and
# we can't just pipe `yes` into it for some reason, so we take the same strategy
# located in https://github.com/appunite/docker by just wrapping it in a script
# which apparently magically accepts the licenses.

SDK=8512546
mkdir -p sdk
wget -q --tries=20 https://dl.google.com/android/repository/commandlinetools-linux-${SDK}_latest.zip
unzip -q commandlinetools-linux-${SDK}_latest.zip
mv cmdline-tools /usr/lib/android-sdk/

case "$1" in
  arm | armv7)
    api=24
    image="system-images;android-${api};default;armeabi-v7a"
    ;;
  aarch64)
    api=24
    image="system-images;android-${api};google_apis;arm64-v8a"
    ;;
  i686)
    api=28
    image="system-images;android-${api};default;x86"
    ;;
  x86_64)
    api=28
    image="system-images;android-${api};default;x86_64"
    ;;
  *)
    echo "invalid arch: $1"
    exit 1
    ;;
esac;

# Try to fix warning about missing file.
# See https://askubuntu.com/a/1078784
mkdir -p /tmp/.android/avd
echo '### User Sources for Android SDK Manager' >> /tmp/.android/repositories.cfg
echo '#Fri Nov 03 10:11:27 CET 2017 count=0' >> /tmp/.android/repositories.cfg

# Print all available packages
# yes | ./sdk/tools/bin/sdkmanager --list --verbose

# --no_https avoids
# javax.net.ssl.SSLHandshakeException: sun.security.validator.ValidatorException: No trusted certificate found
#
# | grep -v = || true    removes the progress bar output from the sdkmanager
# which produces an insane amount of output.
yes | /usr/lib/android-sdk/cmdline-tools/bin/sdkmanager --licenses --no_https --sdk_root=/usr/lib/android-sdk | grep -v = || true
yes | /usr/lib/android-sdk/cmdline-tools/bin/sdkmanager --no_https --sdk_root=/usr/lib/android-sdk \
        "emulator" \
        "platform-tools" \
        "platforms;android-${api}" \
        "${image}" | grep -v = || true

echo "no" |
    /usr/lib/android-sdk/cmdline-tools/bin/avdmanager -v create avd \
        --name "${1}" \
        --package "${image}" \
        -p /usr/lib/android-sdk/ | grep -v = || true
