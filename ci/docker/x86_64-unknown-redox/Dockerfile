FROM redoxos/redoxer

RUN mv /root/.redoxer /.redoxer

RUN ln -sf /root/.redoxer ~/.redoxer

ENV PATH=$PATH:$HOME/.redoxer/x86_64-unknown-redox/toolchain/bin \
    AR_x86_64_unknown_redox="x86_64-unknown-redox-ar" \
    CC_x86_64_unknown_redox="x86_64-unknown-redox-gcc" \
    CARGO_TARGET_X86_64_UNKNOWN_REDOX_LINKER="x86_64-unknown-redox-gcc" \
    CARGO_TARGET_X86_64_UNKNOWN_REDOX_RUNNER="redoxer exec --folder ."
