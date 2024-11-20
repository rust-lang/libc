FROM ubuntu:24.10

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates curl gcc git libc6-dev make qemu-user xz-utils

COPY install-musl-cross.sh /
RUN /install-musl-cross.sh loongarch64-unknown-linux-musl

ENV CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_MUSL_LINKER=loongarch64-unknown-linux-musl-gcc \
    CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-loongarch64" \
    CC_loongarch64_unknown_linux_musl=loongarch64-unknown-linux-musl-gcc \
    CFLAGS_loongarch64_unknown_linux_musl="-mabi=lp64d -fPIC" \
    QEMU_LD_PREFIX=/loongarch64-unknown-linux-musl/loongarch64-unknown-linux-musl/sysroot \
    PATH=$PATH:/loongarch64-unknown-linux-musl/bin:/rust/bin
