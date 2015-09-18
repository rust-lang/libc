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

MAIN_TARGETS=https://static.rust-lang.org/dist
EXTRA_TARGETS=https://people.mozilla.org/~acrichton/libc-test/2015-09-08

install() {
  sudo apt-get update
  sudo apt-get install $@
}

# NOTE: this is not actually run on travis right now, this was added to in
# theory run FreeBSD vagrant images on Travis, but it ended up not working, so
# this may not be working when you read this.
install_vagrant() {
  echo 'deb http://download.virtualbox.org/virtualbox/debian trusty contrib' | \
    sudo tee -a /etc/apt/sources.list
  vbox=virtualbox-5.0_5.0.4-102546~Ubuntu~trusty_amd64.deb
  curl https://www.virtualbox.org/download/oracle_vbox.asc | sudo apt-key add -
  install virtualbox-5.0 linux-headers-3.16.0-31-generic nfs-kernel-server

  # After we've got virtualbox, install vagrant itself. Note that the version in
  # the default ubuntu repos is too old to run the images we have, so install
  # the one from the vagrant website's download link
  curl -LO https://dl.bintray.com/mitchellh/vagrant/vagrant_1.7.4_x86_64.deb
  sudo dpkg -i vagrant_1.7.4_x86_64.deb
}

if [ "$TARGET" = "arm-linux-androideabi" ]; then
  # Pull a pre-built docker image for testing android, then run tests entirely
  # within that image.
  docker pull alexcrichton/rust-libc-test
  exec docker run -v `pwd`:/clone -t alexcrichton/rust-libc-test \
    sh ci/run.sh $TARGET
elif [ "$TARGET" = "x86_64-unknown-linux-musl" ]; then
  curl -s $EXTRA_TARGETS/$TARGET.tar.gz | tar xzf - -C $HOME/rust/lib/rustlib
  install musl-tools
  export CC=musl-gcc
elif [ "$TARGET" = "arm-unknown-linux-gnueabihf" ]; then
  curl -s $EXTRA_TARGETS/$TARGET.tar.gz | tar xzf - -C $HOME/rust/lib/rustlib
  install gcc-4.7-arm-linux-gnueabihf qemu-user
  export CC=arm-linux-gnueabihf-gcc-4.7
elif [ "$TARGET" = "aarch64-unknown-linux-gnu" ]; then
  curl -s $EXTRA_TARGETS/$TARGET.tar.gz | tar xzf - -C $HOME/rust/lib/rustlib
  install gcc-aarch64-linux-gnu qemu-user
  export CC=aarch64-linux-gnu-gcc
elif [ "$TARGET" = "mips-unknown-linux-gnu" ]; then
  # Download pre-built and custom MIPS libs and then also instsall the MIPS
  # compiler according to this post:
  # http://sathisharada.blogspot.com/2014_10_01_archive.html
  curl -s $EXTRA_TARGETS/$TARGET.tar.gz | tar xzf - -C $HOME/rust/lib/rustlib

  echo 'deb http://ftp.de.debian.org/debian squeeze main' | \
    sudo tee -a /etc/apt/sources.list
  echo 'deb http://www.emdebian.org/debian/ squeeze main' | \
    sudo tee -a /etc/apt/sources.list
  install emdebian-archive-keyring
  install qemu-user gcc-4.4-mips-linux-gnu -y --force-yes
  export CC=mips-linux-gnu-gcc
elif [ "$TARGET" = "x86_64-unknown-freebsd" ]; then
  install_vagrant
  cd ci
  vagrant up freebsd
  exec vagrant ssh freebsd -c \
    'cd /vagrant && CARGO_TARGET_DIR=/tmp sh ci/run.sh x86_64-unknown-freebsd'
else
  # Download the rustlib folder from the relevant portion of main distribution's
  # tarballs.
  curl -s $MAIN_TARGETS/rust-$TRAVIS_RUST_VERSION-$HOST.tar.gz | \
    tar xzf - -C $HOME/rust/lib/rustlib --strip-components=4 \
      rust-$TRAVIS_RUST_VERSION-$HOST/rustc/lib/rustlib/$HOST
  TARGET=$HOST

  # clang has better error messages and implements alignof more broadly
  export CC=clang

  if [ "$TARGET" = "i686-unknown-linux-gnu" ]; then
    install gcc-multilib
  fi
fi

mkdir .cargo
cp ci/cargo-config .cargo/config
sh ci/run.sh $TARGET

if [ "$TARGET" = "x86_64-unknown-linux-gnu" ] && \
   [ "$TRAVIS_RUST_VERSION" = "nightly" ] && \
   [ "$TRAVIS_OS_NAME" = "linux" ]; then
  sh ci/dox.sh
fi
