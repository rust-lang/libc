FROM ubuntu:26.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gcc \
    gcc-s390x-linux-gnu \
    git \
    libc6-dev \
    make \
    patch \
    rsync \
    qemu-user \
    xz-utils

ARG MUSL_VERSION
COPY . /ci
RUN ci/install-musl.sh s390x "$MUSL_VERSION"

# FIXME: shouldn't need the `-lgcc` here, shouldn't that be in std?
ENV CC_s390x_unknown_linux_gnu=musl-gcc \
    CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_LINKER=s390x-linux-gnu-gcc \
    CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_RUNNER="qemu-s390x -L /musl-s390x" \
    EXTRA_RUSTFLAGS="-Clink-args=-lgcc -L /musl-s390x/lib" \
    PATH=$PATH:/musl-s390x/bin:/rust/bin
