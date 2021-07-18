# libc - Raw FFI bindings to platforms' system libraries

[![GHA Status]][GitHub Actions] [![Cirrus CI Status]][Cirrus CI] [![Latest Version]][crates.io] [![Documentation]][docs.rs] ![License]

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
  feature to remove this dependency and be able to use `libc` in `#![no_std]`
  crates.

* `extra_traits`: all `struct`s implemented in `libc` are `Copy` and `Clone`.
  This feature derives `Debug`, `Eq`, `Hash`, and `PartialEq`.

* `const-extern-fn`: Changes some `extern fn`s into `const extern fn`s.
   This feature requires a nightly rustc.

* **deprecated**: `use_std` is deprecated, and is equivalent to `std`.

## Rust version support

The minimum supported Rust toolchain version is **Rust 1.13.0** . APIs requiring
newer Rust features are only available on newer Rust toolchains:

| Feature              | Version |
|----------------------|---------|
| `union`              |  1.19.0 |
| `const mem::size_of` |  1.24.0 |
| `repr(align)`        |  1.25.0 |
| `extra_traits`       |  1.25.0 |
| `core::ffi::c_void`  |  1.30.0 |
| `repr(packed(N))`    |  1.33.0 |
| `cfg(target_vendor)` |  1.33.0 |

## Platform support

[Platform-specific documentation (master branch)][docs.master].

See
[`ci/build.sh`](https://github.com/rust-lang/libc/blob/master/ci/build.sh)
for the platforms on which `libc` is guaranteed to build for each Rust
toolchain. The test-matrix at [GitHub Actions] and [Cirrus CI] show the
platforms in which `libc` tests are run.

### Platform-specific documentation
* [aarch64-fuchsia](aarch64-fuchsia/doc/libc/index.html)
* [aarch64-linux-android](aarch64-linux-android/doc/libc/index.html)
* [aarch64-pc-windows-msvc](aarch64-pc-windows-msvc/doc/libc/index.html)
* [aarch64-unknown-freebsd](aarch64-unknown-freebsd/doc/libc/index.html)
* [aarch64-unknown-hermit](aarch64-unknown-hermit/doc/libc/index.html)
* [aarch64-unknown-linux-gnu](aarch64-unknown-linux-gnu/doc/libc/index.html)
* [aarch64-unknown-linux-musl](aarch64-unknown-linux-musl/doc/libc/index.html)
* [aarch64-unknown-netbsd](aarch64-unknown-netbsd/doc/libc/index.html)
* [aarch64-unknown-openbsd](aarch64-unknown-openbsd/doc/libc/index.html)
* [aarch64-wrs-vxworks](aarch64-wrs-vxworks/doc/libc/index.html)
* [arm-linux-androideabi](arm-linux-androideabi/doc/libc/index.html)
* [arm-unknown-linux-gnueabi](arm-unknown-linux-gnueabi/doc/libc/index.html)
* [arm-unknown-linux-gnueabihf](arm-unknown-linux-gnueabihf/doc/libc/index.html)
* [arm-unknown-linux-musleabi](arm-unknown-linux-musleabi/doc/libc/index.html)
* [arm-unknown-linux-musleabihf](arm-unknown-linux-musleabihf/doc/libc/index.html)
* [armebv7r-none-eabi](armebv7r-none-eabi/doc/libc/index.html)
* [armebv7r-none-eabihf](armebv7r-none-eabihf/doc/libc/index.html)
* [armv5te-unknown-linux-gnueabi](armv5te-unknown-linux-gnueabi/doc/libc/index.html)
* [armv5te-unknown-linux-musleabi](armv5te-unknown-linux-musleabi/doc/libc/index.html)
* [armv7-linux-androideabi](armv7-linux-androideabi/doc/libc/index.html)
* [armv7-unknown-linux-gnueabihf](armv7-unknown-linux-gnueabihf/doc/libc/index.html)
* [armv7-unknown-linux-musleabihf](armv7-unknown-linux-musleabihf/doc/libc/index.html)
* [armv7-wrs-vxworks-eabihf](armv7-wrs-vxworks-eabihf/doc/libc/index.html)
* [armv7r-none-eabi](armv7r-none-eabi/doc/libc/index.html)
* [armv7r-none-eabihf](armv7r-none-eabihf/doc/libc/index.html)
* [hexagon-unknown-linux-musl](hexagon-unknown-linux-musl/doc/libc/index.html)
* [i586-pc-windows-msvc](i586-pc-windows-msvc/doc/libc/index.html)
* [i586-unknown-linux-gnu](i586-unknown-linux-gnu/doc/libc/index.html)
* [i586-unknown-linux-musl](i586-unknown-linux-musl/doc/libc/index.html)
* [i686-linux-android](i686-linux-android/doc/libc/index.html)
* [i686-pc-windows-gnu](i686-pc-windows-gnu/doc/libc/index.html)
* [i686-pc-windows-msvc](i686-pc-windows-msvc/doc/libc/index.html)
* [i686-unknown-freebsd](i686-unknown-freebsd/doc/libc/index.html)
* [i686-unknown-haiku](i686-unknown-haiku/doc/libc/index.html)
* [i686-unknown-linux-gnu](i686-unknown-linux-gnu/doc/libc/index.html)
* [i686-unknown-linux-musl](i686-unknown-linux-musl/doc/libc/index.html)
* [i686-unknown-netbsd](i686-unknown-netbsd/doc/libc/index.html)
* [i686-unknown-openbsd](i686-unknown-openbsd/doc/libc/index.html)
* [i686-wrs-vxworks](i686-wrs-vxworks/doc/libc/index.html)
* [mips-unknown-linux-gnu](mips-unknown-linux-gnu/doc/libc/index.html)
* [mips-unknown-linux-musl](mips-unknown-linux-musl/doc/libc/index.html)
* [mips-unknown-linux-uclibc](mips-unknown-linux-uclibc/doc/libc/index.html)
* [mips64-unknown-linux-gnuabi64](mips64-unknown-linux-gnuabi64/doc/libc/index.html)
* [mips64-unknown-linux-muslabi64](mips64-unknown-linux-muslabi64/doc/libc/index.html)
* [mips64el-unknown-linux-gnuabi64](mips64el-unknown-linux-gnuabi64/doc/libc/index.html)
* [mips64el-unknown-linux-muslabi64](mips64el-unknown-linux-muslabi64/doc/libc/index.html)
* [mipsel-sony-psp](mipsel-sony-psp/doc/libc/index.html)
* [mipsel-unknown-linux-gnu](mipsel-unknown-linux-gnu/doc/libc/index.html)
* [mipsel-unknown-linux-musl](mipsel-unknown-linux-musl/doc/libc/index.html)
* [mipsel-unknown-linux-uclibc](mipsel-unknown-linux-uclibc/doc/libc/index.html)
* [nvptx64-nvidia-cuda](nvptx64-nvidia-cuda/doc/libc/index.html)
* [powerpc-unknown-linux-gnu](powerpc-unknown-linux-gnu/doc/libc/index.html)
* [powerpc-unknown-linux-gnuspe](powerpc-unknown-linux-gnuspe/doc/libc/index.html)
* [powerpc-unknown-netbsd](powerpc-unknown-netbsd/doc/libc/index.html)
* [powerpc-wrs-vxworks](powerpc-wrs-vxworks/doc/libc/index.html)
* [powerpc-wrs-vxworks-spe](powerpc-wrs-vxworks-spe/doc/libc/index.html)
* [powerpc64-unknown-freebsd](powerpc64-unknown-freebsd/doc/libc/index.html)
* [powerpc64-unknown-linux-gnu](powerpc64-unknown-linux-gnu/doc/libc/index.html)
* [powerpc64-wrs-vxworks](powerpc64-wrs-vxworks/doc/libc/index.html)
* [powerpc64le-unknown-linux-gnu](powerpc64le-unknown-linux-gnu/doc/libc/index.html)
* [riscv32gc-unknown-linux-gnu](riscv32gc-unknown-linux-gnu/doc/libc/index.html)
* [riscv32i-unknown-none-elf](riscv32i-unknown-none-elf/doc/libc/index.html)
* [riscv32imac-unknown-none-elf](riscv32imac-unknown-none-elf/doc/libc/index.html)
* [riscv32imc-unknown-none-elf](riscv32imc-unknown-none-elf/doc/libc/index.html)
* [riscv64gc-unknown-linux-gnu](riscv64gc-unknown-linux-gnu/doc/libc/index.html)
* [riscv64gc-unknown-none-elf](riscv64gc-unknown-none-elf/doc/libc/index.html)
* [riscv64imac-unknown-none-elf](riscv64imac-unknown-none-elf/doc/libc/index.html)
* [s390x-unknown-linux-gnu](s390x-unknown-linux-gnu/doc/libc/index.html)
* [s390x-unknown-linux-musl](s390x-unknown-linux-musl/doc/libc/index.html)
* [sparc-unknown-linux-gnu](sparc-unknown-linux-gnu/doc/libc/index.html)
* [sparc64-unknown-linux-gnu](sparc64-unknown-linux-gnu/doc/libc/index.html)
* [sparc64-unknown-netbsd](sparc64-unknown-netbsd/doc/libc/index.html)
* [sparcv9-sun-solaris](sparcv9-sun-solaris/doc/libc/index.html)
* [thumbv6m-none-eabi](thumbv6m-none-eabi/doc/libc/index.html)
* [thumbv7em-none-eabi](thumbv7em-none-eabi/doc/libc/index.html)
* [thumbv7em-none-eabihf](thumbv7em-none-eabihf/doc/libc/index.html)
* [thumbv7m-none-eabi](thumbv7m-none-eabi/doc/libc/index.html)
* [thumbv7neon-linux-androideabi](thumbv7neon-linux-androideabi/doc/libc/index.html)
* [thumbv7neon-unknown-linux-gnueabihf](thumbv7neon-unknown-linux-gnueabihf/doc/libc/index.html)
* [wasm32-unknown-emscripten](wasm32-unknown-emscripten/doc/libc/index.html)
* [wasm32-unknown-unknown](wasm32-unknown-unknown/doc/libc/index.html)
* [wasm32-wasi](wasm32-wasi/doc/libc/index.html)
* [x86_64-fortanix-unknown-sgx](x86_64-fortanix-unknown-sgx/doc/libc/index.html)
* [x86_64-fuchsia](x86_64-fuchsia/doc/libc/index.html)
* [x86_64-linux-android](x86_64-linux-android/doc/libc/index.html)
* [x86_64-pc-solaris](x86_64-pc-solaris/doc/libc/index.html)
* [x86_64-pc-windows-gnu](x86_64-pc-windows-gnu/doc/libc/index.html)
* [x86_64-pc-windows-msvc](x86_64-pc-windows-msvc/doc/libc/index.html)
* [x86_64-unknown-dragonfly](x86_64-unknown-dragonfly/doc/libc/index.html)
* [x86_64-unknown-freebsd](x86_64-unknown-freebsd/doc/libc/index.html)
* [x86_64-unknown-haiku](x86_64-unknown-haiku/doc/libc/index.html)
* [x86_64-unknown-hermit](x86_64-unknown-hermit/doc/libc/index.html)
* [x86_64-unknown-illumos](x86_64-unknown-illumos/doc/libc/index.html)
* [x86_64-unknown-l4re-uclibc](x86_64-unknown-l4re-uclibc/doc/libc/index.html)
* [x86_64-unknown-linux-gnu](x86_64-unknown-linux-gnu/doc/libc/index.html)
* [x86_64-unknown-linux-gnux32](x86_64-unknown-linux-gnux32/doc/libc/index.html)
* [x86_64-unknown-linux-musl](x86_64-unknown-linux-musl/doc/libc/index.html)
* [x86_64-unknown-netbsd](x86_64-unknown-netbsd/doc/libc/index.html)
* [x86_64-unknown-openbsd](x86_64-unknown-openbsd/doc/libc/index.html)
* [x86_64-unknown-redox](x86_64-unknown-redox/doc/libc/index.html)
* [x86_64-wrs-vxworks](x86_64-wrs-vxworks/doc/libc/index.html)

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/rust-lang/libc/blob/master/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/rust-lang/libc/blob/master/LICENSE-MIT))

at your option.

## Contributing

We welcome all people who want to contribute. Please see the [contributing
instructions] for more information.

[contributing instructions]: https://github.com/rust-lang/libc/blob/master/CONTRIBUTING.md

Contributions in any form (issues, pull requests, etc.) to this project
must adhere to Rust's [Code of Conduct].

[Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `libc` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[GitHub Actions]: https://github.com/rust-lang/libc/actions
[GHA Status]: https://github.com/rust-lang/libc/workflows/CI/badge.svg
[Cirrus CI]: https://cirrus-ci.com/github/rust-lang/libc
[Cirrus CI Status]: https://api.cirrus-ci.com/github/rust-lang/libc.svg
[crates.io]: https://crates.io/crates/libc
[Latest Version]: https://img.shields.io/crates/v/libc.svg
[Documentation]: https://docs.rs/libc/badge.svg
[docs.rs]: https://docs.rs/libc
[License]: https://img.shields.io/crates/l/libc.svg
[docs.master]: https://rust-lang.github.io/libc/#platform-specific-documentation
