FROM ubuntu:24.10

COPY wasi.sh /
RUN /wasi.sh

# Note that most of these are copied from `wasm32-wasip1/Dockerfile`
#
# FIXME: the `-Clink-arg` to export `cabi_realloc` is a bug in the target
# itself, this should be fixed upstream.
ENV CARGO_TARGET_WASM32_WASIP2_RUNNER=wasmtime \
    CARGO_TARGET_WASM32_WASIP2_LINKER=/opt/wasi-sdk/bin/clang \
    CARGO_TARGET_WASM32_WASIP2_RUSTFLAGS="-lwasi-emulated-process-clocks -Ctarget-feature=-crt-static -Clink-arg=-Wl,--export,cabi_realloc" \
    CC_wasm32_wasip2=/opt/wasi-sdk/bin/clang \
    CFLAGS_wasm32_wasip2=-D_WASI_EMULATED_PROCESS_CLOCKS \
    PATH=$PATH:/rust/bin:/wasmtime
