FROM ubuntu:23.10

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    clang \
    curl \
    git \
    libc6-dev \
    make \
    xz-utils

# Note that we're using `git reset --hard` to pin to a specific commit for
# verification for now. The sysroot is currently in somewhat of a state of flux
# and is expected to have breaking changes, so this is an attempt to mitigate
# those breaking changes on `libc`'s own CI
RUN git clone https://github.com/WebAssembly/wasi-libc && \
  cd wasi-libc && \
  git reset --hard ad5133410f66b93a2381db5b542aad5e0964db96
RUN apt-get install -y --no-install-recommends llvm
RUN make -C wasi-libc install -j $(nproc) INSTALL_DIR=/wasi-libc

RUN curl -L https://github.com/bytecodealliance/wasmtime/releases/download/dev/wasmtime-dev-x86_64-linux.tar.xz | \
  tar xJf -
ENV PATH=$PATH:/wasmtime-dev-x86_64-linux
COPY docker/wasm32-wasi/clang.sh /wasi-libc/bin/clang

RUN apt-get install -y --no-install-recommends lld
RUN ln -s /usr/bin/wasm-ld-10 /usr/bin/wasm-ld
ENV PATH=$PATH:/usr/lib/llvm-10/bin

# Of note here is our clang wrapper which just executes a normal clang
# executable with the right sysroot, and then we're sure to turn off the
# crt-static feature to ensure that the CRT that we're specifying with `clang`
# is used.
ENV CARGO_TARGET_WASM32_WASI_RUNNER=wasmtime \
  CARGO_TARGET_WASM32_WASI_LINKER=/wasi-libc/bin/clang \
  CC_wasm32_wasi=/wasi-libc/bin/clang \
  PATH=$PATH:/rust/bin \
  RUSTFLAGS=-Ctarget-feature=-crt-static
