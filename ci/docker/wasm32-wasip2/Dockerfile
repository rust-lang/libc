FROM ubuntu:25.04

COPY wasi.sh /
RUN /wasi.sh

# Note that most of these are copied from `wasm32-wasip1/Dockerfile`
#
ENV CARGO_TARGET_WASM32_WASIP2_RUNNER=wasmtime \
    CARGO_TARGET_WASM32_WASIP2_LINKER=/opt/wasi-sdk/bin/clang \
    CARGO_TARGET_WASM32_WASIP2_RUSTFLAGS="-lwasi-emulated-process-clocks -Ctarget-feature=-crt-static -Clink-arg=-Wl" \
    CC_wasm32_wasip2=/opt/wasi-sdk/bin/clang \
    CFLAGS_wasm32_wasip2=-D_WASI_EMULATED_PROCESS_CLOCKS \
    PATH=$PATH:/rust/bin:/wasmtime
