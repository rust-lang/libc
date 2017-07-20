FROM ubuntu:16.10
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  gcc libc6-dev ca-certificates \
  qemu-user wget build-essential file git autoconf gcc gperf bison flex texinfo help2man gawk libncurses5-dev


RUN git clone https://github.com/crosstool-ng/crosstool-ng
RUN cd /crosstool-ng && ./bootstrap && ./configure && make && make install

RUN useradd build

RUN mkdir /toolchain && chown build /toolchain
RUN mkdir /x-tools && chown build /x-tools

USER build
WORKDIR /toolchain
ADD config /toolchain/.config
RUN ct-ng build

USER root

RUN apt-get install curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH /root/.cargo/bin/:$PATH

RUN rustup install stable
RUN rustup install nightly && rustup run nightly cargo install xargo && \
    rustup default nightly && rustup component add rust-src && rustup default stable

COPY toolchain.patch /toolchain.patch

RUN cd /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/liblibc/src/unix && \
    patch < /toolchain.patch


RUN git clone https://github.com/rust-lang/libc /rust-libc
WORKDIR /rust-libc

COPY arm-unknown-linux-uclibcgnueabihf.json /rust-libc/arm-unknown-linux-uclibcgnueabihf.json
COPY tests.sh /rust-libc/tests.sh

