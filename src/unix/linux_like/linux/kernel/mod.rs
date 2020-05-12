//! Platform-specific definitions which are independent of libc

cfg_if! {
    if #[cfg(any(target_arch = "x86",
                 target_arch = "arm",
                 target_arch = "mips",
                 target_arch = "powerpc",
                 target_arch = "hexagon",
                 target_arch = "sparc"))] {
        mod b32;
        pub use self::b32::*;
    } else if #[cfg(any(target_arch = "x86_64",
                        target_arch = "aarch64",
                        target_arch = "powerpc64",
                        target_arch = "mips64",
                        target_arch = "s390x",
                        target_arch = "sparc64",
                        target_arch = "riscv64"))] {
        mod b64;
        pub use self::b64::*;
    } else {
        // Unknown target_arch
    }
}
