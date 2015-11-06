# Entry point for all travis builds, this will set up the Travis environment by
# downloading any dependencies. It will then execute the `run.sh` script to
# build and execute all tests.

set -ex

if [ "$TRAVIS_OS_NAME" = "linux" ]; then
  OS=unknown-linux-gnu
else
  OS=apple-darwin
fi

export HOST=$ARCH-$OS
if [ "$TARGET" = "" ]; then
  TARGET=$HOST
fi

MAIN_TARGETS=https://static.rust-lang.org/dist
EXTRA_TARGETS=https://people.mozilla.org/~acrichton/libc-test/2015-09-08

install() {
  sudo apt-get update
  sudo apt-get install -y $@
}

case "$TARGET" in
  *-apple-ios)
    curl -s $EXTRA_TARGETS/$TARGET.tar.gz | tar xzf - -C $HOME/rust/lib/rustlib
    ;;

  *)
    # Download the rustlib folder from the relevant portion of main distribution's
    # tarballs.
    dir=rust-std-$TARGET
    pkg=rust-std
    if [ "$TRAVIS_RUST_VERSION" = "1.0.0" ]; then
      pkg=rust
      dir=rustc
    fi
    curl -s $MAIN_TARGETS/$pkg-$TRAVIS_RUST_VERSION-$TARGET.tar.gz | \
      tar xzf - -C $HOME/rust/lib/rustlib --strip-components=4 \
        $pkg-$TRAVIS_RUST_VERSION-$TARGET/$dir/lib/rustlib/$TARGET
    ;;

esac

case "$TARGET" in
  # Pull a pre-built docker image for testing android, then run tests entirely
  # within that image. Note that this is using the same rustc installation that
  # travis has (sharing it via `-v`) and otherwise the tests run entirely within
  # the container.
  arm-linux-androideabi)
    script="
cp -r /checkout/* .
mkdir .cargo
cp ci/cargo-config .cargo/config
exec sh ci/run.sh $TARGET
"
    exec docker run \
      --entrypoint bash \
      -v $HOME/rust:/usr/local:ro \
      -v `pwd`:/checkout:ro \
      -e LD_LIBRARY_PATH=/usr/local/lib \
      -it alexcrichton/rust-slave-android:2015-10-21 \
      -c "$script"
    ;;

  x86_64-unknown-linux-musl)
    install musl-tools
    export CC=musl-gcc
    ;;

  arm-unknown-linux-gnueabihf)
    install gcc-4.7-arm-linux-gnueabihf qemu-user
    export CC=arm-linux-gnueabihf-gcc-4.7
    ;;

  aarch64-unknown-linux-gnu)
    install gcc-aarch64-linux-gnu qemu-user
    export CC=aarch64-linux-gnu-gcc
    ;;

  *-apple-ios)
    ;;

  mips-unknown-linux-gnu)
    # Download pre-built and custom MIPS libs and then also instsall the MIPS
    # compiler according to this post:
    # http://sathisharada.blogspot.com/2014_10_01_archive.html
    echo 'deb http://ftp.de.debian.org/debian squeeze main' | \
      sudo tee -a /etc/apt/sources.list
    echo 'deb http://www.emdebian.org/debian/ squeeze main' | \
      sudo tee -a /etc/apt/sources.list
    install emdebian-archive-keyring
    install qemu-user gcc-4.4-mips-linux-gnu -y --force-yes
    export CC=mips-linux-gnu-gcc
    ;;

  *)
    # clang has better error messages and implements alignof more broadly
    export CC=clang

    if [ "$TARGET" = "i686-unknown-linux-gnu" ]; then
      install gcc-multilib
    fi
    ;;

esac

mkdir .cargo
cp ci/cargo-config .cargo/config
sh ci/run.sh $TARGET

if [ "$TARGET" = "x86_64-unknown-linux-gnu" ] && \
   [ "$TRAVIS_RUST_VERSION" = "nightly" ] && \
   [ "$TRAVIS_OS_NAME" = "linux" ]; then
  sh ci/dox.sh
fi
