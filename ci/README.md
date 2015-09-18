The goal of the libc crate is to have CI running everywhere to have the
strongest guarantees about the definitions that this library contains, and as a
result the CI is pretty complicated and also pretty large! Hopefully this can
serve as a guide through the sea of scripts in this directory and elsewhere in
this project.

First up, let's talk about the files in this directory:

* `Dockerfile-android`, `android-accept-licenses.sh` -- these two files are
  used to build the Docker image that the android CI builder uses. The
  `Dockerfile` just installs the Android SDK, NDK, a Rust nightly, Rust target
  libraries for Android, and sets up an emulator to run tests in. You can build
  a new image with this command (from the root of the project):

      docker build -t alexcrichton/rust-libc-test -f ci/Dockerfile-android .

  When building a new image contact @alexcrichton to push it to the docker hub
  and have libc start using it. This hasn't needed to happen yet, so the process
  may be a little involved.

  The script here, `android-accept-licenses.sh` is just a helper used to accept
  the licenses of the SDK of Android while the docker image is being created.

* `msys2.ps1` - a PowerShell script which is used to install MSYS2 on the
  AppVeyor bots. As of this writing MSYS2 isn't installed by default, and this
  script will install the right version/arch of msys2 in preparation of using
  the contained C compiler to compile C shims.

* `run-travis.sh` - a shell script run by all Travis builders, this is
  responsible for setting up the rest of the environment such as installing new
  packages, downloading Rust target libraries, etc.

* `run.sh` - the actual script which runs tests for a particular architecture.
  Called from the `run-travis.sh` script this will run all tests for the target
  specified.

* `cargo-config` - Cargo configuration of linkers to use copied into place by
  the `run-travis.sh` script before builds are run.

* `dox.sh` - script called from `run-travis.sh` on only the linux 64-bit nightly
  Travis bots to build documentation for this crate.

* `landing-page-*.html` - used by `dox.sh` to generate a landing page for all
  architectures' documentation.

