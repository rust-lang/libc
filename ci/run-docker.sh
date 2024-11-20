#!/bin/sh

# Disable SC2086 as it confuses the docker command.
# shellcheck disable=SC2086

# Small script to run tests for a target (or all targets) inside all the
# respective docker images.

set -eux

target="$1"

# Default to assuming the CARGO_HOME is one directory up (to account for a `bin`
# subdir) from where the `cargo` binary in `$PATH` lives.
default_cargo_home="$(dirname "$(dirname "$(command -v cargo)")")"
# If the CARGO_HOME env var is already set, use that. If it isn't set use the
# default.
export CARGO_HOME="${CARGO_HOME:-$default_cargo_home}"

echo "${HOME}"
pwd

# Avoid "no space left on device" failure if running in CI
if [ "${CI:-0}" != "0" ] && [ "$target" = "aarch64-linux-android" ] ; then
    docker system prune -af
    docker system df
fi

run() {
    echo "Building docker container for target $target"

    # use -f so we can use ci/ as build context
    docker build -t "libc-$target" -f "ci/docker/$target/Dockerfile" ci/
    mkdir -p target
    if [ -w /dev/kvm ]; then
        kvm="--volume /dev/kvm:/dev/kvm"
    else
        kvm=""
    fi

    docker run \
        --rm \
        --user "$(id -u)":"$(id -g)" \
        --env LIBC_CI \
        --env LIBC_CI_ZBUILD_STD \
        --env CARGO_HOME=/cargo \
        --env CARGO_TARGET_DIR=/checkout/target \
        --volume "$CARGO_HOME":/cargo \
        --volume "$(rustc --print sysroot)":/rust:ro \
        --volume "$(pwd)":/checkout:ro \
        --volume "$(pwd)"/target:/checkout/target \
        $kvm \
        --init \
        --workdir /checkout \
        "libc-$target" \
        sh -c "HOME=/tmp PATH=\$PATH:/rust/bin exec ci/run.sh $target"
}

build_switch() {
    echo "Building docker container for target switch"

    # use -f so we can use ci/ as build context
    docker build -t libc-switch -f "ci/docker/switch/Dockerfile" ci/
    mkdir -p target
    if [ -w /dev/kvm ]; then
        kvm="--volume /dev/kvm:/dev/kvm"
    else
        kvm=""
    fi

    cp "$(command -v rustup)" "$(rustc --print sysroot)/bin"

    docker run \
        --rm \
        --user "$(id -u)":"$(id -g)" \
        --env LIBC_CI \
        --env CARGO_HOME=/cargo \
        --env CARGO_TARGET_DIR=/checkout/target \
        --volume "$CARGO_HOME":/cargo \
        --volume "$(rustc --print sysroot)":/rust:ro \
        --volume "$(pwd)":/checkout:ro \
        --volume "$(pwd)"/target:/checkout/target \
        --volume ~/.rustup:/.rustup:Z \
        $kvm \
        --init \
        --workdir /checkout \
        libc-switch \
        sh -c "HOME=/tmp RUSTUP_HOME=/tmp PATH=\$PATH:/rust/bin rustup default nightly \
            && rustup component add rust-src --target ci/switch.json \
            && cargo build -Z build-std=core,alloc --target ci/switch.json"
}

if [ -z "$target" ]; then
    for d in ci/docker/*; do
        run "${d}"
    done
else
    if [ "$target" != "switch" ]; then
        run "$target"
    else
        build_switch
    fi
fi
