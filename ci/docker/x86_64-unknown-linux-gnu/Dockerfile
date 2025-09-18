FROM ubuntu:24.10

RUN cat /etc/apt/sources.list.d/ubuntu.sources
RUN cat /etc/apt/sources.list
RUN sed -i 's|ports.ubuntu.com/ubuntu-ports/|old-releases.ubuntu.com/ubuntu/|g' \
    /etc/apt/sources.list.d/ubuntu.sources
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    gcc g++ libc6-dev ca-certificates linux-headers-generic

RUN apt search linux-headers
RUN ls /usr/src

ENV PATH=$PATH:/rust/bin
