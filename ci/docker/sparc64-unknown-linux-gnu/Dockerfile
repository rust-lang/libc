FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    cpio \
    curl \
    gcc \
    gcc-sparc64-linux-gnu \
    ipxe-qemu \
    libc6-dev \
    libc6-dev-sparc64-cross \
    linux-libc-dev-sparc64-cross \
    openbios-sparc \
    p7zip-full \
    qemu-system-sparc64 \
    qemu-user \
    seabios

ENV CARGO_TARGET_SPARC64_UNKNOWN_LINUX_GNU_LINKER=sparc64-linux-gnu-gcc \
    CARGO_TARGET_SPARC64_UNKNOWN_LINUX_GNU_RUNNER="qemu-sparc64 -L /usr/sparc64-linux-gnu" \
    CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc \
    PATH=$PATH:/rust/bin
