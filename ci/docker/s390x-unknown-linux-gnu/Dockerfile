FROM ubuntu:26.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    gcc-s390x-linux-gnu \
    libc6-dev \
    libc6-dev-s390x-cross \
    qemu-system-s390x \
    qemu-user

ENV CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_LINKER=s390x-linux-gnu-gcc \
    CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_RUNNER="qemu-s390x -L /usr/s390x-linux-gnu" \
    CC_s390x_unknown_linux_gnu=s390x-linux-gnu-gcc \
    PATH=$PATH:/rust/bin
