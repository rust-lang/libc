FROM ubuntu:24.10

RUN sed -i -re 's/([a-z]{2}\.)?archive.ubuntu.com|security.ubuntu.com/old-releases.ubuntu.com/g' /etc/apt/sources.list
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  gcc g++ libc6-dev ca-certificates linux-headers-generic

RUN apt search linux-headers
RUN ls /usr/src

ENV PATH=$PATH:/rust/bin
