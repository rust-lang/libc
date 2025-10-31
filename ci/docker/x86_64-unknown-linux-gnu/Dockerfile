FROM ubuntu:25.10

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev

RUN apt search linux-headers
RUN ls /usr/src

ENV PATH=$PATH:/rust/bin
