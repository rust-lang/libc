FROM huyuumi/libc-switch:latest

RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc libc6-dev ca-certificates

ENV PATH=$PATH:/rust/bin
