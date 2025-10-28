FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc-multilib \
    libc6-dev

ENV PATH=$PATH:/rust/bin
