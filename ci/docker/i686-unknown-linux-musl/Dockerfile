FROM ubuntu:26.04

RUN dpkg --add-architecture i386 && \
    apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        gcc-multilib \
        git \
        libc6-dev \
        make \
        patch \
        rsync \
        xz-utils

ARG MUSL_VERSION
COPY . /ci
RUN ci/install-musl.sh i686 "$MUSL_VERSION"

ENV PATH=$PATH:/musl-i686/bin:/rust/bin \
    CC_i686_unknown_linux_musl=musl-gcc \
    EXTRA_RUSTFLAGS="-L /musl-i686/lib"
