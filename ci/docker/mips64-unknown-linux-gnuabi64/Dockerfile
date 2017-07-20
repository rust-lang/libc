FROM ubuntu:17.10

RUN apt-get update
RUN apt-get install -y --no-install-recommends software-properties-common
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/ppa
RUN add-apt-repository 'deb http://ppa.launchpad.net/ubuntu-toolchain-r/ppa/ubuntu artful main'
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
        gcc libc6-dev qemu-user ca-certificates \
        gcc-mips64-linux-gnuabi64 libc6-dev-mips64-cross \
        qemu-system-mips64

ENV CARGO_TARGET_MIPS64_UNKNOWN_LINUX_GNUABI64_LINKER=mips64-linux-gnuabi64-gcc \
    CC_mips64_unknown_linux_gnuabi64=mips64-linux-gnuabi64-gcc \
    PATH=$PATH:/rust/bin
