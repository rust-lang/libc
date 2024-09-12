FROM ubuntu:24.10

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  gcc make libc6-dev git curl ca-certificates \
  xz-utils patch rsync

COPY install-musl.sh /
RUN /install-musl.sh x86_64

ENV PATH=$PATH:/musl-x86_64/bin:/rust/bin \
    RUSTFLAGS="-L /musl-x86_64/lib"
