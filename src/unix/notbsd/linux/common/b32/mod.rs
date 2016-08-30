// Native C types
pub type c_long = i32;
pub type c_ulong = u32;

/* Header <fcntl.h> */
cfg_if! {
    if #[cfg(any(feature = "file_offset64", target_env = "musl"))] {
        pub const F_GETLK: ::c_int = ::F_GETLK64;
        pub const F_SETLK: ::c_int = ::F_SETLK64;
        pub const F_SETLKW: ::c_int = ::F_SETLKW64;
    }
}

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
    } else {
        // Unknown target_arch
    }
}

