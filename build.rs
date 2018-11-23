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

    if cfg!(any(target_os = "macos", target_os = "ios")) {
        let xcrun = Xcrun::version();
        println!("cargo:rustc-cfg=osx_major=\"{}\"", xcrun.0);
        println!("cargo:rustc-cfg=osx_minor=\"{}\"", xcrun.1);
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

// Derived from https://github.com/fitzgen/mach/blob/bc30359e394162201f7806e8600606ed6e4a99de/mach-test/build.rs

#[derive(Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Debug)]
struct Xcrun(pub u32, pub u32);

impl Xcrun {
    pub fn version() -> Xcrun {
        use std::process::Command;
        let out = Command::new("/usr/bin/xcrun")
            .arg("--show-sdk-version")
            .output()
            .expect("failed to execute xcrun");
        let stdout = ::std::str::from_utf8(&out.stdout).expect("couldn't parse stdout as UTF8");
        let stderr = ::std::str::from_utf8(&out.stderr).expect("couldn't parse stderr as UTF8");

        if !out.status.success() {
            eprintln!("stdout: {}", stdout);
            eprintln!("stderr: {}", stderr);
            panic!("xcrun --show-sdk-version failed");
        }

        let mut iter = stdout
            .split(|c: char| c.is_whitespace() || c == '.')
            .map(|c| {
                c.parse()
                    .expect("failed to parse Xcrun version into number")
            });
        let major: u32 = iter.next().expect("failed to parse Xcrun major version");
        let minor: u32 = iter.next().expect("failed to parse Xcrun minor version");

        Xcrun(major, minor)
    }
}
