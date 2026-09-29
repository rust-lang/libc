FROM ubuntu:26.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    file \
    gcc-multilib \
    libc6-dev

ENV PATH=$PATH:/rust/bin
