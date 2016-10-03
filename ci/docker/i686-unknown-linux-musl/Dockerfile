FROM ubuntu:16.10

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  gcc make libc6-dev git curl ca-certificates
RUN curl https://www.musl-libc.org/releases/musl-1.1.15.tar.gz | \
    tar xzf - && \
    cd musl-1.1.15 && \
    CFLAGS=-m32 ./configure --prefix=/musl-i686 --disable-shared --target=i686 && \
    make install -j4 && \
    cd .. && \
    rm -rf musl-1.1.15
ENV PATH=$PATH:/musl-i686/bin:/rust/bin
