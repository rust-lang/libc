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

# The device images are per-guest-architecture but the host tools that run
# them are not: on the arm64 target these have to keep coming from the
# x86_64 build, because cvd fetch otherwise takes the host package from
# --default_build and lands arm64 binaries the runner cannot execute.
host_package_args=()
if [ -n "${CUTTLEFISH_HOST_TARGET:-}" ]; then
    host_package_args=(--host_package_build="${build}/${CUTTLEFISH_HOST_TARGET}")
fi

# crosvm needs host and guest to share an architecture. Cross-architecture
# guests go through qemu instead, which Cuttlefish supports: it drops -accel
# and swaps -cpu host for -cpu max when the target doesn't match the host
# (see IsHostCompatible() in host/libs/vm_manager/qemu_manager.cpp), so the
# guest runs under TCG.
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

# -daemon exits only once the guest has fully booted.
# -enable_sandbox=false is needed because crosvm's minijail device sandbox
#  (auto-enabled when /var/empty exists) requires unprivileged user
#  namespaces, which Ubuntu 24.04+ blocks via AppArmor by default.
#
# The timeout is generous because a TCG guest (the arm64 target, which has no
# matching hardware to accelerate it) boots far slower than a hardware-
# accelerated one, and slower still on a loaded CI runner: measured arm64
# boots ranged from 16 to 22 minutes for the same configuration, against
# roughly one for x86_64. It is only here to bound a hang, not to enforce a
# target.
# launch_cvd -daemon prints nothing until the guest has finished booting,
# which on an emulated guest is around twenty silent minutes. Follow the
# device's own logs meanwhile so the job shows progress, and so a boot that
# hangs leaves something to read instead of just hitting the timeout.
#
# cvd keeps the runtime under a per-group directory rather than $cf_dir, and
# creates it as it goes, so wait for the instance directory to appear and let
# tail -F pick the files up from there once they exist. Match the instance
# itself rather than any "logs" directory: cvd keeps its own tool logs in
# /var/tmp/cvd/<uid>/logs, which does not hold either of these.
follow_device_logs() {
    local instance
    while :; do
        instance=$(find /var/tmp/cvd "$cf_dir" -type d \
            -path '*/instances/cvd-[0-9]' 2>/dev/null | head -1)
        [ -n "$instance" ] && break
        sleep 2
    done
    # exec so this background job *is* tail, and killing it below is enough.
    exec tail -n +1 -F "$instance/logs/launcher.log" "$instance/kernel.log"
}

follow_device_logs &
follow_pid=$!

stop_following() {
    kill "$follow_pid" 2>/dev/null || true
    wait "$follow_pid" 2>/dev/null || true
}
trap stop_following EXIT

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

# The device is up; stop following so the checks below aren't interleaved
# with the guest's own logging.
stop_following
trap - EXIT

# Check the guest's adbd is reachable on the port the test runner uses,
# and log the Android version and ABI actually being tested. The ABI
# matters because a mismatch here shows up much later as the test binaries
# failing to exec, which is far harder to read than this line.
HOME="$cf_dir" ./bin/adb connect 127.0.0.1:6520
HOME="$cf_dir" ./bin/adb -s 127.0.0.1:6520 wait-for-device
HOME="$cf_dir" ./bin/adb devices
HOME="$cf_dir" ./bin/adb -s 127.0.0.1:6520 shell \
    getprop ro.build.version.release
HOME="$cf_dir" ./bin/adb -s 127.0.0.1:6520 shell \
    getprop ro.product.cpu.abi
