FROM ubuntu:24.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    clang \
    xz-utils

# Wasmtime is used to execute tests and wasi-sdk is used to compile tests.
# Download appropriate versions here and configure various flags below.
ENV WASMTIME 24.0.0
ENV WASI_SDK 24

RUN curl -L https://github.com/bytecodealliance/wasmtime/releases/download/v$WASMTIME/wasmtime-v$WASMTIME-x86_64-linux.tar.xz | \
  tar xJf -
ENV PATH=$PATH:/wasmtime-v$WASMTIME-x86_64-linux

RUN curl -LO https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-$WASI_SDK/wasi-sdk-$WASI_SDK.0-x86_64-linux.deb
RUN dpkg -i ./wasi-sdk-*.deb

# Note that `-D_WASI_EMULATED_PROCESS_CLOCKS` is used to enable access to
# clock-related defines even though they're emulated. Also note that the usage
# of `-Ctarget-feature=-crt-static` here forces usage of the external wasi-libc
# installed via `wasi-sdk` instead of the version that comes with the standard
# library.
ENV CARGO_TARGET_WASM32_WASIP1_RUNNER=wasmtime \
    CARGO_TARGET_WASM32_WASIP1_LINKER=/opt/wasi-sdk/bin/clang \
    CARGO_TARGET_WASM32_WASIP1_RUSTFLAGS="-lwasi-emulated-process-clocks -Ctarget-feature=-crt-static" \
    CC_wasm32_wasip1=/opt/wasi-sdk/bin/clang \
    CFLAGS_wasm32_wasip1=-D_WASI_EMULATED_PROCESS_CLOCKS \
    PATH=$PATH:/rust/bin
