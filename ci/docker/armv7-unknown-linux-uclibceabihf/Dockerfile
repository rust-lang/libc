FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        file \
        gcc \
        libc6-dev \
        patch \
        qemu-system-arm \
        qemu-user \
        bzip2 \
        gzip \
        make \
        wget \
        cpio \
        unzip \
        rsync \
        bc \
        g++ \
        perl \
        xz-utils

ARG TEST_UCLIBC_TIME64
COPY install-uclibc.sh /
COPY buildroot_defconfigs/armv7_unknown_linux_uclibceabihf.config /
RUN /install-uclibc.sh /armv7_unknown_linux_uclibceabihf.config "$TEST_UCLIBC_TIME64"

ENV PATH=$PATH:/rust/bin:/buildroot/output/host/bin \
    STAGING_DIR=/buildroot/output/host/armv7-buildroot-linux-uclibceabihf/sysroot \
    CC_armv7_unknown_linux_uclibceabihf=arm-buildroot-linux-uclibcgnueabihf-gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABIHF_LINKER=arm-buildroot-linux-uclibcgnueabihf-gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABIHF_RUNNER="qemu-arm -L /buildroot/output/host/arm-buildroot-linux-uclibcgnueabihf/sysroot/"
