//! 32-bit specific definitions for linux-like values on platforms that aren't
//! MIPS or android

pub type blkcnt_t = i32;
pub type blksize_t = i32;
pub type dev_t = u64;
pub type mode_t = u32;
pub type nlink_t = u32;

s! {
    pub struct stat {
        pub st_dev: dev_t,
        __pad1: c_short,
        pub st_ino: ino_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: dev_t,
        __pad2: c_short,
        pub st_size: off_t,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        __unused4: c_long,
        __unused5: c_long,
    }

    pub struct pthread_attr_t {
        __size: [u32; 9]
    }
}
