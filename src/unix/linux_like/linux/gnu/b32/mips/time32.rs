s! {
    pub struct stat64 {
        pub st_dev: ::c_ulong,
        st_pad1: [::c_long; 3],
        pub st_ino: ::ino64_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::c_ulong,
        st_pad2: [::c_long; 2],
        pub st_size: ::off64_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_blksize: ::blksize_t,
        st_pad3: ::c_long,
        pub st_blocks: ::blkcnt64_t,
        st_pad5: [::c_long; 14],
    }
}
