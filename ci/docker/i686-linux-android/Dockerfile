FROM ubuntu:23.10

RUN dpkg --add-architecture i386
RUN apt-get update
RUN apt-get install -y --no-install-recommends libc6-dev gcc
RUN apt-get install -y --no-install-recommends \
  file \
  wget \
  ca-certificates \
  python3 \
  python3-distutils \
  unzip \
  expect \
  openjdk-8-jre \
  libstdc++6:i386 \
  libpulse0

WORKDIR /android/
COPY android* /android/

ENV ANDROID_ARCH=i686
ENV PATH=$PATH:/android/linux-x86_64/bin:/android/sdk/cmdline-tools/tools:/android/sdk/platform-tools

RUN sh /android/android-install-ndk.sh
RUN sh /android/android-install-sdk.sh $ANDROID_ARCH
RUN mv /root/.android /tmp
RUN chmod 777 -R /tmp/.android
RUN chmod 755 /android/sdk/cmdline-tools/tools/* /android/sdk/emulator/qemu/linux-x86_64/*

ENV PATH=$PATH:/rust/bin \
    CARGO_TARGET_I686_LINUX_ANDROID_LINKER=i686-linux-android28-clang \
    CARGO_TARGET_I686_LINUX_ANDROID_RUNNER=/tmp/runtest \
    CC_i686_linux_android=i686-linux-android28-clang \
    AR_i686_linux_android=llvm-ar \
    HOME=/tmp

ADD runtest-android.rs /tmp/runtest.rs
ENTRYPOINT [ \
  "bash", \
  "-c", \
  # set SHELL so android can detect a 64bits system, see
  # http://stackoverflow.com/a/41789144
  "SHELL=/bin/dash /android/sdk/emulator/emulator @i686 -no-window -no-accel & \
   rustc /tmp/runtest.rs -o /tmp/runtest && \
   exec \"$@\"", \
  "--" \
]
