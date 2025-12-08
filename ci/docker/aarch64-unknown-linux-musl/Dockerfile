FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gcc \
    gcc-aarch64-linux-gnu \
    git \
    libc6-dev \
    make \
    patch \
    qemu-user \
    rsync \
    xz-utils

ARG MUSL_VERSION
COPY install-musl.sh /
RUN /install-musl.sh aarch64 "$MUSL_VERSION"

# FIXME: shouldn't need the `-lgcc` here, shouldn't that be in std?
ENV PATH=$PATH:/musl-aarch64/bin:/rust/bin \
    CC_aarch64_unknown_linux_musl=musl-gcc \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-aarch64 -L /musl-aarch64" \
    EXTRA_RUSTFLAGS="-Clink-args=-lgcc -L /musl-aarch64/lib"
