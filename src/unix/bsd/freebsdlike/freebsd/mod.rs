pub type fflags_t = u32;
pub type clock_t = i32;
pub type ino_t = u32;
pub type nlink_t = u16;
pub type blksize_t = u32;

pub type fsblkcnt_t = ::uint64_t;
pub type fsfilcnt_t = ::uint64_t;

s! {
    pub struct dirent {
        pub d_fileno: u32,
        pub d_reclen: u16,
        pub d_type: u8,
        pub d_namlen: u8,
        pub d_name: [::c_char; 256],
    }

    pub struct statvfs {
        pub f_bavail: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_blocks: ::fsblkcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_bsize: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_fsid: ::c_ulong,
        pub f_namemax: ::c_ulong,
    }
}

pub const RAND_MAX: ::c_int = 0x7fff_fffd;
pub const PTHREAD_STACK_MIN: ::size_t = 2048;
pub const KERN_PROC_PATHNAME: ::c_int = 12;
pub const SIGSTKSZ: ::size_t = 34816;
pub const SF_NODISKIO: ::c_int = 0x00000001;
pub const SF_MNOWAIT: ::c_int = 0x00000002;
pub const SF_SYNC: ::c_int = 0x00000004;
pub const O_CLOEXEC: ::c_int = 0x00100000;
pub const F_GETLK: ::c_int = 11;
pub const F_SETLK: ::c_int = 12;
pub const F_SETLKW: ::c_int = 13;
pub const ELAST: ::c_int = 96;
pub const RLIMIT_NPTS: ::c_int = 11;
pub const RLIMIT_SWAP: ::c_int = 12;
pub const RLIM_NLIMITS: ::rlim_t = 13;

pub const Q_GETQUOTA: ::c_int = 0x700;
pub const Q_SETQUOTA: ::c_int = 0x800;

pub const POSIX_FADV_NORMAL: ::c_int = 0;
pub const POSIX_FADV_RANDOM: ::c_int = 1;
pub const POSIX_FADV_SEQUENTIAL: ::c_int = 2;
pub const POSIX_FADV_WILLNEED: ::c_int = 3;
pub const POSIX_FADV_DONTNEED: ::c_int = 4;
pub const POSIX_FADV_NOREUSE: ::c_int = 5;

pub const MADV_PROTECT: ::c_int = 10;
pub const RUSAGE_THREAD: ::c_int = 1;

extern {
    pub fn __error() -> *mut ::c_int;

    pub fn mprotect(addr: *const ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;

    pub fn clock_gettime(clk_id: ::c_int, tp: *mut ::timespec) -> ::c_int;

    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t,
                           len: ::off_t) -> ::c_int;
    pub fn posix_fadvise(fd: ::c_int, offset: ::off_t, len: ::off_t,
                         advise: ::c_int) -> ::c_int;
    pub fn mkostemp(template: *mut ::c_char, flags: ::c_int) -> ::c_int;
    pub fn mkostemps(template: *mut ::c_char,
                     suffixlen: ::c_int,
                     flags: ::c_int) -> ::c_int;
}

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        mod x86;
        pub use self::x86::*;
    } else if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    } else {
        // ...
    }
}
