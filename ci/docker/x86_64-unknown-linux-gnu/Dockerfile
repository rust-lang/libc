FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    g++ \
    gcc \
    libc6-dev \
    linux-headers-generic

RUN apt search linux-headers
RUN ls /usr/src

ENV PATH=$PATH:/rust/bin
