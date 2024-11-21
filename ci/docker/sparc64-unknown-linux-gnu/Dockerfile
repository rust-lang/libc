# FIXME(sparc): newer versions of Ubuntu get the following errors
# ```
# /prog: /lib/sparc64-linux-gnu/libm.so.6: version `GLIBC_2.38' not found (required by /prog)
# /prog: /lib/sparc64-linux-gnu/libc.so.6: version `GLIBC_2.39' not found (required by /prog)
# ```
# Not sure if this is a problem from rustc, our libc, or Ubuntu so we just
# stick with an old LTS for now.
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y --no-install-recommends \
        curl ca-certificates \
        gcc libc6-dev \
        gcc-sparc64-linux-gnu libc6-dev-sparc64-cross \
        qemu-system-sparc64 openbios-sparc seabios ipxe-qemu \
        p7zip-full cpio linux-libc-dev-sparc64-cross qemu-user

COPY linux-sparc64.sh /
RUN /linux-sparc64.sh

ENV CARGO_TARGET_SPARC64_UNKNOWN_LINUX_GNU_LINKER=sparc64-linux-gnu-gcc \
    CARGO_TARGET_SPARC64_UNKNOWN_LINUX_GNU_RUNNER="qemu-sparc64 -L /usr/sparc64-linux-gnu" \
    CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc \
    PATH=$PATH:/rust/bin
