FROM ubuntu:23.10

# FIXME(time): we are using an EOL release because 24.04 changes to 64-bit time
RUN sed -i -E 's/(archive|security)\.ubuntu\.com/old-releases.ubuntu.com/g' \
        /etc/apt/sources.list && \
    apt-get update && apt-get install -y --no-install-recommends \
        gcc make libc6-dev git curl ca-certificates \
        gcc-arm-linux-gnueabihf qemu-user xz-utils patch rsync

COPY install-musl.sh /
RUN /install-musl.sh arm

ENV PATH=$PATH:/musl-arm/bin:/rust/bin \
    CC_arm_unknown_linux_musleabihf=musl-gcc \
    CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER=musl-gcc \
    CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_RUNNER="qemu-arm -L /musl-arm" \
    RUSTFLAGS="-L /musl-arm/lib"
