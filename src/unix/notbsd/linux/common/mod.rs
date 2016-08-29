cfg_if! {
    if #[cfg(any(target_arch = "x86",
                 target_arch = "arm",
                 target_arch = "mips",
                 target_arch = "powerpc"))] {
        mod b32;
        pub use self::b32::*;
    } else if #[cfg(any(target_arch = "x86_64",
                        target_arch = "aarch64",
                        target_arch = "mips64",
                        target_arch = "powerpc64"))] {
        mod b64;
        pub use self::b64::*;
    } else {
        // Unknown target_arch
    }
}
