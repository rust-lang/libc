set -ex

if [ "$TRAVIS_OS_NAME" = "linux" ]; then
  OS=unknown-linux-gnu
else
  OS=apple-darwin
fi

export HOST=$ARCH-$OS

# clang has better error messages and implements alignof more broadly
export CC=clang

if [ "$TARGET" = "arm-linux-androideabi" ]; then
  # Pull a pre-built docker image for testing android, then run tests entirely
  # within that image.
  docker pull alexcrichton/rust-libc-test
  docker run -v `pwd`:/clone -t alexcrichton/rust-libc-test sh ci/run.sh $TARGET
elif [ "$TARGET" = "x86_64-unknown-linux-musl" ]; then
  curl -sO https://people.mozilla.org/~acrichton/libc-test/2015-09-08/x86_64-unknown-linux-musl.tar.gz | \
    tar xzf -C $HOME/rust/lib/rustlib
  sh ci/run.sh $TARGET
else
  # Download and install the relevant target locally, then run tests
  curl -sO https://static.rust-lang.org/dist/rust-$TRAVIS_RUST_VERSION-$HOST.tar.gz
  tar xf rust-$TRAVIS_RUST_VERSION-$HOST.tar.gz
  rm -rf $HOME/rust/lib/rustlib/$HOST
  mv rust-$TRAVIS_RUST_VERSION-$HOST/rustc/lib/rustlib/$HOST \
     $HOME/rust/lib/rustlib
  sh ci/run.sh $HOST
fi
