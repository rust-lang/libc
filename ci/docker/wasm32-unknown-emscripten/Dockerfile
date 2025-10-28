FROM ubuntu:25.04

# This is a workaround to avoid the interaction with tzdata.
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=America/New_York

RUN apt-get update && apt-get install -y --no-install-recommends \
    bzip2 \
    ca-certificates \
    cmake \
    curl \
    file \
    g++ \
    gcc \
    gdb \
    git \
    libc6-dev \
    libxml2 \
    make \
    python3 \
    tzdata \
    sudo \
    xz-utils

RUN ln -s /usr/bin/python3 /usr/bin/python & \
    ln -s /usr/bin/pip3 /usr/bin/pip
COPY emscripten.sh /
RUN /emscripten.sh

ENV PATH=$PATH:/rust/bin \
    CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_RUNNER=node-wrapper.sh

COPY emscripten-entry.sh /
COPY docker/wasm32-unknown-emscripten/node-wrapper.sh /usr/local/bin/node-wrapper.sh
ENTRYPOINT ["/emscripten-entry.sh"]
