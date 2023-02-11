FROM ubuntu:22.04

RUN apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev qemu-user ca-certificates qemu-system-mips curl \
        xz-utils patch

RUN mkdir /toolchain

# Linux kernel version: 5.4.154
# See build_dir/target-mips_24kc_musl/linux-ath79_generic/linux-5.4.154
# Musl version: 1.1.24
# See staging_dir/toolchain-mips_24kc_gcc-8.4.0_musl/info.mk
RUN curl --retry 5 -L https://downloads.openwrt.org/releases/21.02.1/targets/ath79/generic/openwrt-sdk-21.02.1-ath79-generic_gcc-8.4.0_musl.Linux-x86_64.tar.xz | \
    tar xJf - -C /toolchain --strip-components=1

# See https://lkml.org/lkml/2014/3/14/269
COPY sysinfo_guard.patch /toolchain
RUN patch /toolchain/staging_dir/toolchain-mips_24kc_gcc-8.4.0_musl/include/linux/kernel.h </toolchain/sysinfo_guard.patch

ENV PATH=$PATH:/rust/bin:/toolchain/staging_dir/toolchain-mips_24kc_gcc-8.4.0_musl/bin \
    STAGING_DIR=/toolchain/staging_dir \
    CC_mips_unknown_linux_musl=mips-openwrt-linux-musl-gcc \
    CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_LINKER=mips-openwrt-linux-musl-gcc \
    CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_RUNNER="qemu-mips -L /toolchain/staging_dir/toolchain-mips_24kc_gcc-8.4.0_musl" \
    RUSTFLAGS="-L /toolchain/staging_dir/toolchain-mips_24kc_gcc-8.4.0_musl/lib"
