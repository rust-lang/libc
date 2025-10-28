FROM ubuntu:23.10

# FIXME(time): we are using an EOL release because 24.04 changes to 64-bit time
RUN sed -i -E 's/(archive|security)\.ubuntu\.com/old-releases.ubuntu.com/g' \
        /etc/apt/sources.list && \
    dpkg --add-architecture i386 && \
    apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        gcc-multilib \
        git \
        libc6-dev \
        libc6-i386 \
        make \
        patch \
        rsync \
        xz-utils

COPY install-musl.sh /
RUN /install-musl.sh i686

ENV PATH=$PATH:/musl-i686/bin:/rust/bin \
    CC_i686_unknown_linux_musl=musl-gcc \
    RUSTFLAGS="-L /musl-i686/lib"
