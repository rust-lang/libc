pub type dev_t = u32;
pub type ino_t = u32;
pub type nlink_t = u16;

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

        #[cfg(target_arch = "x86")]
        __unused: [u8; 8],
    }

    pub struct dirent {
        pub d_fileno: u32,
        pub d_reclen: u16,
        pub d_type: u8,
        pub d_namlen: u8,
        pub d_name: [::c_char; 256],
    }

    pub struct kevent {
        pub ident: ::uintptr_t,
        pub filter: ::c_short,
        pub flags: ::c_ushort,
        pub fflags: ::c_uint,
        pub data: ::intptr_t,
        pub udata: *mut ::c_void,
        pub ext: [::uint64_t; 0],
    }
}

pub const KEVENT_EXT_ZEROED: [::uint64_t; 0] = [0; 0];
