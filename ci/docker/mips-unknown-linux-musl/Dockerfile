FROM ubuntu:19.04

RUN apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev qemu-user ca-certificates qemu-system-mips curl \
        bzip2

RUN mkdir /toolchain

# Note that this originally came from:
# https://downloads.openwrt.org/snapshots/targets/ar71xx/generic/openwrt-sdk-ar71xx-generic_gcc-7.4.0_musl.Linux-x86_64.tar.xz
RUN curl --retry 5 -L https://downloads.openwrt.org/snapshots/targets/ar71xx/generic/openwrt-sdk-ar71xx-generic_gcc-7.4.0_musl.Linux-x86_64.tar.xz | \
      tar xf -J - -C /toolchain --strip-components=1

ENV PATH=$PATH:/rust/bin:/toolchain/staging_dir/toolchain-mips_34kc_gcc-7.4.0_musl.Linux-x86_64/bin \
    CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc \
    CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_LINKER=mips-openwrt-linux-gcc \
    CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_RUNNER="qemu-mips -L /toolchain/staging_dir/toolchain-mips_34kc_gcc-7.4.0_musl.Linux-x86_64"
