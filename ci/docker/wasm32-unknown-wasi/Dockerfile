FROM ubuntu:18.04 as reference-sysroot

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    clang \
    cmake \
    curl \
    g++ \
    git \
    libc6-dev \
    libclang-dev \
    make \
    ssh \
    xz-utils

RUN curl http://releases.llvm.org/8.0.0/clang+llvm-8.0.0-x86_64-linux-gnu-ubuntu-18.04.tar.xz | tar xJf -
RUN mv /clang+llvm-8.0.0-x86_64-linux-gnu-ubuntu-18.04 /wasmcc

RUN git clone https://github.com/CraneStation/reference-sysroot-wasi && \
  cd reference-sysroot-wasi && \
  git reset --hard d5a609fe63926533e1054e539ba5f2693d51bdf5
RUN make -C reference-sysroot-wasi install -j $(nproc) WASM_CC=/wasmcc/bin/clang INSTALL_DIR=/wasm-sysroot
COPY docker/wasm32-unknown-wasi/clang.sh /wasm-sysroot/bin/clang

FROM ubuntu:18.04 as wasmtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    clang \
    cmake \
    curl \
    g++ \
    git \
    libclang-dev \
    make \
    ssh

RUN curl -sSf https://sh.rustup.rs |  sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

RUN apt-get install -y --no-install-recommends python
RUN git clone https://github.com/CraneStation/wasmtime-wasi wasmtime && \
  cd wasmtime && \
  git reset --hard a7ac05df74759a7536b2b1e30adc6ff4867e36c3

# Install wasmtime in /usr/bin, but make sure to remove rust afterwards because
# we don't want it conflicting with the main Rust we're using to compile
# `libc`.
RUN cargo build --release --manifest-path wasmtime/Cargo.toml

FROM ubuntu:18.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    gcc \
    libc6-dev \
    libxml2

COPY --from=reference-sysroot /wasmcc /wasmcc/
COPY --from=reference-sysroot /wasm-sysroot/ /wasm-sysroot/
COPY --from=wasmtime /wasmtime/target/release/wasmtime /usr/bin/

ENV CARGO_TARGET_WASM32_UNKNOWN_WASI_RUNNER=wasmtime \
  CARGO_TARGET_WASM32_UNKNOWN_WASI_LINKER=/wasm-sysroot/bin/clang \
  CC_wasm32_unknown_wasi=/wasm-sysroot/bin/clang \
  PATH=$PATH:/rust/bin \
  RUSTFLAGS=-Ctarget-feature=-crt-static
