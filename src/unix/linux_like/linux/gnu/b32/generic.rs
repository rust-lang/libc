s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        #[cfg(not(gnu_time64_abi))]
        _pad1: i32,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        #[cfg(not(gnu_time64_abi))]
        __pad2: i32,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        #[cfg(gnu_time64_abi)]
        __pad3: i32,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        #[cfg(gnu_time64_abi)]
        __pad4: i32,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        #[cfg(gnu_time64_abi)]
        __pad5: i32,
    }
}
