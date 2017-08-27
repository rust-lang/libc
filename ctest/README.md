# ctest

[![Build Status](https://travis-ci.org/alexcrichton/ctest.svg?branch=master)](https://travis-ci.org/alexcrichton/ctest)
[![Build status](https://ci.appveyor.com/api/projects/status/akjf8gn5pem05iyw?svg=true)](https://ci.appveyor.com/project/alexcrichton/ctest)

[Documentation][dox]

[dox]: https://docs.rs/ctest

Automated testing of FFI bindings in Rust. This repository is intended to
validate the `*-sys` crates that can be found on crates.io to ensure that the
APIs in Rust match the APIs defined in C.

### Example

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
my-sys-library = { path = "../my-sys-library" }
libc = "0.2"

[build-dependencies]
ctest = "0.1"
```

Next, add a build script to `systest/build.rs`:

```rust
extern crate ctest;

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    // Include the header files where the C APIs are defined
    cfg.header("foo.h")
       .header("bar.h");

    // Include the directory where the header files are defined
    cfg.include("path/to/include");

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    cfg.generate("../my-sys-library/lib.rs", "all.rs");
}

```

Next, add this to `src/main.rs`

```rust
#![allow(bad_style)]

extern crate my_sys_library;
extern crate libc;

use libc::*;
use my_sys_library::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
```

And you're good to go! To run the tests execute `cargo run` in the `systest`
directory, and everything should be kicked into action!

### How it works

This library will parse the `*-sys` crate to learn about all extern fn
definitions within. It will then generate a test suite to ensure that all
function function signatures, constant values, struct layout/alignment, type
size/alignment, etc, all match their C equivalent.

The generated tests come in two forms. One is a Rust file which contains the
`main` function (hence the `include!` above), and another is a C file which is
compiled as part of the build script. The C file is what includes all headers
and returns information about the C side of things (which is validated in Rust).

A large amount of configuration can be applied to how the C file is generated,
you can browse [the documentation][dox].

### Projects using ctest

* [libc](https://github.com/rust-lang/libc)
* [git2-rs](https://github.com/alexcrichton/git2-rs)
* [ssh2-rs](https://github.com/alexcrichton/ssh2-rs)
* [libz-sys](https://github.com/alexcrichton/libz-sys)
* [openssl-sys](https://github.com/sfackler/rust-openssl)

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
