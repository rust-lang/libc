# ctest

[Documentation][dox]

[dox]: https://docs.rs/ctest

Automated testing of FFI bindings in Rust. This repository is intended to
validate the `*-sys` crates that can be found on crates.io to ensure that the
APIs in Rust match the APIs defined in C.

## MSRV (Minimum Supported Rust Version)

The MSRV is 1.88.0 because of the transitive dependencies.
Note that MSRV may be changed anytime by dependencies.

## Example

Unfortunately the usage today is a little wonky, but to use this library, first,
create a new Cargo project in your repo:

```
$ cargo new --bin systest
```

Then, edit `systest/Cargo.toml` to add these dependencies:

```toml
[package]
# ...
build = "build.rs"

[dependencies]
mylib-sys = { path = "../mylib-sys" }
libc = "0.2"

[build-dependencies]
ctest = "0.5.0-beta.0"
```

Next, add a build script to `systest/build.rs`:

```rust
fn main() {
    let mut cfg = ctest::TestGenerator::new();

    // Include the header files where the C APIs are defined
    cfg.header("foo.h")
       .header("bar.h");

    // Include the directory where the header files are defined
    cfg.include("path/to/include");

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    ctest::generate_test(&mut cfg, "../mylib-sys/lib.rs", "all.rs");
}
```

Next, add this to `src/main.rs`

```rust
#![allow(bad_style)]

use libc::*;
use mylib_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
```

And you're good to go! To run the tests execute `cargo run` in the `systest`
directory, and everything should be kicked into action!

## How it works

This library will parse the `*-sys` crate to learn about all definitions within.
It will then generate a test suite to ensure that all function signatures,
constant values, struct layout/alignment, type size/alignment, etc,
all match their C equivalent.

The generated tests come in two forms. One is a Rust file which contains the
`main` function (hence the `include!` above), and another is a C file which is
compiled as part of the build script. The C file is what includes all headers
and returns information about the C side of things (which is validated in Rust).

A large amount of configuration can be applied to how the C file is generated,
you can browse [the documentation][dox].

## Projects using ctest

- [libc](https://github.com/rust-lang/libc)
- [libz-sys](https://github.com/rust-lang/libz-sys)

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)

at your option.

## Modifying test templates
If you modify the test templates for either Rust or C in any way, then before
contributing you must run the following command to update the pre-generated test
files we check against:
```rust
$ LIBC_BLESS=1 cargo test
```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in ctest by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
