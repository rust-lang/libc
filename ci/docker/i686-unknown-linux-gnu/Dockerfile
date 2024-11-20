FROM ubuntu:23.10


# FIXME(time): we are using an EOL release because 24.04 changes to 64-bit time
RUN sed -i -E 's/(archive|security)\.ubuntu\.com/old-releases.ubuntu.com/g' \
        /etc/apt/sources.list && \
    apt-get update && apt-get install -y --no-install-recommends \
        gcc-multilib libc6-dev ca-certificates

ENV PATH=$PATH:/rust/bin
