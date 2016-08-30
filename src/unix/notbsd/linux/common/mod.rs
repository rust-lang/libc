/* Header <sys/type.h> */
cfg_if! {
    if #[cfg(any(feature = "file_offset64", target_env = "musl"))] {
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

/* Header <fcntl.h> */
pub const F_RDLCK: ::c_short = 0;
pub const F_WRLCK: ::c_short = 1;
pub const F_UNLCK: ::c_short = 2;

// Here start non POSIX definitions.
pub const AT_NO_AUTOMOUNT: ::c_int = 0x800;
pub const AT_EMPTY_PATH: ::c_int = 0x1000;

pub const FAPPEND: ::c_int = ::O_APPEND;
pub const FFSYNC: ::c_int = ::O_FSYNC;
pub const FASYNC: ::c_int = ::O_ASYNC;
pub const FNONBLOCK: ::c_int = ::O_NONBLOCK;
pub const FNDELAY: ::c_int = ::O_NDELAY;

pub const DN_ACCESS: ::c_int = 0x00000001;
pub const DN_MODIFY: ::c_int = 0x00000002;
pub const DN_CREATE: ::c_int = 0x00000004;
pub const DN_DELETE: ::c_int = 0x00000008;
pub const DN_RENAME: ::c_int = 0x00000010;
pub const DN_ATTRIB: ::c_int = 0x00000020;
pub const DN_MULTISHOT: ::c_int = 0x80000000;

pub const FALLOC_FL_KEEP_SIZE: ::c_int = 0x01;
pub const FALLOC_FL_PUNCH_HOLE: ::c_int = 0x02;

pub const SYNC_FILE_RANGE_WAIT_BEFORE: ::c_uint = 0x01;
pub const SYNC_FILE_RANGE_WRITE: ::c_uint = 0x02;
pub const SYNC_FILE_RANGE_WAIT_AFTER: ::c_uint = 0x04;

pub const F_SETSIG: ::c_int = 10;
pub const F_GETSIG: ::c_int = 11;

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

