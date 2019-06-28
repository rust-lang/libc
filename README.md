[![Travis-CI Status]][Travis-CI] [![Appveyor Status]][Appveyor] [![Cirrus-CI Status]][Cirrus-CI] [![Latest Version]][crates.io] [![Documentation]][docs.rs] ![License]

libc - Raw FFI bindings to platforms' system libraries
====

`libc` provides all of the definitions necessary to easily interoperate with C
code (or "C-like" code) on each of the platforms that Rust supports. This
includes type definitions (e.g. `c_int`), constants (e.g. `EINVAL`) as well as
function headers (e.g. `malloc`).

This crate exports all underlying platform types, functions, and constants under
the crate root, so all items are accessible as `libc::foo`. The types and values
of all the exported APIs match the platform that libc is compiled for.

More detailed information about the design of this library can be found in its
[associated RFC][rfc].

[rfc]: https://github.com/rust-lang/rfcs/blob/master/text/1291-promote-libc.md

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
libc = "0.2"
```

## Features

* `std`: by default `libc` links to the standard library. Disable this
  feature remove this dependency and be able to use `libc` in `#![no_std]`
  crates.

* `extra_traits`: all `struct`s implemented in `libc` are `Copy` and `Clone`.
  This feature derives `Debug`, `Eq`, `Hash`, and `PartialEq`.

* **deprecated**: `use_std` is deprecated, and is equivalent to `std`.

## Rust version support

The minimum supported Rust toolchain version is **Rust 1.13.0** . APIs requiring
newer Rust features are only available on newer Rust toolchains:

| Feature              | Version |
|----------------------|---------|
| `union`              |  1.19.0 |
| `const mem::size_of` |  1.24.0 |
| `repr(align)`        |  1.25.0 |
| `extra_traits`      |  1.25.0 |
| `core::ffi::c_void`  |  1.30.0 |
| `repr(packed(N))` |  1.33.0 |

## Platform support

[Platform-specific documentation (master branch)][docs.master].

See
[`ci/build.sh`](https://github.com/rust-lang/libc/blob/master/ci/build.sh)
for the platforms on which `libc` is guaranteed to build for each Rust
toolchain. The test-matrix at [Travis-CI], [Appveyor], and [Cirrus-CI] show the
platforms in which `libc` tests are run.

### Platform-specific documentation
* [aarch64-linux-android](aarch64-linux-android/libc/index.html)
* [aarch64-unknown-linux-gnu](aarch64-unknown-linux-gnu/libc/index.html)
* [arm-linux-androideabi](arm-linux-androideabi/libc/index.html)
* [arm-unknown-linux-gnueabi](arm-unknown-linux-gnueabi/libc/index.html)
* [arm-unknown-linux-gnueabihf](arm-unknown-linux-gnueabihf/libc/index.html)
* [armv7-linux-androideabi](armv7-linux-androideabi/libc/index.html)
* [armv7-unknown-linux-gnueabihf](armv7-unknown-linux-gnueabihf/libc/index.html)
* [i586-unknown-linux-gnu](i586-unknown-linux-gnu/libc/index.html)
* [i686-linux-android](i686-linux-android/libc/index.html)
* [i686-unknown-freebsd](i686-unknown-freebsd/libc/index.html)
* [i686-unknown-linux-gnu](i686-unknown-linux-gnu/libc/index.html)
* [i686-unknown-linux-musl](i686-unknown-linux-musl/libc/index.html)
* [mips-unknown-linux-gnu](mips-unknown-linux-gnu/libc/index.html)
* [mips-unknown-linux-musl](mips-unknown-linux-musl/libc/index.html)
* [mips64-unknown-linux-gnuabi64](mips64-unknown-linux-gnuabi64/libc/index.html)
* [mips64el-unknown-linux-gnuabi64](mips64el-unknown-linux-gnuabi64/libc/index.html)
* [mipsel-unknown-linux-gnu](mipsel-unknown-linux-gnu/libc/index.html)
* [mipsel-unknown-linux-gnu](mipsel-unknown-linux-gnu/libc/index.html)
* [mipsel-unknown-linux-musl](mipsel-unknown-linux-musl/libc/index.html)
* [powerpc-unknown-linux-gnu](powerpc-unknown-linux-gnu/libc/index.html)
* [powerpc64-unknown-linux-gnu](powerpc64-unknown-linux-gnu/libc/index.html)
* [powerpc64le-unknown-linux-gnu](powerpc64le-unknown-linux-gnu/libc/index.html)
* [s390x-unknown-linux-gnu](s390x-unknown-linux-gnu/libc/index.html)
* [x86_64-unknown-freebsd](x86_64-unknown-freebsd/libc/index.html)
* [x86_64-unknown-linux-gnu](x86_64-unknown-linux-gnu/libc/index.html)
* [x86_64-unknown-linux-musl](x86_64-unknown-linux-musl/libc/index.html)
* [x86_64-unknown-netbsd](x86_64-unknown-netbsd/libc/index.html)
* [arm-unknown-linux-musleabi](arm-unknown-linux-musleabi/libc/index.html)
* [arm-unknown-linux-musleabihf](arm-unknown-linux-musleabihf/libc/index.html)
* [armv7-unknown-linux-musleabihf](armv7-unknown-linux-musleabihf/libc/index.html)
* [sparc64-unknown-linux-gnu](sparc64-unknown-linux-gnu/libc/index.html)
* [wasm32-unknown-emscripten](wasm32-unknown-emscripten/libc/index.html)
* [x86_64-linux-android](x86_64-linux-android/libc/index.html)
* [x86_64-rumprun-netbsd](x86_64-rumprun-netbsd/libc/index.html)
* [aarch64-unknown-linux-musl](aarch64-unknown-linux-musl/libc/index.html)
* [sparcv9-sun-solaris](sparcv9-sun-solaris/libc/index.html)
* [wasm32-unknown-unknown](wasm32-unknown-unknown/libc/index.html)
* [x86_64-sun-solaris](x86_64-sun-solaris/libc/index.html)
* [i586-unknown-linux-musl](i586-unknown-linux-musl/libc/index.html)
* [x86_64-unknown-cloudabi](x86_64-unknown-cloudabi/libc/index.html)
* [aarch64-fuchsia](aarch64-fuchsia/libc/index.html)
* [armv5te-unknown-linux-gnueabi](armv5te-unknown-linux-gnueabi/libc/index.html)
* [armv5te-unknown-linux-musleabi](armv5te-unknown-linux-musleabi/libc/index.html)
* [i686-pc-windows-gnu](i686-pc-windows-gnu/libc/index.html)
* [wasm32-wasi](wasm32-wasi/libc/index.html)
* [x86_64-fortanix-unknown-sgx](x86_64-fortanix-unknown-sgx/libc/index.html)
* [x86_64-fuchsia](x86_64-fuchsia/libc/index.html)
* [x86_64-pc-windows-gnu](x86_64-pc-windows-gnu/libc/index.html)
* [x86_64-unknown-linux-gnux32](x86_64-unknown-linux-gnux32/libc/index.html)
* [x86_64-unknown-redox](x86_64-unknown-redox/libc/index.html)
* [aarch64-pc-windows-msvc](aarch64-pc-windows-msvc/libc/index.html)
* [aarch64-unknown-cloudabi](aarch64-unknown-cloudabi/libc/index.html)
* [aarch64-unknown-freebsd](aarch64-unknown-freebsd/libc/index.html)
* [aarch64-unknown-hermit](aarch64-unknown-hermit/libc/index.html)
* [aarch64-unknown-netbsd](aarch64-unknown-netbsd/libc/index.html)
* [aarch64-unknown-openbsd](aarch64-unknown-openbsd/libc/index.html)
* [armebv7r-none-eabi](armebv7r-none-eabi/libc/index.html)
* [armebv7r-none-eabihf](armebv7r-none-eabihf/libc/index.html)
* [armv7-unknown-cloudabi-eabihf](armv7-unknown-cloudabi-eabihf/libc/index.html)
* [armv7r-none-eabi](armv7r-none-eabi/libc/index.html)
* [armv7r-none-eabihf](armv7r-none-eabihf/libc/index.html)
* [i586-pc-windows-msvc](i586-pc-windows-msvc/libc/index.html)
* [i686-pc-windows-msvc](i686-pc-windows-msvc/libc/index.html)
* [i686-unknown-cloudabi](i686-unknown-cloudabi/libc/index.html)
* [i686-unknown-haiku](i686-unknown-haiku/libc/index.html)
* [i686-unknown-netbsd](i686-unknown-netbsd/libc/index.html)
* [i686-unknown-openbsd](i686-unknown-openbsd/libc/index.html)
* [mips-unknown-linux-uclibc](mips-unknown-linux-uclibc/libc/index.html)
* [mipsel-unknown-linux-uclibc](mipsel-unknown-linux-uclibc/libc/index.html)
* [nvptx64-nvidia-cuda](nvptx64-nvidia-cuda/libc/index.html)
* [powerpc-unknown-linux-gnuspe](powerpc-unknown-linux-gnuspe/libc/index.html)
* [powerpc-unknown-netbsd](powerpc-unknown-netbsd/libc/index.html)
* [powerpc64-unknown-freebsd](powerpc64-unknown-freebsd/libc/index.html)
* [riscv32imac-unknown-none-elf](riscv32imac-unknown-none-elf/libc/index.html)
* [riscv32imc-unknown-none-elf](riscv32imc-unknown-none-elf/libc/index.html)
* [sparc64-unknown-netbsd](sparc64-unknown-netbsd/libc/index.html)
* [thumbv6m-none-eabi](thumbv6m-none-eabi/libc/index.html)
* [thumbv7em-none-eabi](thumbv7em-none-eabi/libc/index.html)
* [thumbv7em-none-eabihf](thumbv7em-none-eabihf/libc/index.html)
* [thumbv7m-none-eabi](thumbv7m-none-eabi/libc/index.html)
* [thumbv7neon-linux-androideabi](thumbv7neon-linux-androideabi/libc/index.html)
* [thumbv7neon-unknown-linux-gnueabihf](thumbv7neon-unknown-linux-gnueabihf/libc/index.html)
* [x86_64-unknown-dragonfly](x86_64-unknown-dragonfly/libc/index.html)
* [x86_64-unknown-haiku](x86_64-unknown-haiku/libc/index.html)
* [x86_64-unknown-hermit](x86_64-unknown-hermit/libc/index.html)
* [x86_64-unknown-l4re-uclibc](x86_64-unknown-l4re-uclibc/libc/index.html)
* [x86_64-unknown-openbsd](x86_64-unknown-openbsd/libc/index.html)

## License

This project is licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](http://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

We welcome all people who want to contribute. Please see the [contributing
instructions] for more information.

[contributing instructions]: CONTRIBUTING.md

Contributions in any form (issues, pull requests, etc.) to this project
must adhere to Rust's [Code of Conduct].

[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `libc` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[Travis-CI]: https://travis-ci.com/rust-lang/libc
[Travis-CI Status]: https://travis-ci.com/rust-lang/libc.svg?branch=master
[Appveyor]: https://ci.appveyor.com/project/rust-lang-libs/libc
[Appveyor Status]: https://ci.appveyor.com/api/projects/status/github/rust-lang/libc?svg=true
[Cirrus-CI]: https://cirrus-ci.com/github/rust-lang/libc
[Cirrus-CI Status]: https://api.cirrus-ci.com/github/rust-lang/libc.svg
[crates.io]: https://crates.io/crates/libc
[Latest Version]: https://img.shields.io/crates/v/libc.svg
[Documentation]: https://docs.rs/libc/badge.svg
[docs.rs]: https://docs.rs/libc
[License]: https://img.shields.io/crates/l/libc.svg
[docs.master]: https://rust-lang.github.io/libc/#platform-specific-documentation
