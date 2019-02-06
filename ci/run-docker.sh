#!/usr/bin/env sh

# Small script to run tests for a target (or all targets) inside all the
# respective docker images.

set -ex

run() {
    echo "Building docker container for target ${1}"

    # use -f so we can use ci/ as build context
    docker build -t libc -f "ci/docker/${1}/Dockerfile" ci/
    mkdir -p target
    if [ -w /dev/kvm ]; then
        kvm="--volume /dev/kvm:/dev/kvm"
    else
        kvm=""
    fi

    docker run \
      --user "$(id -u)":"$(id -g)" \
      --rm \
      --init \
      --volume "${HOME}/.cargo":/cargo \
      $kvm \
      --env CARGO_HOME=/cargo \
      --env RUNNING_IN_DOCKER=1 \
      --env BUILD_ONLY="${BUILD_ONLY}" \
      --env NIGHTLY_ONLY="${NIGHTLY_ONLY}" \
      --env NO_STD="${NO_STD}" \
      --volume "$(rustc --print sysroot)":/rust:ro \
      --volume "${HOME}"/.rustup:/.rustup:ro \
      --env RUSTUP_HOME=/.rustup \
      --volume "$(pwd)":/checkout \
      --volume "$(pwd)"/target:/checkout/target \
      --env CARGO_TARGET_DIR=/checkout/target \
      --workdir /checkout \
      libc \
      ci/run.sh "${1}"
}

if [ -z "${1}" ]; then
  for d in ci/docker/*; do
    run "${d}"
  done
else
  run "${1}"
fi
