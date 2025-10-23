FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
        curl ca-certificates \
        gcc libc6-dev \
        gcc-sparc64-linux-gnu libc6-dev-sparc64-cross \
        qemu-system-sparc64 openbios-sparc seabios ipxe-qemu \
        p7zip-full cpio linux-libc-dev-sparc64-cross qemu-user

ENV CARGO_TARGET_SPARC64_UNKNOWN_LINUX_GNU_LINKER=sparc64-linux-gnu-gcc \
    CARGO_TARGET_SPARC64_UNKNOWN_LINUX_GNU_RUNNER="qemu-sparc64 -L /usr/sparc64-linux-gnu" \
    CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc \
    PATH=$PATH:/rust/bin
