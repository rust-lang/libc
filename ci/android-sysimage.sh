#!/usr/bin/env bash

set -ex

URL=https://dl.google.com/android/repository/sys-img/android

main() {
    local arch="${1}"
    local name="${2}"
    local dest=/system
    local td
    td="$(mktemp -d)"

    apt-get install --no-install-recommends e2tools

    pushd "${td}"
    wget -q --tries=5 "${URL}/${name}"
    unzip -q "${name}"

    local system
    system="$(find . -name system.img)"
    mkdir -p ${dest}/{bin,lib,lib64}

    # Extract android linker and libraries to /system
    # This allows android executables to be run directly (or with qemu)
    if [ "${arch}" = "x86_64" ] || [ "${arch}" = "arm64" ]; then
        e2cp -p "${system}:/bin/linker64" "${dest}/bin/"
        e2cp -p "${system}:/lib64/libdl.so" "${dest}/lib64/"
        e2cp -p "${system}:/lib64/libc.so" "${dest}/lib64/"
        e2cp -p "${system}:/lib64/libm.so" "${dest}/lib64/"
    else
        e2cp -p "${system}:/bin/linker" "${dest}/bin/"
        e2cp -p "${system}:/lib/libdl.so" "${dest}/lib/"
        e2cp -p "${system}:/lib/libc.so" "${dest}/lib/"
        e2cp -p "${system}:/lib/libm.so" "${dest}/lib/"
    fi

    # clean up
    apt-get purge --auto-remove -y e2tools

    popd

    rm -rf "${td}"
}

main "${@}"
