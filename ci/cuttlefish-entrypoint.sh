#!/usr/bin/env bash

set -eux

# Boot a Cuttlefish virtual device inside this container, then run the
# tests against it as an unprivileged user (cvd refuses to run as root).
#
# The container needs only /dev/kvm, /dev/net/tun, /dev/vhost-net,
# /dev/vhost-vsock and CAP_NET_ADMIN, passed by ci/run-docker.sh. It's
# the same set upstream uses for its own containerized Cuttlefish CI:
# https://github.com/google/android-cuttlefish/blob/main/.github/workflows/presubmit.yaml

# The device nodes come in with their host-side ownership: open them up
# for the unprivileged user (container-local, doesn't affect the host).
chmod 666 /dev/kvm /dev/net/tun /dev/vhost-net /dev/vhost-vsock

# Create the bridges, taps and dnsmasq instances backing the device's
# virtio-net, inside this container's own network namespace. The deb ships
# a sysv script for this, usable without systemd, which is also how
# upstream's container image starts it.
service cuttlefish-host-resources start

# cvd requires the current process to be in these groups. The groups are
# normally created by the deb postinst, -f covers any it didn't create.
for g in kvm cvdnetwork render; do
    groupadd -f "$g"
done
# Match the invoking user's uid so writes to the bind-mounted target/
# directory stay owned by the host user.
useradd -m -u "${HOST_UID:?}" -G kvm,cvdnetwork,render cf

run_as_cf() {
    runuser -u cf -- env PATH="$PATH" HOME=/tmp "$@"
}

# Keep the device under target/ (bind-mounted from the host) so its logs
# survive the container for ci/create-artifacts.py to pick up.
export CUTTLEFISH_DIR=/checkout/target/cuttlefish

run_as_cf /android/cuttlefish-setup.sh

# The test runner (runtest-android.rs) drives the device with the
# container's adb via $ANDROID_SERIAL. Connect its server to the device
# (this may replace the server started by the fetched adb during boot).
run_as_cf adb connect "$ANDROID_SERIAL"
run_as_cf adb wait-for-device

run_as_cf rustc /tmp/runtest.rs -o /tmp/runtest

rc=0
run_as_cf "$@" || rc=$?

# Stop the device so its logs are flushed for ci/create-artifacts.py.
run_as_cf env HOME="$CUTTLEFISH_DIR" \
    "$CUTTLEFISH_DIR/bin/stop_cvd" -wait_for_launcher=40 || true

exit "$rc"
