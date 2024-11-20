FROM ubuntu:24.10

COPY wasi.sh /
RUN /wasi.sh

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
    PATH=$PATH:/rust/bin:/wasmtime
