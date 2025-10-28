FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    gcc-14-loongarch64-linux-gnu \
    libc6-dev \
    libc6-dev-loong64-cross \
    linux-headers-generic \
    qemu-user

ENV CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_LINKER=loongarch64-linux-gnu-gcc-14 \
    CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_RUNNER="qemu-loongarch64 -L /usr/loongarch64-linux-gnu" \
    CC_loongarch64_unknown_linux_gnu=loongarch64-linux-gnu-gcc-14 \
    CFLAGS_loongarch64_unknown_linux_gnu="-mabi=lp64d -fPIC" \
    PATH=$PATH:/rust/bin
