FROM ubuntu:18.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  gcc make libc6-dev git curl ca-certificates \
  gcc-mips-linux-gnu qemu-user qemu-system-mips
RUN curl --retry 5 https://www.musl-libc.org/releases/musl-1.2.22.tar.gz | \
    tar xzf - && \
    cd musl-1.1.22 \
    CC=mips-linux-gcc \
    ./configure --prefix=/musl-mips --enable-wrapper=yes && \
    make install -j4 && \
    cd .. && \
    rm -rf musl-1.1.22
# Install linux kernel headers sanitized for use with musl
RUN curl --retry 5 -L https://github.com/sabotage-linux/kernel-headers/archive/v3.12.6-6.tar.gz | \
    tar xzf - && \
    cd kernel-headers-3.12.6-6 && \
    make ARCH=mips prefix=/musl-mips install -j4 && \
    cd .. && \
    rm -rf kernel-headers-3.12.6-6

# FIXME: shouldn't need the `-lgcc` here, shouldn't that be in libstd?
ENV PATH=$PATH:/musl-aarch64/bin:/rust/bin \
    CC_mips_unknown_linux_musl=musl-gcc \
    RUSTFLAGS='-Clink-args=-lgcc' \
    CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
    CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_RUNNER="qemu-mips -L /musl-mips"

# RUN mkdir /toolchain

# Note that this originally came from:
# https://downloads.openwrt.org/snapshots/trunk/ar71xx/generic/OpenWrt-SDK-ar71xx-generic_gcc-5.3.0_musl-1.1.15.Linux-x86_64.tar.bz2
# RUN curl --retry 5 -L https://s3-us-west-1.amazonaws.com/rust-lang-ci2/libc/OpenWrt-SDK-ar71xx-generic_gcc-5.3.0_musl-1.1.15.Linux-x86_64.tar.bz2 | \
#       tar xjf - -C /toolchain --strip-components=1

# ENV PATH=$PATH:/rust/bin:/toolchain/staging_dir/toolchain-mips_34kc_gcc-5.3.0_musl-1.1.15/bin \
#     CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc \
#     CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_LINKER=mips-openwrt-linux-gcc \
#     CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_RUNNER="qemu-mips -L /toolchain/staging_dir/toolchain-mips_34kc_gcc-5.3.0_musl-1.1.15"
