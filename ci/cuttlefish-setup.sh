#!/usr/bin/env bash

set -eux

# Fetch and boot a Cuttlefish Android virtual device, heavily inspired by
# Mesa's CI:
# https://gitlab.freedesktop.org/mesa/mesa/-/blob/main/.gitlab-ci/cuttlefish-runner.sh
#
# Runs unprivileged (cvd refuses to run as root) inside the test container,
# invoked by cuttlefish-entrypoint.sh after it has prepared the devices,
# networking and groups (see ci/docker/x86_64-linux-android/).
build="${CUTTLEFISH_BUILD:-15660610}"
target="${CUTTLEFISH_TARGET:-aosp_cf_x86_64_only_phone-userdebug}"
cf_dir="${CUTTLEFISH_DIR:-$HOME/cuttlefish}"

mkdir -p "$cf_dir"
cvd fetch --default_build="${build}/${target}" --target_directory="$cf_dir"

cd "$cf_dir"

# -daemon exits only once the guest has fully booted.
# -enable_sandbox=false is needed because crosvm's minijail device sandbox
#  (auto-enabled when /var/empty exists) requires unprivileged user
#  namespaces, which Ubuntu 24.04+ blocks via AppArmor by default.
HOME="$cf_dir" timeout 10m ./bin/launch_cvd \
    -daemon \
    -enable_sandbox=false \
    -verbosity=INFO \
    -file_verbosity=DEBUG \
    -enable_audio=false \
    -enable_bootanimation=false \
    -enable_minimal_mode=true \
    -enable_modem_simulator=false \
    -enable_wifi=false \
    -report_anonymous_usage_stats=no \
    -cpus=2 \
    -memory_mb=4096

# Check the guest's adbd is reachable on the port the test runner uses,
# and log the Android version actually being tested.
HOME="$cf_dir" ./bin/adb connect 127.0.0.1:6520
HOME="$cf_dir" ./bin/adb -s 127.0.0.1:6520 wait-for-device
HOME="$cf_dir" ./bin/adb devices
HOME="$cf_dir" ./bin/adb -s 127.0.0.1:6520 shell \
    getprop ro.build.version.release
