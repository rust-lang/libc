pub type blkcnt_t = i32;
pub type blksize_t = i32;
pub type dev_t = u64;
pub type mode_t = u32;
pub type nlink_t = u32;

s! {
    pub struct stat {
        pub st_dev: c_ulong,
        pub st_pad1: [c_long; 3],
        pub st_ino: ino_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: c_ulong,
        pub st_pad2: [c_long; 2],
        pub st_size: off_t,
        pub st_pad3: c_long,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub st_pad5: [c_long; 14],
    }
}
