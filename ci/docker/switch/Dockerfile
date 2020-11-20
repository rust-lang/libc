# NOTE: the pacman that we use for this target doesn't support
# to use it on CI and we should pull it from another Docker image.

FROM huyuumi/libc-switch:latest

RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc libc6-dev ca-certificates

ENV PATH=$PATH:/rust/bin
