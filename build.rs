use std::env;
use std::process::Command;
use std::str;

fn main() {
    /*
     * If `core::ffi::c_void` exists, libc can just re-export it. Otherwise, it
     * must define an incompatible type to retain backwards-compatibility.
     */
    if rustc_minor_version().expect("Failed to get rustc version") >= 30 {
        println!("cargo:rustc-cfg=core_cvoid");
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
