cfg_if! {
    if #[cfg(target_arch = "x86")] {
        mod x86;
        pub use self::x86::*;
    } else if #[cfg(target_arch = "arm")] {
        mod arm;
        pub use self::arm::*;
    } else if #[cfg(target_arch = "mips")] {
        mod mips;
        pub use self::mips::*;
    } else if #[cfg(target_arch = "powerpc")] {
        mod powerpc;
        pub use self::powerpc::*;
    } else if #[cfg(target_arch = "hexagon")] {
        mod hexagon;
        pub use self::hexagon::*;
    } else if #[cfg(target_arch = "sparc")] {
        mod sparc;
        pub use self::sparc::*;
    } else {
        // Unknown target_arch
    }
}
