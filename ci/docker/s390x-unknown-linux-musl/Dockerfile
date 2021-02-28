FROM ubuntu:20.04

RUN apt-get update && apt-get install -y --no-install-recommends \
        curl ca-certificates \
        gcc \
        gcc-s390x-linux-gnu \
        qemu-user

COPY install-musl.sh /
RUN sh /install-musl.sh s390x

# FIXME: shouldn't need the `-lgcc` here, shouldn't that be in libstd?
ENV CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_LINKER=s390x-linux-gnu-gcc \
    CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_RUNNER="qemu-s390x -L /musl-s390x" \
    CC_s390x_unknown_linux_gnu=musl-gcc \
    RUSTFLAGS='-Clink-args=-lgcc' \
    PATH=$PATH:/musl-s390x/bin:/rust/bin
