// Native C types
pub type c_long = i64;
pub type c_ulong = u64;

/* Header <fcntl.h> */
cfg_if! {
    if #[cfg(feature = "file_offset64")] {
        type flock = ::flock64;
    } else {
        s! {
            pub struct flock {
                pub l_type: ::c_short,
                pub l_whence: ::c_short,
                pub l_start: ::off_t,
                pub l_len: ::off_t,
                pub l_pid: ::pid_t,
            }
        }
    }
}

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(any(target_arch = "mips64"))] {
        mod mips64;
        pub use self::mips64::*;
    } else if #[cfg(any(target_arch = "powerpc64"))] {
        mod powerpc64;
        pub use self::powerpc64::*;
    } else if #[cfg(any(target_arch = "x86_64"))] {
        mod x86_64;
        pub use self::x86_64::*;
    } else {
        // Unknown target_arch
    }
}
