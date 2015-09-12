//! Android specific definitions for 32-bit linux-like values

pub type blkcnt_t = u32;
pub type blksize_t = u32;
pub type dev_t = u32;
pub type mode_t = u16;
pub type nlink_t = u16;

s! {
    pub struct stat {
        pub st_dev: ::c_ulonglong,
        __pad0: [::c_uchar; 4],
        __st_ino: ::ino_t,
        pub st_mode: ::c_uint,
        pub st_nlink: ::c_uint,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::c_ulonglong,
        __pad3: [::c_uchar; 4],
        pub st_size: ::c_longlong,
        pub st_blksize: blksize_t,
        pub st_blocks: ::c_ulonglong,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_ulong,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_ulong,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_ulong,
        pub st_ino: ::c_ulonglong,
    }

    pub struct pthread_attr_t {
        pub flags: ::uint32_t,
        pub stack_base: *mut ::c_void,
        pub stack_size: ::size_t,
        pub guard_size: ::size_t,
        pub sched_policy: ::int32_t,
        pub sched_priority: ::int32_t,
    }
}
