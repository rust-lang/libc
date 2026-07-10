FROM ubuntu:26.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    adb \
    ca-certificates \
    curl \
    gcc \
    libc-dev \
    python3 \
    unzip \
    wget

# Cuttlefish host packages: the virtual device the tests run on boots
# inside this container (see cuttlefish-entrypoint.sh for the docker run
# flags this needs).
RUN curl -fsSL https://us-apt.pkg.dev/doc/repo-signing-key.gpg \
        -o /etc/apt/trusted.gpg.d/artifact-registry.asc \
    && echo "deb https://us-apt.pkg.dev/projects/android-cuttlefish-artifacts android-cuttlefish main" \
        >/etc/apt/sources.list.d/artifact-registry.list \
    && apt-get update && apt-get install -y cuttlefish-base cuttlefish-user

WORKDIR /android/
ENV ANDROID_ARCH=x86_64
COPY android-install-ndk.sh /android/
RUN /android/android-install-ndk.sh

ENV PATH=$PATH:/rust/bin:/android/linux-x86_64/bin \
    ANDROID_SERIAL=127.0.0.1:6520 \
    CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=x86_64-linux-android24-clang \
    CARGO_TARGET_X86_64_LINUX_ANDROID_RUNNER=/tmp/runtest \
    CC_x86_64_linux_android=x86_64-linux-android24-clang \
    CXX_x86_64_linux_android=x86_64-linux-android24-clang++ \
    AR_x86_64_linux_android=llvm-ar \
    HOME=/tmp

ADD runtest-android.rs /tmp/runtest.rs
COPY cuttlefish-setup.sh docker/x86_64-linux-android/cuttlefish-entrypoint.sh /android/
ENTRYPOINT ["/android/cuttlefish-entrypoint.sh"]
