use std::process::{Command, Output};
use std::{env, str};

// List of cfgs this build script is allowed to set. The list is needed to support check-cfg, as we
// need to know all the possible cfgs that this script will set. If you need to set another cfg
// make sure to add it to this list as well.
const ALLOWED_CFGS: &[&str] = &[
    "emscripten_old_stat_abi",
    "espidf_time32",
    "freebsd10",
    "freebsd11",
    "freebsd12",
    "freebsd13",
    "freebsd14",
    "freebsd15",
    // Corresponds to `_FILE_OFFSET_BITS=64` in glibc
    "gnu_file_offset_bits64",
    // FIXME(ctest): this config shouldn't be needed but ctest can't parse `const extern fn`
    "libc_const_extern_fn",
    "libc_deny_warnings",
    "libc_ctest",
    // Corresponds to `__USE_TIME_BITS64` in UAPI
    "linux_time_bits64",
    "ntoqnx_mutex_u_is_nodestroy",
    "ntoqnx_mutex_owner_is_free",
];

// Extra values to allow for check-cfg.
const CHECK_CFG_EXTRA: &[(&str, &[&str])] = &[
    (
        "target_os",
        &[
            "switch", "aix", "ohos", "hurd", "rtems", "visionos", "nuttx", "cygwin",
        ],
    ),
    (
        "target_env",
        &["illumos", "wasi", "aix", "ohos", "nto71_iosock", "nto80"],
    ),
    (
        "target_arch",
        &["loongarch64", "mips32r6", "mips64r6", "csky"],
    ),
];

fn main() {
    // Avoid unnecessary re-building.
    println!("cargo:rerun-if-changed=build.rs");

    let (rustc_minor_ver, _is_nightly) = rustc_minor_nightly();
    let libc_ci = env::var("LIBC_CI").is_ok();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_ptr_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // The ABI of libc used by std is backward compatible with FreeBSD 12.
    // The ABI of libc from crates.io is backward compatible with FreeBSD 12.
    //
    // On CI, we detect the actual FreeBSD version and match its ABI exactly,
    // running tests to ensure that the ABI is correct.
    println!("cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_FREEBSD_VERSION");
    // Allow overriding the default version for testing
    let which_freebsd = if let Ok(version) = env::var("RUST_LIBC_UNSTABLE_FREEBSD_VERSION") {
        let vers = version.parse().unwrap();
        println!("cargo:warning=setting FreeBSD version to {vers}");
        vers
    } else if libc_ci {
        which_freebsd().unwrap_or(12)
    } else {
        12 // regardless of CARGO_FEATURE_RUSTC_DEP_OF_STD env var
    };

    match which_freebsd {
        x if x < 10 => panic!("FreeBSD older than 10 is not supported"),
        10 => set_cfg("freebsd10"),
        11 => set_cfg("freebsd11"),
        12 => set_cfg("freebsd12"),
        13 => set_cfg("freebsd13"),
        14 => set_cfg("freebsd14"),
        _ => set_cfg("freebsd15"),
    }

    match emcc_version_code() {
        Some(v) if (v < 30142) => set_cfg("emscripten_old_stat_abi"),
        // Non-Emscripten or version >= 3.1.42.
        _ => (),
    }

    let linux_time_bits64 = env::var("RUST_LIBC_UNSTABLE_LINUX_TIME_BITS64").is_ok();
    println!("cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_LINUX_TIME_BITS64");
    if linux_time_bits64 {
        set_cfg("linux_time_bits64");
    }
    println!("cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS");
    match env::var("RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS") {
        Ok(val) if val == "64" => {
            if target_env == "gnu"
                && target_os == "linux"
                && target_ptr_width == "32"
                && target_arch != "riscv32"
                && target_arch != "x86_64"
            {
                set_cfg("gnu_file_offset_bits64");
            }
        }
        Ok(val) if val != "32" => {
            panic!("RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS may only be set to '32' or '64'")
        }
        _ => {}
    }

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "nto" {
        nto_qnx::unit_tests::run();
        nto_qnx::dynamic_qnx_neutrino_values();
    }

    // On CI: deny all warnings
    if libc_ci {
        set_cfg("libc_deny_warnings");
    }

    // Set unconditionally when ctest is not being invoked.
    set_cfg("libc_const_extern_fn");

    // Since Rust 1.80, configuration that isn't recognized by default needs to be provided to
    // avoid warnings.
    if rustc_minor_ver >= 80 {
        for cfg in ALLOWED_CFGS {
            if rustc_minor_ver >= 75 {
                println!("cargo:rustc-check-cfg=cfg({cfg})");
            } else {
                println!("cargo:rustc-check-cfg=values({cfg})");
            }
        }
        for &(name, values) in CHECK_CFG_EXTRA {
            let values = values.join("\",\"");
            if rustc_minor_ver >= 75 {
                println!("cargo:rustc-check-cfg=cfg({name},values(\"{values}\"))");
            } else {
                println!("cargo:rustc-check-cfg=values({name},\"{values}\")");
            }
        }
    }
}

/// Run `rustc --version` and capture the output, adjusting arguments as needed if `clippy-driver`
/// is used instead.
fn rustc_version_cmd(is_clippy_driver: bool) -> Output {
    let rustc = env::var_os("RUSTC").expect("Failed to get rustc version: missing RUSTC env");

    let mut cmd = match env::var_os("RUSTC_WRAPPER") {
        Some(ref wrapper) if wrapper.is_empty() => Command::new(rustc),
        Some(wrapper) => {
            let mut cmd = Command::new(wrapper);
            cmd.arg(rustc);
            if is_clippy_driver {
                cmd.arg("--rustc");
            }

            cmd
        }
        None => Command::new(rustc),
    };

    cmd.arg("--version");

    let output = cmd.output().expect("Failed to get rustc version");

    assert!(
        output.status.success(),
        "failed to run rustc: {}",
        String::from_utf8_lossy(output.stderr.as_slice())
    );

    output
}

/// Return the minor version of `rustc`, as well as a bool indicating whether or not the version
/// is a nightly.
fn rustc_minor_nightly() -> (u32, bool) {
    macro_rules! otry {
        ($e:expr) => {
            match $e {
                Some(e) => e,
                None => panic!("Failed to get rustc version"),
            }
        };
    }

    let mut output = rustc_version_cmd(false);

    if otry!(str::from_utf8(&output.stdout).ok()).starts_with("clippy") {
        output = rustc_version_cmd(true);
    }

    let version = otry!(str::from_utf8(&output.stdout).ok());

    let mut pieces = version.split('.');

    assert_eq!(
        pieces.next(),
        Some("rustc 1"),
        "Failed to get rustc version"
    );

    let minor = pieces.next();

    // If `rustc` was built from a tarball, its version string
    // will have neither a git hash nor a commit date
    // (e.g. "rustc 1.39.0"). Treat this case as non-nightly,
    // since a nightly build should either come from CI
    // or a git checkout
    let nightly_raw = otry!(pieces.next()).split('-').nth(1);
    let nightly = nightly_raw.map_or(false, |raw| {
        raw.starts_with("dev") || raw.starts_with("nightly")
    });
    let minor = otry!(otry!(minor).parse().ok());

    (minor, nightly)
}

fn which_freebsd() -> Option<i32> {
    let output = Command::new("freebsd-version").output().ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;

    match &stdout {
        s if s.starts_with("10") => Some(10),
        s if s.starts_with("11") => Some(11),
        s if s.starts_with("12") => Some(12),
        s if s.starts_with("13") => Some(13),
        s if s.starts_with("14") => Some(14),
        s if s.starts_with("15") => Some(15),
        _ => None,
    }
}

fn emcc_version_code() -> Option<u64> {
    let emcc = if cfg!(target_os = "windows") {
        "emcc.bat"
    } else {
        "emcc"
    };

    let output = Command::new(emcc).arg("-dumpversion").output().ok()?;
    if !output.status.success() {
        return None;
    }

    let version = String::from_utf8(output.stdout).ok()?;

    // Some Emscripten versions come with `-git` attached, so split the
    // version string also on the `-` char.
    let mut pieces = version.trim().split(['.', '-']);

    let major = pieces.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let minor = pieces.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let patch = pieces.next().and_then(|x| x.parse().ok()).unwrap_or(0);

    Some(major * 10000 + minor * 100 + patch)
}

fn set_cfg(cfg: &str) {
    assert!(
        ALLOWED_CFGS.contains(&cfg),
        "trying to set cfg {cfg}, but it is not in ALLOWED_CFGS",
    );
    println!("cargo:rustc-cfg={cfg}");
}

mod nto_qnx {

    use std::env;

    // Unfortunately, some "constant" values get changed between release
    // of the Neutrino QNX toolchain, including in minorpatch-versions.
    // This function tries to determine the values by reading
    // header files that are always provided when building
    // for QNX. This is not a proper C parser, but very simple code
    // which checks for usage of other constants.
    // So far, only the three versions used in the unit tests
    // have been observed, and the algorithm works fine for them.
    pub fn dynamic_qnx_neutrino_values() {
        // The QNX toolchain reads files (e.g. headers, library files) from
        // directory $QNX_TARGET
        // When the path changes, also the header files can change.
        println!("cargo:rerun-if-env-changed=QNX_TARGET");
        let qnx_target_dir = env::var("QNX_TARGET").unwrap();
        let include_dir = qnx_target_dir + "/usr/include";
        let pthread_h = include_dir + "/pthread.h";
        println!("cargo:rerun-if-changed={pthread_h}");
        let pthread_h = std::fs::read_to_string(pthread_h).unwrap();
        let mut take_next_line = true;
        let pthread_mutex_init = pthread_h
            .lines()
            .map(|l| l.trim())
            .skip_while(|l| !l.contains("#define PTHREAD_RMUTEX_INITIALIZER"))
            .take_while(|l| match take_next_line {
                true => {
                    take_next_line = l.ends_with("\\");
                    true
                }
                false => false,
            })
            .collect::<String>();

        dbg!(&pthread_mutex_init);

        let u = extract_u(&pthread_mutex_init).unwrap();
        if u.contains("_NTO_SYNC_NODESTROY") {
            super::set_cfg("ntoqnx_mutex_u_is_nodestroy");
        }
        let owner = extract_owner(&pthread_mutex_init).unwrap();
        if owner.contains("_NTO_SYNC_MUTEX_FREE") {
            super::set_cfg("ntoqnx_mutex_owner_is_free");
        }
    }

    fn extract_value(name: &str, text: &str) -> Option<String> {
        let text = text.split_once(name)?.1;
        let value = text
            .chars()
            .skip_while(|c| *c != '=')
            .skip(1)
            .skip_while(|c| *c == '{' || c.is_whitespace())
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| *c != '}' && *c != ',')
            .collect::<String>();
        let value = value.trim().to_string();
        Some(value)
    }

    fn extract_u(text: &str) -> Option<String> {
        dbg!(text);
        extract_value(".__count", text).or_else(|| extract_value(".__u", text))
    }

    fn extract_owner(text: &str) -> Option<String> {
        extract_value(".__owner", text)
    }

    pub mod unit_tests {
        use super::*;

        pub fn run() {
            test_extract_value_qnx7orig();
            test_extract_value_qnx7_variant();
            test_extract_value_qnx8();
        }
        fn test_extract_value_qnx7orig() {
            let text = "#define PTHREAD_MUTEX_INITIALIZER       { .__u={_NTO_SYNC_NONRECURSIVE}, .__owner=_NTO_SYNC_INITIALIZER }";
            let value = extract_u(text).unwrap();
            assert_eq!(value, "_NTO_SYNC_NONRECURSIVE");
            let value = extract_owner(text).unwrap();
            assert_eq!(value, "_NTO_SYNC_INITIALIZER");
        }

        fn test_extract_value_qnx7_variant() {
            let text = "#define PTHREAD_RMUTEX_INITIALIZER	\
{ .__u = { .__count = _NTO_SYNC_NODESTROY}, .__owner=_NTO_SYNC_MUTEX_FREE }";
            let value = extract_u(text).unwrap();
            assert_eq!(value, "_NTO_SYNC_NODESTROY");
            let value = extract_owner(text).unwrap();
            assert_eq!(value, "_NTO_SYNC_MUTEX_FREE");
        }

        fn test_extract_value_qnx8() {
            let text = "#define PTHREAD_RMUTEX_INITIALIZER	\
   { .__u = { .__count = _NTO_SYNC_NONRECURSIVE }, .__owner = _NTO_SYNC_MUTEX_FREE }";
            let value = extract_u(text).unwrap();
            assert_eq!(value, "_NTO_SYNC_NONRECURSIVE");
            let value = extract_owner(text).unwrap();
            assert_eq!(value, "_NTO_SYNC_MUTEX_FREE");
        }
    }
}
