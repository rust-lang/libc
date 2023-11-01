FROM ubuntu:23.10

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  gcc make libc6-dev git curl ca-certificates

COPY install-musl.sh /
RUN sh /install-musl.sh x86_64

ENV PATH=$PATH:/musl-x86_64/bin:/rust/bin \
    RUSTFLAGS="-L /musl-x86_64/lib"
