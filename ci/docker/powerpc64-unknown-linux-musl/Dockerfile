FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gcc \
    gcc-powerpc64-linux-gnu \
    git \
    libc6-dev \
    make \
    patch \
    qemu-user \
    rsync \
    xz-utils

RUN curl -O https://static.rust-lang.org/dist/2026-06-26/rust-std-nightly-powerpc64-unknown-linux-musl.tar.xz && \
    tar xf rust-std-nightly-powerpc64-unknown-linux-musl.tar.xz && \
    find rust-std-nightly-powerpc64-unknown-linux-musl/ -name libcompiler_builtins*.rlib -exec sh -c "cp {} /compiler_builtins.rlib" \;

ARG MUSL_VERSION
COPY . /ci
RUN ci/install-musl.sh powerpc64 "$MUSL_VERSION"

# Rust's unwind unconditionally links to libgcc_s.so.1
# We instead get the unwinding symbols from the self-contained libunwind.a
# since that's included in the toolchain
RUN touch /musl-powerpc64/lib/libgcc_s.so.1

ENV PATH=$PATH:/musl-powerpc64/bin:/rust/bin \
    CC_powerpc64_unknown_linux_musl=musl-gcc \
    CFLAGS_powerpc64_unknown_linux_musl="-mabi=elfv2" \
    CARGO_TARGET_POWERPC64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
    CARGO_TARGET_POWERPC64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-ppc64 -L /musl-powerpc64" \
    EXTRA_RUSTFLAGS="-Clink-args=-lunwind -Clink-self-contained=yes -L /musl-powerpc64/lib"
