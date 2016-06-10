FROM ubuntu:15.10

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
        software-properties-common gcc libc6-dev qemu-user
RUN add-apt-repository ppa:angelsl/mips-cross
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
        gcc-5-mips-linux-gnu libc6-dev-mips-cross

ENV CARGO_TARGET_MIPS_UNKNOWN_LINUX_GNU_LINKER=mips-linux-gnu-gcc-5 \
    CC_mips_unknown_linux_gnu=mips-linux-gnu-gcc-5 \
    PATH=$PATH:/rust/bin
