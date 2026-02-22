FROM ubuntu:25.10

# This is a workaround to avoid the interaction with tzdata.
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=America/New_York

RUN apt-get update && apt-get install -y --no-install-recommends \
    bzip2 \
    ca-certificates \
    curl \
    gcc \
    git \
    libc6-dev \
    libxml2 \
    python3 \
    tzdata \
    xz-utils

COPY emscripten.sh /
RUN /emscripten.sh

ENV PATH=$PATH:/rust/bin \
    CARGO_TARGET_ASMJS_UNKNOWN_EMSCRIPTEN_RUNNER=node

# `-g4` is used by default which causes a linking error.
# Using `-g3` not to generate a source map.
ENV EMCC_CFLAGS=-g3

COPY emscripten-entry.sh /
ENTRYPOINT ["/emscripten-entry.sh"]
