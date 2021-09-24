FROM ubuntu:20.04

RUN apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev qemu-user ca-certificates qemu-system-arm curl \
        xz-utils patch

RUN mkdir /toolchain

RUN curl --retry 5 -L https://toolchains.bootlin.com/downloads/releases/toolchains/armv7-eabihf/tarballs/armv7-eabihf--uclibc--bleeding-edge-2020.08-1.tar.bz2 | \
    tar xjf - -C /toolchain --strip-components=1
RUN /toolchain/relocate-sdk.sh

ENV PATH=$PATH:/rust/bin:/toolchain/bin \
    STAGING_DIR=/toolchain/armv7-buildroot-linux-uclibceabihf/sysroot \
    CC_armv7_unknown_linux_uclibc=armv7-buildroot-linux-uclibc-gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABIHF_LINKER=armv7-buildroot-linux-uclibc-gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABIHF_RUNNER="qemu-arm -L /toolchain/arm-buildroot-linux-uclibcgnueabihf/sysroot/"
