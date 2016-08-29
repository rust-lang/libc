/* Header <sys/type.h> */
cfg_if! {
    if #[cfg(feature = "file_offset64")] {
        pub type off_t = ::off64_t;
    } else {
        pub type off_t = ::c_long;
    }
}
pub type off64_t = ::int64_t;

s! {
    /* Header <fcntl.h> */
    pub struct flock64 {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off64_t,
        pub l_len: ::off64_t,
        pub l_pid: ::pid_t,
    }
}

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
