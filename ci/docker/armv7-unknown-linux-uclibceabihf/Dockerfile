FROM ubuntu:26.04

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        gcc \
        libc6-dev \
        qemu-system-arm \
        qemu-user \
        bzip2 \
        gzip \
        cpio \
        unzip \
        rsync \
        bc \
        xz-utils

# The toolchain URL should point to a compressed buildroot SDK.
# See https://buildroot.org/downloads/manual/manual.html#build-toolchain-with-buildroot
# for how to build one, or see https://github.com/rust-lang/libc/pull/5261
# for an example toolchain.
ARG UCLIBC_TOOLCHAIN_URL
COPY install-uclibc.sh /
RUN /install-uclibc.sh "$UCLIBC_TOOLCHAIN_URL"

ENV PATH=$PATH:/rust/bin:/toolchain/bin \
    STAGING_DIR=/toolchain/armv7-buildroot-linux-uclibceabihf/sysroot \
    CC_armv7_unknown_linux_uclibceabihf=arm-buildroot-linux-uclibcgnueabihf-gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABIHF_LINKER=arm-buildroot-linux-uclibcgnueabihf-gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABIHF_RUNNER="qemu-arm -L /toolchain/arm-buildroot-linux-uclibcgnueabihf/sysroot/"
