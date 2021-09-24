FROM ubuntu:20.04

RUN apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev qemu-user ca-certificates qemu-system-mipsel curl \
        xz-utils patch

RUN mkdir /toolchain

# binutils	2.33.1
# gcc	9.3.0
# gdb	8.3.1
# linux-headers	4.9.234
# uclibc	1.0.34
RUN curl --retry 5 -L https://toolchains.bootlin.com/downloads/releases/toolchains/mips32el/tarballs/mips32el--uclibc--stable-2020.08-1.tar.bz2 | \
    tar xjf - -C /toolchain --strip-components=1
RUN /toolchain/relocate-sdk.sh

ENV PATH=$PATH:/rust/bin:/toolchain/bin \
    STAGING_DIR=/toolchain/mipsel-buildroot-linux-uclibc/sysroot \
    CC_mipsel_unknown_linux_uclibc=mipsel-buildroot-linux-uclibc-gcc \
    CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_UCLIBC_LINKER=mipsel-buildroot-linux-uclibc-gcc \
    CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_UCLIBC_RUNNER="qemu-mipsel -L /toolchain/mipsel-buildroot-linux-uclibc/sysroot/"
