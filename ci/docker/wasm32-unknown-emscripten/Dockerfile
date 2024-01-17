FROM ubuntu:23.10

# This is a workaround to avoid the interaction with tzdata.
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=America/New_York

RUN apt-get update
RUN apt-get install -y --no-install-recommends tzdata
RUN apt-get install -y --no-install-recommends \
    ca-certificates \
    g++ \
    make \
    file \
    curl \
    gcc \
    git \
    libc6-dev \
    libxml2 \
    python3 \
    python3-distutils \
    cmake \
    sudo \
    gdb \
    xz-utils \
    bzip2

RUN ln -s /usr/bin/python3 /usr/bin/python & \
    ln -s /usr/bin/pip3 /usr/bin/pip
COPY emscripten.sh /
RUN bash /emscripten.sh

ENV PATH=$PATH:/rust/bin \
    CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_RUNNER=node-wrapper.sh

COPY emscripten-entry.sh /
COPY docker/wasm32-unknown-emscripten/node-wrapper.sh /usr/local/bin/node-wrapper.sh
ENTRYPOINT ["/emscripten-entry.sh"]
