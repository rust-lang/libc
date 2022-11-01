FROM ubuntu:22.04

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
  openjdk-11-jre \
  libstdc++6:i386 \
  libpulse0 \
  android-sdk

WORKDIR /android/
COPY android* /android/

ENV ANDROID_ARCH=i686 \
  ANDROID_HOME=/usr/lib/android-sdk \
  ANDROID_SDK_ROOT=/usr/lib/android-sdk
ENV PATH=$PATH:/android/ndk-$ANDROID_ARCH/bin:/usr/lib/android-sdk/cmdline-tools:/usr/lib/android-sdk/platform-tools

RUN sh /android/android-install-ndk.sh $ANDROID_ARCH
RUN sh /android/android-install-sdk.sh $ANDROID_ARCH
RUN cp -r /root/.android /tmp
RUN chmod 777 -R /root/.android
RUN chmod 777 -R /tmp/.android
RUN chmod 777 -R /usr/lib/android-sdk
RUN chmod 755 /usr/lib/android-sdk/cmdline-tools/* /usr/lib/android-sdk/emulator/qemu/linux-x86_64/*

ENV PATH=$PATH:/rust/bin \
    CARGO_TARGET_I686_LINUX_ANDROID_LINKER=i686-linux-android-gcc \
    CARGO_TARGET_I686_LINUX_ANDROID_RUNNER=/tmp/runtest \
    CC_i686_linux_android=i686-linux-android-gcc \
    HOME=/tmp

ADD runtest-android.rs /tmp/runtest.rs
ENTRYPOINT [ \
  "bash", \
  "-c", \
  # set SHELL so android can detect a 64bits system, see
  # http://stackoverflow.com/a/41789144
  "SHELL=/bin/dash /usr/lib/android-sdk/emulator/emulator @i686 -no-window -no-accel & \
   rustc /tmp/runtest.rs -o /tmp/runtest && \
   exec \"$@\"", \
  "--" \
]
