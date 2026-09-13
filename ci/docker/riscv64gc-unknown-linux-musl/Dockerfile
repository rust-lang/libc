FROM ubuntu:26.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gcc \
    gcc-riscv64-linux-gnu \
    git \
    libc6-dev \
    make \
    patch \
    rsync \
    qemu-user \
    xz-utils

ARG MUSL_VERSION
COPY . /ci
RUN ci/install-musl.sh riscv64 "$MUSL_VERSION"

ENV PATH=$PATH:/musl-riscv64/bin:/rust/bin \
    CC_riscv64gc_unknown_linux_musl=musl-gcc \
    CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
    CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_MUSL_RUNNER="qemu-riscv64 -L /musl-riscv64" \
    EXTRA_RUSTFLAGS="-Ctarget-feature=+crt-static -Clink-args=-L/musl-riscv64/lib"
