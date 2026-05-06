FROM ubuntu:23.10

# FIXME(time): we are using an EOL release because 24.04 changes to 64-bit time
RUN sed -i -E 's/(archive|security)\.ubuntu\.com/old-releases.ubuntu.com/g' \
        /etc/apt/sources.list && \
    apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        file \
        gcc \
        libc6-dev \
        patch \
        qemu-system-arm \
        qemu-user \
        xz-utils

ARG TEST_UCLIBC_TIIME64
COPY install-uclibc.sh /
RUN /install-uclibc.sh "$TEST_UCLIBC_TIIME64"

ENV PATH=$PATH:/rust/bin:/toolchain/bin \
    STAGING_DIR=/toolchain/armv7-buildroot-linux-uclibceabihf/sysroot \
    CC_armv7_unknown_linux_uclibceabihf=arm-buildroot-linux-uclibcgnueabihf-gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABIHF_LINKER=arm-buildroot-linux-uclibcgnueabihf-gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABIHF_RUNNER="qemu-arm -L /toolchain/arm-buildroot-linux-uclibcgnueabihf/sysroot/"
