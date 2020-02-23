FROM ubuntu:19.10

RUN apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev qemu-user ca-certificates qemu-system-mips curl \
        xz-utils patch

RUN mkdir /toolchain

# Linux kernel version: 4.14.151
# See build_dir/target-mips_24kc_musl/linux-ar71xx_generic/linux-4.14.151
# Musl version: 1.1.24
# See staging_dir/toolchain-mips_24kc_gcc-7.4.0_musl/info.mk
RUN curl --retry 5 -L https://downloads.openwrt.org/releases/19.07.0-rc1/targets/ar71xx/generic/openwrt-sdk-19.07.0-rc1-ar71xx-generic_gcc-7.4.0_musl.Linux-x86_64.tar.xz | \
    tar xJf - -C /toolchain --strip-components=1

# See https://lkml.org/lkml/2014/3/14/269
COPY sysinfo_guard.patch /toolchain
RUN patch /toolchain/staging_dir/toolchain-mips_24kc_gcc-7.4.0_musl/include/linux/kernel.h </toolchain/sysinfo_guard.patch

ENV PATH=$PATH:/rust/bin:/toolchain/staging_dir/toolchain-mips_24kc_gcc-7.4.0_musl/bin \
    STAGING_DIR=/toolchain/staging_dir \
    CC_mips_unknown_linux_musl=mips-openwrt-linux-musl-gcc \
    CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_LINKER=mips-openwrt-linux-musl-gcc \
    CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_RUNNER="qemu-mips -L /toolchain/staging_dir/toolchain-mips_24kc_gcc-7.4.0_musl"
