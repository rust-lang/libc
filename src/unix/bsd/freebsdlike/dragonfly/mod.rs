pub type clock_t = u64;
pub type ino_t = u64;
pub type nlink_t = u32;
pub type blksize_t = i64;

pub type c_long = i64;
pub type c_ulong = u64;
pub type time_t = i64;
pub type suseconds_t = i64;

pub type uuid_t = ::uuid;

pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;

s! {
    pub struct dirent {
        pub d_fileno: ::ino_t,
        pub d_namlen: u16,
        pub d_type: u8,
        __unused1: u8,
        __unused2: u32,
        pub d_name: [::c_char; 256],
    }

    pub struct uuid {
        pub time_low: u32,
        pub time_mid: u16,
        pub time_hi_and_version: u16,
        pub clock_seq_hi_and_reserved: u8,
        pub clock_seq_low: u8,
        pub node: [u8; 6],
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        pub f_owner: ::uid_t,
        pub f_type: ::c_uint,
        pub f_syncreads: u64,
        pub f_syncwrites: u64,
        pub f_asyncreads: u64,
        pub f_asyncwrites: u64,
        pub f_fsid_uuid: ::uuid_t,
        pub f_uid_uuid: ::uuid_t,
    }

    pub struct stat {
        pub st_ino: ::ino_t,
        pub st_nlink: ::nlink_t,
        pub st_dev: ::dev_t,
        pub st_mode: ::mode_t,
        pub st_padding1: ::uint16_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_size: ::off_t,
        pub st_blocks: ::int64_t,
        pub st_blksize: ::uint32_t,
        pub st_flags: ::uint32_t,
        pub st_gen: ::uint32_t,
        pub st_lspare: ::int32_t,
        pub st_qspare1: ::int64_t,
        pub st_qspare2: ::int64_t,
    }
}

pub const RAND_MAX: ::c_int = 0x7fff_ffff;
pub const PTHREAD_STACK_MIN: ::size_t = 1024;
pub const KERN_PROC_PATHNAME: ::c_int = 9;
pub const SIGSTKSZ: ::size_t = 40960;
pub const MADV_INVAL: ::c_int = 10;
pub const O_CLOEXEC: ::c_int = 0x00020000;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const ELAST: ::c_int = 99;
pub const RLIMIT_POSIXLOCKS: ::c_int = 11;
pub const RLIM_NLIMITS: ::rlim_t = 12;

pub const Q_GETQUOTA: ::c_int = 0x300;
pub const Q_SETQUOTA: ::c_int = 0x400;

extern {
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn clock_gettime(clk_id: ::c_ulong, tp: *mut ::timespec) -> ::c_int;
}
