FROM ubuntu:17.10

RUN apt-get update
RUN apt-get install -y --no-install-recommends software-properties-common
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/ppa
RUN add-apt-repository 'deb http://ppa.launchpad.net/ubuntu-toolchain-r/ppa/ubuntu artful main'
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
        gcc libc6-dev qemu-user ca-certificates \
        gcc-powerpc64-linux-gnu libc6-dev-ppc64-cross \
        qemu-system-ppc

ENV CARGO_TARGET_POWERPC64_UNKNOWN_LINUX_GNU_LINKER=powerpc64-linux-gnu-gcc \
    CC=powerpc64-linux-gnu-gcc \
    PATH=$PATH:/rust/bin
