#!/bin/bash

# Runs `run.sh` on a non-macOS Apple target.
#
# This requires `cargo-apple-runner`, which you can get with `cargo install cargo-apple-runner`.

set -eux

target="$1"

# Find reasonable simulator device.
case "$target" in
    *ios-sim) SIMULATOR_DEVICE="iPhone 17" ;;
    *tvos-sim) SIMULATOR_DEVICE="Apple TV" ;;
    *watchos-sim) SIMULATOR_DEVICE="Apple Watch SE 3 (40mm)" ;;
    *visionos-sim) SIMULATOR_DEVICE="Apple Vision Pro" ;;
    *) echo "Unknown simulator target: $target"; exit 1 ;;
esac

# Start the simulator.
# If this fails, the user is likely already running a simulator,
# in that case we don't want to shut it down on exit.
if xcrun simctl boot "$SIMULATOR_DEVICE"; then
    echo "Successfully booted simulator"
    trap 'xcrun simctl shutdown "$SIMULATOR_DEVICE"' EXIT
else
    echo "Didn't boot simulator; one is probably already running (?)"
fi

# Set the Cargo runner to `cargo-apple-runner` for the simulator targets.
export CARGO_TARGET_AARCH64_APPLE_IOS_SIM_RUNNER=cargo-apple-runner
export CARGO_TARGET_AARCH64_APPLE_TVOS_SIM_RUNNER=cargo-apple-runner
export CARGO_TARGET_AARCH64_APPLE_WATCHOS_SIM_RUNNER=cargo-apple-runner
export CARGO_TARGET_AARCH64_APPLE_VISIONOS_SIM_RUNNER=cargo-apple-runner
export CARGO_TARGET_X86_64_APPLE_IOS_RUNNER=cargo-apple-runner
export CARGO_TARGET_X86_64_APPLE_WATCHOS_SIM_RUNNER=cargo-apple-runner
export CARGO_TARGET_X86_64_APPLE_TVOS_RUNNER=cargo-apple-runner
export CARGO_TARGET_I386_APPLE_IOS_RUNNER=cargo-apple-runner

# Delegate to run.sh for the rest.
"$(dirname "$0")/run.sh" "$target"
