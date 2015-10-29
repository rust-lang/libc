The goal of the libc crate is to have CI running everywhere to have the
strongest guarantees about the definitions that this library contains, and as a
result the CI is pretty complicated and also pretty large! Hopefully this can
serve as a guide through the sea of scripts in this directory and elsewhere in
this project.

# Files

First up, let's talk about the files in this directory:

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

# CI Systems

Currently this repository leverages a combination of Travis CI and AppVeyor for
running tests. The triples tested are:

* AppVeyor
  * `{i686,x86_64}-pc-windows-{msvc,gnu}`
* Travis
  *  `{i686,x86_64,mips,aarch64}-unknown-linux-gnu`
  *  `x86_64-unknown-linux-musl`
  *  `arm-unknown-linux-gnueabihf`
  *  `arm-linux-androideabi`
  *  `{i686,x86_64}-apple-{darwin,ios}`

The Windows triples are all pretty standard, they just set up their environment
then run tests, no need for downloading any extra target libs (we just download
the right installer). The Intel Linux/OSX builds are similar in that we just
download the right target libs and run tests. Note that the Intel Linux/OSX
builds are run on stable/beta/nightly, but are the only ones that do so.

The remaining architectures look like:

* Android runs in a [docker image][android-docker] with an emulator, the NDK,
  and the SDK already set up. The entire build happens within the docker image.
* The MIPS, ARM, and AArch64 builds all use QEMU to run the generated binary to
  actually verify the tests pass.
* The MUSL build just has to download a MUSL compiler and target libraries and
  then otherwise runs tests normally.
* iOS builds need an extra linker flag currently, but beyond that they're built
  as standard as everything else.

[android-docker]: https://github.com/rust-lang/rust-buildbot/blob/master/slaves/android/Dockerfile

Hopefully that's at least somewhat of an introduction to everything going on
here, and feel free to ping @alexcrichton with questions!

