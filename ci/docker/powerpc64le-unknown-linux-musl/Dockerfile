FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gcc \
    gcc-powerpc64le-linux-gnu \
    git \
    libc6-dev \
    make \
    patch \
    qemu-user \
    rsync \
    xz-utils

ARG MUSL_VERSION
COPY install-musl.sh /
RUN /install-musl.sh powerpc64le "$MUSL_VERSION"

# FIXME: shouldn't need the `-lgcc` here, shouldn't that be in std?
ENV PATH=$PATH:/musl-powerpc64/bin:/rust/bin \
    CC_powerpc64le_unknown_linux_musl=musl-gcc \
    RUSTFLAGS='-Clink-args=-lgcc -L /musl-powerpc64/lib' \
    CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
    CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_MUSL_RUNNER="qemu-ppc64le -L /musl-powerpc64"
