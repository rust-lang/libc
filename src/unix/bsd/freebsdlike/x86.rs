pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type size_t = u32;
pub type ptrdiff_t = i32;
pub type clock_t = i32;
pub type time_t = i32;
pub type suseconds_t = i32;
pub type wchar_t = i32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intptr_t = i32;
pub type uintptr_t = u32;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type off_t = i64;
pub type dev_t = u32;
pub type ino_t = u32;
pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type useconds_t = u32;
pub type mode_t = u16;
pub type ssize_t = i32;
pub type nlink_t = u16;
pub type blksize_t = u32;
pub type blkcnt_t = i64;
pub type fflags_t = u32;
pub type pthread_attr_t = *mut c_void;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
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
        pub st_blocks: ::blkcnt_t,
        pub st_blksize: ::blksize_t,
        pub st_flags: ::fflags_t,
        pub st_gen: ::uint32_t,
        pub st_lspare: ::int32_t,
        pub st_birthtime: ::time_t,
        pub st_birthtime_nsec: ::c_long,
        __unused: [u8; 8],
    }
}
