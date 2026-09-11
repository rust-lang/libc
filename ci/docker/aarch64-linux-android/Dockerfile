FROM debian:13

RUN apt-get update && apt-get install -y --no-install-recommends \
    adb \
    ca-certificates \
    curl \
    gcc \
    libasound2t64 \
    libc-dev \
    libpulse0 \
    python3 \
    unzip \
    wget

# Cuttlefish host packages: the virtual device the tests run on boots
# inside this container (see cuttlefish-entrypoint.sh for the docker run
# flags this needs). These are x86_64 binaries running an arm64 guest, so
# no qemu package is needed here: the host package fetched by
# cuttlefish-setup.sh brings its own qemu-system-aarch64. That binary does
# link against libasound/libpulse though, which the base image does not carry
# and crosvm never needed, so they are installed above (t64 names, as Debian
# renamed them); without them qemu cannot start and the device just reports
# VIRTUAL_DEVICE_BOOT_FAILED.
RUN curl -fsSL https://us-apt.pkg.dev/doc/repo-signing-key.gpg \
        -o /etc/apt/trusted.gpg.d/artifact-registry.asc \
    && echo "deb https://us-apt.pkg.dev/projects/android-cuttlefish-artifacts android-cuttlefish main" \
        >/etc/apt/sources.list.d/artifact-registry.list \
    && apt-get update && apt-get install -y cuttlefish-base cuttlefish-user

WORKDIR /android/
ENV ANDROID_ARCH=aarch64
# The API level we compile against: this only names the CC wrapper and the toolchain we use.
# The Android Cuttlefish device is pinned separately and is newer, i.e. capable of testing
# newer APIs. Held at the level the retired emulator ran so the version-gated test skips
# resolve exactly as before; raising it widens coverage and is a separate change.
ARG ANDROID_API=24
COPY android-install-ndk.sh /android/
RUN /android/android-install-ndk.sh

ENV PATH=$PATH:/rust/bin:/android/linux-x86_64/bin \
    ANDROID_SERIAL=127.0.0.1:6520 \
    CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android${ANDROID_API}-clang \
    CARGO_TARGET_AARCH64_LINUX_ANDROID_RUNNER=/tmp/runtest \
    CC_aarch64_linux_android=aarch64-linux-android${ANDROID_API}-clang \
    CXX_aarch64_linux_android=aarch64-linux-android${ANDROID_API}-clang++ \
    AR_aarch64_linux_android=llvm-ar \
    HOME=/tmp

# The device images are arm64 but every host tool running them must be x86_64, so the
# host package comes from the x86_64 build of the same drop. crosvm cannot run a
# foreign-architecture guest; qemu_cli can.
ENV CUTTLEFISH_TARGET=aosp_cf_arm64_only_phone-userdebug \
    CUTTLEFISH_HOST_TARGET=aosp_cf_x86_64_only_phone-userdebug \
    CUTTLEFISH_VM_MANAGER=qemu_cli

ADD runtest-android.rs /tmp/runtest.rs
COPY cuttlefish-setup.sh cuttlefish-entrypoint.sh /android/
ENTRYPOINT ["/android/cuttlefish-entrypoint.sh"]
