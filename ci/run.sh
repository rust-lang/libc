#!/bin/sh

# Builds and runs tests for a particular target passed as an argument to this
# script.

set -ex

TARGET=$1

# If we're going to run tests inside of a qemu image, then we don't need any of
# the scripts below. Instead, download the image, prepare a filesystem which has
# the current state of this repository, and then run the image.
#
# It's assume that all images, when run with two disks, will run the `run.sh`
# script from the second which we place inside.
if [ "$QEMU" != "" ]; then
  tmpdir=/tmp/qemu-img-creation
  mkdir -p $tmpdir

  if [ -z "${QEMU#*.gz}" ]; then
    # image is .gz : download and uncompress it
    qemufile=$(echo ${QEMU%.gz} | sed 's/\//__/g')
    if [ ! -f $tmpdir/$qemufile ]; then
      curl https://s3.amazonaws.com/rust-lang-ci/libc/$QEMU | \
        gunzip -d > $tmpdir/$qemufile
    fi
  else
    # plain qcow2 image: just download it
    qemufile=$(echo ${QEMU} | sed 's/\//__/g')
    if [ ! -f $tmpdir/$qemufile ]; then
      curl https://s3.amazonaws.com/rust-lang-ci/libc/$QEMU \
        > $tmpdir/$qemufile
    fi
  fi

  # Create a mount a fresh new filesystem image that we'll later pass to QEMU.
  # This will have a `run.sh` script will which use the artifacts inside to run
  # on the host.
  rm -f $tmpdir/libc-test.img
  mkdir $tmpdir/mount

  # Do the standard rigamarole of cross-compiling an executable and then the
  # script to run just executes the binary.
  cargo build --manifest-path libc-test/Cargo.toml --target $TARGET
  cp $CARGO_TARGET_DIR/$TARGET/debug/libc-test $tmpdir/mount/
  echo 'exec $1/libc-test' > $tmpdir/mount/run.sh

  du -sh $tmpdir/mount
  genext2fs \
      --root $tmpdir/mount \
      --size-in-blocks 100000 \
      $tmpdir/libc-test.img

  # Pass -snapshot to prevent tampering with the disk images, this helps when
  # running this script in development. The two drives are then passed next,
  # first is the OS and second is the one we just made. Next the network is
  # configured to work (I'm not entirely sure how), and then finally we turn off
  # graphics and redirect the serial console output to out.log.
  qemu-system-x86_64 \
    -m 1024 \
    -snapshot \
    -drive if=virtio,file=$tmpdir/$qemufile \
    -drive if=virtio,file=$tmpdir/libc-test.img \
    -net nic,model=virtio \
    -net user \
    -nographic \
    -vga none 2>&1 | tee $CARGO_TARGET_DIR/out.log
  exec grep "^PASSED .* tests" $CARGO_TARGET_DIR/out.log
fi

case "$TARGET" in
  *-apple-ios)
    cargo rustc --manifest-path libc-test/Cargo.toml --target $TARGET \
        --test main -- -C link-args=-mios-simulator-version-min=7.0
    cargo rustc --manifest-path libc-test/Cargo.toml --target $TARGET \
        --test linux-fcntl -- -C link-args=-mios-simulator-version-min=7.0
    ;;

  *)
    cargo build --manifest-path libc-test/Cargo.toml --target $TARGET --tests
    ;;
esac

case "$TARGET" in
  # Android emulator for x86_64 does not work on travis (missing hardware
  # acceleration). Tests are run on case *). See ci/android-sysimage.sh for
  # informations about how tests are run.
  arm-linux-androideabi | aarch64-linux-android | i686-linux-android)
    # set SHELL so android can detect a 64bits system, see
    # http://stackoverflow.com/a/41789144
    # https://issues.jenkins-ci.org/browse/JENKINS-26930?focusedCommentId=230791&page=com.atlassian.jira.plugin.system.issuetabpanels:comment-tabpanel#comment-230791
    export SHELL=/bin/dash
    arch=$(echo $TARGET | cut -d- -f1)
    accel="-no-accel"
    if emulator -accel-check; then
      accel=""
    fi
    emulator @$arch -no-window $accel &
    adb wait-for-device
    adb push $CARGO_TARGET_DIR/$TARGET/debug/main-* /data/local/tmp/main
    adb shell /data/local/tmp/main 2>&1 | tee /tmp/out
    grep "^PASSED .* tests" /tmp/out
    adb push $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-* /data/local/tmp/linux_fcntl
    adb shell /data/local/tmp/linux_fcntl 2>&1 | tee /tmp/out
    grep "^PASSED .* tests" /tmp/out
    ;;

  i386-apple-ios)
    rustc -O ./ci/ios/deploy_and_run_on_ios_simulator.rs
    ./deploy_and_run_on_ios_simulator $CARGO_TARGET_DIR/$TARGET/debug/main-*
    ./deploy_and_run_on_ios_simulator $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;

  x86_64-apple-ios)
    rustc -O ./ci/ios/deploy_and_run_on_ios_simulator.rs
    ./deploy_and_run_on_ios_simulator $CARGO_TARGET_DIR/$TARGET/debug/main-*
    ./deploy_and_run_on_ios_simulator $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;

  arm-unknown-linux-gnueabihf)
    qemu-arm -L /usr/arm-linux-gnueabihf $CARGO_TARGET_DIR/$TARGET/debug/main-*
    qemu-arm -L /usr/arm-linux-gnueabihf $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;

  mips-unknown-linux-gnu)
    qemu-mips -L /usr/mips-linux-gnu $CARGO_TARGET_DIR/$TARGET/debug/main-*
    qemu-mips -L /usr/mips-linux-gnu $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;

  mips64-unknown-linux-gnuabi64)
    qemu-mips64 -L /usr/mips64-linux-gnuabi64 $CARGO_TARGET_DIR/$TARGET/debug/main-*
    qemu-mips64 -L /usr/mips64-linux-gnuabi64 $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;

  mips-unknown-linux-musl)
    qemu-mips -L /toolchain/staging_dir/toolchain-mips_34kc_gcc-5.3.0_musl-1.1.15 \
              $CARGO_TARGET_DIR/$TARGET/debug/main-*
    qemu-mips -L /toolchain/staging_dir/toolchain-mips_34kc_gcc-5.3.0_musl-1.1.15 \
              $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;

  mipsel-unknown-linux-musl)
      qemu-mipsel -L /toolchain $CARGO_TARGET_DIR/$TARGET/debug/main-*
      qemu-mipsel -L /toolchain $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
      ;;

  powerpc-unknown-linux-gnu)
    qemu-ppc -L /usr/powerpc-linux-gnu $CARGO_TARGET_DIR/$TARGET/debug/main-*
    qemu-ppc -L /usr/powerpc-linux-gnu $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;

  powerpc64-unknown-linux-gnu)
    qemu-ppc64 -L /usr/powerpc64-linux-gnu $CARGO_TARGET_DIR/$TARGET/debug/main-*
    qemu-ppc64 -L /usr/powerpc64-linux-gnu $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;

  aarch64-unknown-linux-gnu)
    qemu-aarch64 -L /usr/aarch64-linux-gnu/ $CARGO_TARGET_DIR/$TARGET/debug/main-*
    qemu-aarch64 -L /usr/aarch64-linux-gnu/ $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;

  s390x-unknown-linux-gnu)
    # TODO: in theory we should execute this, but qemu segfaults immediately :(
    # qemu-s390x -L /usr/s390x-linux-gnu/ $CARGO_TARGET_DIR/$TARGET/debug/main-*
    # qemu-s390x -L /usr/s390x-linux-gnu/ $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;

  *-rumprun-netbsd)
    rumprun-bake hw_virtio /tmp/libc-test.img $CARGO_TARGET_DIR/$TARGET/debug/main-*
    qemu-system-x86_64 -nographic -vga none -m 64 \
        -kernel /tmp/libc-test.img 2>&1 | tee /tmp/out &
    sleep 5
    grep "^PASSED .* tests" /tmp/out
    rumprun-bake hw_virtio /tmp/libc-test.img $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    qemu-system-x86_64 -nographic -vga none -m 64 \
        -kernel /tmp/libc-test.img 2>&1 | tee /tmp/out &
    sleep 5
    grep "^PASSED .* tests" /tmp/out
    ;;

  *)
    $CARGO_TARGET_DIR/$TARGET/debug/main-*
    $CARGO_TARGET_DIR/$TARGET/debug/linux_fcntl-*
    ;;
esac
