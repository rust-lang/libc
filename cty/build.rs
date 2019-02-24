use std::env;
use std::process::Command;
use std::str;

fn main() {
    let rustc_minor_ver =
        rustc_minor_version().expect("Failed to get rustc version");
    let rustc_dep_of_std =
        std::env::var("CARGO_FEATURE_RUSTC_DEP_OF_STD").is_ok();

    // Rust >= 1.30 supports `core::ffi::c_void`, so cty can just re-export it.
    // Otherwise, it defines an incompatible type to retain
    // backwards-compatibility.
    if rustc_minor_ver >= 30 || rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_core_cvoid");
    }
}

fn rustc_minor_version() -> Option<u32> {
    macro_rules! otry {
        ($e:expr) => {
            match $e {
                Some(e) => e,
                None => return None,
            }
        };
    }

    let rustc = otry!(env::var_os("RUSTC"));
    let output = otry!(Command::new(rustc).arg("--version").output().ok());
    let version = otry!(str::from_utf8(&output.stdout).ok());
    let mut pieces = version.split('.');

    if pieces.next() != Some("rustc 1") {
        return None;
    }

    otry!(pieces.next()).parse().ok()
}
