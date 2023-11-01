FROM ubuntu:23.10

RUN apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev qemu-user ca-certificates \
        gcc-riscv64-linux-gnu libc6-dev-riscv64-cross \
        qemu-system-riscv64 linux-headers-generic

ENV CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER=riscv64-linux-gnu-gcc \
    CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_RUNNER="qemu-riscv64 -L /usr/riscv64-linux-gnu" \
    CC_riscv64gc_unknown_linux_gnu=riscv64-linux-gnu-gcc \
    CFLAGS_riscv64gc_unknown_linux_gnu="-mabi=lp64d -fPIC" \
    PATH=$PATH:/rust/bin
