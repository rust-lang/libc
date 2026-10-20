FROM ubuntu:26.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    e2fsprogs \
    erofs-utils \
    file \
    gcc \
    libc-dev \
    python3 \
    qemu-user \
    unzip \
    wget

WORKDIR /android/
COPY android-install-ndk.sh android-bionic-sysroot.sh /android/
# The API level we compile against: this only names the CC wrapper and the
# toolchain we use. The bionic extracted by android-bionic-sysroot.sh is
# pinned separately and is newer, i.e. capable of testing newer APIs. The
# toolchain wrapper is held at the same baseline as arm/x86_64 for now.
ARG ANDROID_API=24
RUN /android/android-install-ndk.sh
RUN /android/android-bionic-sysroot.sh

# Test binaries run under qemu-user against the bionic extracted to
# /system by android-bionic-sysroot.sh. This is a stopgap until GitHub's
# arm64 runners expose KVM, at which point we can move to Cuttlefish like
# the x86_64-linux-android test.
ENV PATH=$PATH:/rust/bin:/android/linux-x86_64/bin \
    CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android${ANDROID_API}-clang \
    CARGO_TARGET_AARCH64_LINUX_ANDROID_RUNNER=qemu-aarch64 \
    CC_aarch64_linux_android=aarch64-linux-android${ANDROID_API}-clang \
    AR_aarch64_linux_android=llvm-ar \
    HOME=/tmp
