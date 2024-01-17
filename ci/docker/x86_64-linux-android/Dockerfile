FROM ubuntu:23.10

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
  ca-certificates \
  wget \
  gcc \
  libc-dev \
  python3 \
  python3-distutils \
  unzip

WORKDIR /android/
ENV ANDROID_ARCH=x86_64
COPY android-install-ndk.sh /android/
RUN sh /android/android-install-ndk.sh

# We do not run x86_64-linux-android tests on an android emulator.
# See ci/android-sysimage.sh for information about how tests are run.
COPY android-sysimage.sh /android/
RUN bash /android/android-sysimage.sh x86_64 x86_64-24_r07.zip

ENV PATH=$PATH:/rust/bin:/android/linux-x86_64/bin \
    CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=x86_64-linux-android28-clang \
    CC_x86_64_linux_android=x86_64-linux-android28-clang \
    CXX_x86_64_linux_android=x86_64-linux-android28-clang++ \
    AR_x86_64_linux_android=llvm-ar \
    HOME=/tmp
