FROM ubuntu:23.10

# FIXME(time): we are using an EOL release because 24.04 changes to 64-bit time
RUN sed -i -E 's/(archive|security)\.ubuntu\.com/old-releases.ubuntu.com/g' \
        /etc/apt/sources.list && \
    apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev qemu-user ca-certificates \
        gcc-powerpc-linux-gnu libc6-dev-powerpc-cross \
        qemu-system-ppc

ENV CARGO_TARGET_POWERPC_UNKNOWN_LINUX_GNU_LINKER=powerpc-linux-gnu-gcc \
    CARGO_TARGET_POWERPC_UNKNOWN_LINUX_GNU_RUNNER="qemu-ppc -L /usr/powerpc-linux-gnu" \
    CC=powerpc-linux-gnu-gcc \
    PATH=$PATH:/rust/bin
