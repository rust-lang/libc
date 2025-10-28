FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gcc \
    gcc-14-loongarch64-linux-gnu \
    git \
    libc6-dev \
    make \
    patch \
    qemu-user \
    rsync \
    xz-utils

COPY install-musl.sh /
RUN /install-musl.sh loongarch64

ENV CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
    CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-loongarch64" \
    CC_loongarch64_unknown_linux_musl=musl-gcc \
    CFLAGS_loongarch64_unknown_linux_musl="-mabi=lp64d -fPIC" \
    RUSTFLAGS="-Ctarget-feature=+crt-static" \
    QEMU_LD_PREFIX=/musl-loongarch64 \
    PATH=$PATH:/musl-loongarch64/bin:/rust/bin
