FROM ubuntu:25.10

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    gcc-powerpc64le-linux-gnu \
    libc6-dev \
    libc6-dev-ppc64el-cross \
    qemu-system-ppc \
    qemu-user

ENV CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_GNU_LINKER=powerpc64le-linux-gnu-gcc \
    CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_GNU_RUNNER="qemu-ppc64le -L /usr/powerpc64le-linux-gnu" \
    CC=powerpc64le-linux-gnu-gcc \
    PATH=$PATH:/rust/bin
