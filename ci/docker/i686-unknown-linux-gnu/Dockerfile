# FIXME: Somehow we encounter a panic with Ubuntu 20.04.
# Should investigate why it causes and fix.
FROM ubuntu:19.10
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  gcc-multilib libc6-dev ca-certificates
ENV PATH=$PATH:/rust/bin
