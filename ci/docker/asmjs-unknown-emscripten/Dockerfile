FROM ubuntu:23.10

# This is a workaround to avoid the interaction with tzdata.
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=America/New_York

RUN apt-get update
RUN apt-get install -y --no-install-recommends tzdata
RUN apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gcc \
    git \
    libc6-dev \
    libxml2 \
    python3 \
    python3-distutils \
    xz-utils \
    bzip2

COPY emscripten.sh /
RUN bash /emscripten.sh

ENV PATH=$PATH:/rust/bin \
    CARGO_TARGET_ASMJS_UNKNOWN_EMSCRIPTEN_RUNNER=node

# `-g4` is used by default which causes a linking error.
# Using `-g3` not to generate a source map.
ENV EMCC_CFLAGS=-g3

COPY emscripten-entry.sh /
ENTRYPOINT ["/emscripten-entry.sh"]
