// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Crate docs

#![allow(bad_style, overflowing_literals, improper_ctypes)]
#![crate_type = "rlib"]
#![crate_name = "libc"]
#![cfg_attr(cross_platform_docs, feature(no_core, lang_items, const_fn))]
#![cfg_attr(cross_platform_docs, no_core)]
#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
    html_favicon_url = "https://doc.rust-lang.org/favicon.ico"
)]
#![cfg_attr(
    all(target_os = "linux", target_arch = "x86_64"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-unknown-linux-gnu"
    )
)]
#![cfg_attr(
    all(target_os = "linux", target_arch = "x86"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/i686-unknown-linux-gnu"
    )
)]
#![cfg_attr(
    all(target_os = "linux", target_arch = "arm"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/arm-unknown-linux-gnueabihf"
    )
)]
#![cfg_attr(
    all(target_os = "linux", target_arch = "mips"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/mips-unknown-linux-gnu"
    )
)]
#![cfg_attr(
    all(target_os = "linux", target_arch = "aarch64"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/aarch64-unknown-linux-gnu"
    )
)]
#![cfg_attr(
    all(target_os = "linux", target_env = "musl"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-unknown-linux-musl"
    )
)]
#![cfg_attr(
    all(target_os = "macos", target_arch = "x86_64"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-apple-darwin"
    )
)]
#![cfg_attr(
    all(target_os = "macos", target_arch = "x86"),
    doc(html_root_url = "https://rust-lang.github.io/libc/i686-apple-darwin")
)]
#![cfg_attr(
    all(windows, target_arch = "x86_64", target_env = "gnu"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-pc-windows-gnu"
    )
)]
#![cfg_attr(
    all(windows, target_arch = "x86", target_env = "gnu"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/i686-pc-windows-gnu"
    )
)]
#![cfg_attr(
    all(windows, target_arch = "x86_64", target_env = "msvc"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-pc-windows-msvc"
    )
)]
#![cfg_attr(
    all(windows, target_arch = "x86", target_env = "msvc"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/i686-pc-windows-msvc"
    )
)]
#![cfg_attr(
    target_os = "android",
    doc(
        html_root_url = "https://rust-lang.github.io/libc/arm-linux-androideabi"
    )
)]
#![cfg_attr(
    target_os = "freebsd",
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-unknown-freebsd"
    )
)]
#![cfg_attr(
    target_os = "openbsd",
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-unknown-openbsd"
    )
)]
#![cfg_attr(
    target_os = "bitrig",
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-unknown-bitrig"
    )
)]
#![cfg_attr(
    target_os = "netbsd",
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-unknown-netbsd"
    )
)]
#![cfg_attr(
    target_os = "dragonfly",
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-unknown-dragonfly"
    )
)]
#![cfg_attr(
    target_os = "solaris",
    doc(
        html_root_url = "https://rust-lang.github.io/libc/x86_64-sun-solaris"
    )
)]
#![cfg_attr(
    all(target_os = "emscripten", target_arch = "asmjs"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/asmjs-unknown-emscripten"
    )
)]
#![cfg_attr(
    all(target_os = "emscripten", target_arch = "wasm32"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/wasm32-unknown-emscripten"
    )
)]
#![cfg_attr(
    all(target_os = "linux", target_arch = "sparc64"),
    doc(
        html_root_url = "https://rust-lang.github.io/libc/sparc64-unknown-linux-gnu"
    )
)]
// Attributes needed when building as part of the standard library
#![cfg_attr(feature = "rustc-dep-of-std", feature(cfg_target_vendor))]
#![cfg_attr(feature = "rustc-dep-of-std", feature(link_cfg, repr_packed))]
#![cfg_attr(feature = "rustc-dep-of-std", feature(no_core))]
#![cfg_attr(feature = "rustc-dep-of-std", no_core)]
#![cfg_attr(feature = "rustc-dep-of-std", allow(warnings))]
#![cfg_attr(not(any(feature = "use_std", feature = "rustc-dep-of-std")), no_std)]

#[cfg(all(not(cross_platform_docs), feature = "use_std"))]
extern crate std as core;

#[cfg(feature = "rustc-dep-of-std")]
extern crate rustc_std_workspace_core as core;
#[cfg(feature = "rustc-dep-of-std")]
#[allow(unused_imports)]
use core::iter;
#[cfg(feature = "rustc-dep-of-std")]
#[allow(unused_imports)]
use core::option;

#[macro_use]
mod macros;

mod dox;

cfg_if! {
    if #[cfg(windows)] {
        mod windows;
        pub use windows::*;
    } else if #[cfg(target_os = "redox")] {
        mod redox;
        pub use redox::*;
    } else if #[cfg(target_os = "cloudabi")] {
        mod cloudabi;
        pub use cloudabi::*;
    } else if #[cfg(target_os = "fuchsia")] {
        mod fuchsia;
        pub use fuchsia::*;
    } else if #[cfg(target_os = "switch")] {
        mod switch;
        pub use switch::*;
    } else if #[cfg(unix)] {
        mod unix;
        pub use unix::*;
    } else if #[cfg(target_env = "sgx")] {
        mod sgx;
        pub use sgx::*;
    }  else {
        // non-supported targets: empty...
    }
}
