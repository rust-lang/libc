FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  gcc make libc6-dev git curl ca-certificates \
  gcc-powerpc64le-linux-gnu qemu-user xz-utils patch rsync

COPY install-musl.sh /
RUN /install-musl.sh powerpc64le

# FIXME: shouldn't need the `-lgcc` here, shouldn't that be in std?
ENV PATH=$PATH:/musl-powerpc64/bin:/rust/bin \
    CC_powerpc64le_unknown_linux_musl=musl-gcc \
    RUSTFLAGS='-Clink-args=-lgcc -L /musl-powerpc64/lib' \
    CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
    CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_MUSL_RUNNER="qemu-ppc64le -L /musl-powerpc64"
