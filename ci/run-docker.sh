#!/bin/bash

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
if [ "${CI:-0}" != "0" ] && [ "$target" = "aarch64-linux-android" ]; then
    docker system prune -af
    docker system df
fi

run() {
    run_target="$1"
    echo "Building docker container for target $run_target"

    build_args=(
        "--tag=libc-$run_target"
        "--file=ci/docker/$run_target/Dockerfile"
        "ci/"
    )

    # FIXME(ci): we could probably build both versions in the same dockerfile
    # and test them both in `ci/run.sh` more similar to what we do with glibc,
    # rather than needing two separate jobs.
    if [[ "$run_target" = *"musl"* ]]; then
        if [ -n "${TEST_MUSL_V1_2_3:-}" ]; then
            export RUSTFLAGS="$RUSTFLAGS --cfg=libc_unstable_musl_v1_2_3"
            build_args+=("--build-arg=MUSL_VERSION=new")
        else
            build_args+=("--build-arg=MUSL_VERSION=old")
        fi
    fi

    if [ -n "${TEST_UCLIBC_TIME64:-}" ]; then
        build_args+=("--build-arg=TEST_UCLIBC_TIME64=1")
        export RUSTFLAGS="$RUSTFLAGS --cfg=libc_unstable_uclibc_time64"
    fi

    # use -f so we can use ci/ as build context
    docker build "${build_args[@]}"

    mkdir -p target

    extra_args=()
    # x86_64-linux-android boots a Cuttlefish virtual device inside the
    # container. Give it the virtualization device nodes and NET_ADMIN it needs.
    # These are the same flags Google's android-cuttlefish project uses for its
    # own containerized CI.
    if [ "$run_target" = "x86_64-linux-android" ]; then
        extra_args+=(
            --device /dev/kvm
            --device /dev/net/tun
            --device /dev/vhost-net
            --device /dev/vhost-vsock
            --cap-add NET_ADMIN
            --security-opt seccomp=unconfined
            --env CUTTLEFISH_BUILD
            --env CUTTLEFISH_TARGET
            --env HOST_UID="$(id -u)"
        )
    else
        extra_args+=(--user "$(id -u)":"$(id -g)")
        if [ -w /dev/kvm ]; then
            extra_args+=(--volume /dev/kvm:/dev/kvm)
        fi
    fi

    docker run \
        --rm \
        --env LIBC_BUILD_VERBOSE \
        "${extra_args[@]}" \
        --env LIBC_CI \
        --env LIBC_CI_ZBUILD_STD \
        --env RUSTFLAGS \
        --env RUSTDOCFLAGS \
        --env RUST_BACKTRACE \
        --env CARGO_TERM_COLOR \
        --env CARGO_TERM_VERBOSE \
        --env CARGO_HOME=/cargo \
        --env CARGO_TARGET_DIR=/checkout/target \
        --volume "$CARGO_HOME":/cargo:Z \
        --volume "$(rustc --print sysroot)":/rust:ro,Z \
        --volume "$PWD":/checkout:ro,Z \
        --volume "$PWD"/target:/checkout/target \
        --init \
        --workdir /checkout \
        "libc-$target" \
        sh -c "HOME=/tmp PATH=\$PATH:/rust/bin exec ci/run.sh $target"
}

if [ -z "$target" ]; then
    # Run all docker targets
    for d in ci/docker/*; do
        run "${d}"
    done
else
    run "$target"
fi
