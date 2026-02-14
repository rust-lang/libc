FROM ubuntu:25.10

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

ARG MUSL_VERSION
COPY install-musl.sh /
RUN /install-musl.sh x86_64 "$MUSL_VERSION"

ENV PATH=$PATH:/musl-x86_64/bin:/rust/bin \
    EXTRA_RUSTFLAGS="-L /musl-x86_64/lib"
