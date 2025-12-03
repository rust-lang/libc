FROM ubuntu:25.04

COPY wasi.sh /
RUN /wasi.sh

# Note that most of these are copied from `wasm32-wasip1/Dockerfile`
#
# FIXME: the `-Clink-arg` to export `cabi_realloc` is a bug in the target
# itself, this should be fixed upstream.
ENV CARGO_TARGET_WASM32_WASIP2_RUNNER=wasmtime \
    CARGO_TARGET_WASM32_WASIP2_LINKER=/opt/wasi-sdk/bin/clang \
    EXTRA_RUSTFLAGS="-Clink-self-contained=n -lwasi-emulated-process-clocks -Clink-arg=-Wl,--export,cabi_realloc" \
    CC_wasm32_wasip2=/opt/wasi-sdk/bin/clang \
    CFLAGS_wasm32_wasip2=-D_WASI_EMULATED_PROCESS_CLOCKS \
    PATH=$PATH:/rust/bin:/wasmtime
