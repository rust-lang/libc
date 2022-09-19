FROM ubuntu:22.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  gcc make libc6-dev git curl ca-certificates \
  gcc-mips64-linux-gnuabi64 qemu-user

COPY install-musl.sh /
RUN sh /install-musl.sh mips64

# FIXME: shouldn't need the `-lgcc` here, shouldn't that be in libstd?
ENV PATH=$PATH:/musl-mips64/bin:/rust/bin \
    CC_mips64_unknown_linux_muslabi64=musl-gcc \
    RUSTFLAGS='-Clink-args=-lgcc -L /musl-mips64/lib' \
    CARGO_TARGET_MIPS64_UNKNOWN_LINUX_MUSLABI64_LINKER=musl-gcc \
    CARGO_TARGET_MIPS64_UNKNOWN_LINUX_MUSLABI64_RUNNER="qemu-mips64 -L /musl-mips64"
