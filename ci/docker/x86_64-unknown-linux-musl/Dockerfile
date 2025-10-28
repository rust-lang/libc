FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gcc \
    git \
    libc6-dev \
    make \
    patch \
    rsync \
    xz-utils

COPY install-musl.sh /
RUN /install-musl.sh x86_64

ENV PATH=$PATH:/musl-x86_64/bin:/rust/bin \
    RUSTFLAGS="-L /musl-x86_64/lib"
