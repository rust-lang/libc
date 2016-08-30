#[allow(dead_code)]

cfg_if! {
    if #[cfg(target_arch = "arm")] {
        mod arm;
        pub use self::arm::*;
    } else if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(target_arch = "powerpc")] {
        mod powerpc;
        pub use self::powerpc::*;
    } else if #[cfg(any(target_arch = "powerpc64"))] {
        mod powerpc64;
        pub use self::powerpc64::*;
    } else if #[cfg(target_arch = "x86")] {
        mod x86;
        pub use self::x86::*;
    } else if #[cfg(any(target_arch = "x86_64"))] {
        mod x86_64;
        pub use self::x86_64::*;
    } else {
        // Unknown target_arch
    }
}
