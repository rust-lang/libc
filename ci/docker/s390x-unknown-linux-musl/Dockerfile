FROM ubuntu:25.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  gcc make libc6-dev git curl ca-certificates \
  gcc-s390x-linux-gnu qemu-user xz-utils patch rsync

# Set up an Alpine sysroot for acquiring libgcc and libgcc_s
RUN curl https://repo.chimera-linux.org/apk/3.0.0_rc4-r2/apk-x86_64.static -o /usr/bin/apk && \
  chmod +x /usr/bin/apk && \
  mkdir /alpine-sysroot && \
  apk \
    --arch s390x \
    --root /alpine-sysroot \
    --repository https://dl-cdn.alpinelinux.org/alpine/v3.22/main \
    add \
    --initdb \
    --allow-untrusted \
    --no-scripts \
    alpine-baselayout libgcc

COPY install-musl.sh /
RUN /install-musl.sh s390x

# FIXME: shouldn't need the `-lgcc` here, shouldn't that be in std?
ENV PATH=$PATH:/musl-s390x/bin:/rust/bin \
    CC_s390x_unknown_linux_musl=musl-gcc \
    RUSTFLAGS='-Clink-args=-lgcc -L/alpine-sysroot/usr/lib -L /musl-s390x/lib' \
    CARGO_TARGET_S390X_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
    CARGO_TARGET_S390X_UNKNOWN_LINUX_MUSL_RUNNER="qemu-s390x -L /musl-s390x"
