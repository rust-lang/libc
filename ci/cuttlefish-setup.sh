#!/usr/bin/env bash

set -eux

# Fetch and boot a Cuttlefish Android virtual device, heavily inspired by
# Mesa's CI:
# https://gitlab.freedesktop.org/mesa/mesa/-/blob/main/.gitlab-ci/cuttlefish-runner.sh
#
# Runs unprivileged (cvd refuses to run as root) inside the test container,
# invoked by cuttlefish-entrypoint.sh after it has prepared the devices,
# networking and groups (see ci/cuttlefish-entrypoint.sh).
build="${CUTTLEFISH_BUILD:-15660610}"
target="${CUTTLEFISH_TARGET:-aosp_cf_x86_64_only_phone-userdebug}"
cf_dir="${CUTTLEFISH_DIR:-$HOME/cuttlefish}"

# Cross-arch guests need the host tools from the host's own build.
host_package_args=()
if [ -n "${CUTTLEFISH_HOST_TARGET:-}" ]; then
    host_package_args=(--host_package_build="${build}/${CUTTLEFISH_HOST_TARGET}")
fi

# crosvm can't run cross-arch guests; qemu_cli runs them under TCG.
vm_manager_args=()
if [ -n "${CUTTLEFISH_VM_MANAGER:-}" ]; then
    vm_manager_args=(-vm_manager="${CUTTLEFISH_VM_MANAGER}")
fi

mkdir -p "$cf_dir"
cvd fetch \
    --default_build="${build}/${target}" \
    "${host_package_args[@]}" \
    --target_directory="$cf_dir"

cd "$cf_dir"

# launch_cvd -daemon is silent until the guest has booted, so tail the
# device logs meanwhile. cvd creates them under /var/tmp/cvd as it goes.
follow_device_logs() {
    local instance
    while :; do
        instance=$(find /var/tmp/cvd "$cf_dir" -type d \
            -path '*/instances/cvd-[0-9]' 2>/dev/null | head -1)
        [ -n "$instance" ] && break
        sleep 2
    done
    exec tail -n +1 -F "$instance/logs/launcher.log" "$instance/kernel.log"
}

follow_device_logs &
follow_pid=$!

stop_following() {
    kill "$follow_pid" 2>/dev/null || true
    wait "$follow_pid" 2>/dev/null || true
}
trap stop_following EXIT

# -daemon exits only once the guest has fully booted.
# -enable_sandbox=false is needed because crosvm's minijail device sandbox
#  (auto-enabled when /var/empty exists) requires unprivileged user
#  namespaces, which Ubuntu 24.04+ blocks via AppArmor by default.
# The timeout allows for TCG boots, which take ~20m.
HOME="$cf_dir" timeout "${CUTTLEFISH_BOOT_TIMEOUT:-45m}" ./bin/launch_cvd \
    -daemon \
    "${vm_manager_args[@]}" \
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

stop_following
trap - EXIT

# Check the guest's adbd is reachable on the port the test runner uses,
# and log the Android version and ABI actually being tested.
HOME="$cf_dir" ./bin/adb connect 127.0.0.1:6520
HOME="$cf_dir" ./bin/adb -s 127.0.0.1:6520 wait-for-device
HOME="$cf_dir" ./bin/adb devices
HOME="$cf_dir" ./bin/adb -s 127.0.0.1:6520 shell \
    getprop ro.build.version.release
HOME="$cf_dir" ./bin/adb -s 127.0.0.1:6520 shell \
    getprop ro.product.cpu.abi
