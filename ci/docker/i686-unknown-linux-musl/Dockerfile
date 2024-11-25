FROM ubuntu:23.10

# FIXME(time): we are using an EOL release because 24.04 changes to 64-bit time
RUN sed -i -E 's/(archive|security)\.ubuntu\.com/old-releases.ubuntu.com/g' \
        /etc/apt/sources.list && \
    dpkg --add-architecture i386 && \
    apt-get update && apt-get install -y --no-install-recommends \
        gcc-multilib make libc6-dev git curl ca-certificates libc6-i386 \
        xz-utils patch rsync

COPY install-musl.sh /
RUN /install-musl.sh i686

ENV PATH=$PATH:/musl-i686/bin:/rust/bin \
    CC_i686_unknown_linux_musl=musl-gcc \
    RUSTFLAGS="-L /musl-i686/lib"
