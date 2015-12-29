pub type c_long = i32;
pub type c_ulong = u32;

pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 32;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 24;

pub const F_GETLK: ::c_int = 12;
pub const F_SETLK: ::c_int = 13;
pub const F_SETLKW: ::c_int = 14;


s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        __st_dev_padding: ::c_int,
        __st_ino_truncated: ::c_long,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __st_rdev_padding: ::c_int,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atim: ::timespec,
        pub st_mtim: ::timespec,
        pub st_ctim: ::timespec,
        pub st_ino: ::ino_t,
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        __st_dev_padding: ::c_int,
        __st_ino_truncated: ::c_long,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __st_rdev_padding: ::c_int,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atim: ::timespec,
        pub st_mtim: ::timespec,
        pub st_ctim: ::timespec,
        pub st_ino: ::ino_t,
    }

    pub struct pthread_attr_t {
        __size: [u32; 9]
    }

    pub struct sigset_t {
        __val: [::c_ulong; 32],
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_segsz: ::size_t,
        pub shm_atime: ::time_t,
        __unused1: ::c_int,
        pub shm_dtime: ::time_t,
        __unused2: ::c_int,
        pub shm_ctime: ::time_t,
        __unused3: ::c_int,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::c_ulong,
        __pad1: ::c_ulong,
        __pad2: ::c_ulong,
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::c_int,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::socklen_t,
        pub msg_flags: ::c_int,
    }
}

cfg_if! {
    if #[cfg(any(target_arch = "x86"))] {
        mod x86;
        pub use self::x86::*;
    } else if #[cfg(any(target_arch = "arm"))] {
        mod arm;
        pub use self::arm::*;
    } else if #[cfg(any(target_arch = "asmjs"))] {
        mod asmjs;
        pub use self::asmjs::*;
    } else { }
}
