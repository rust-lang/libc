pub type clock_t = ::c_uint;
pub type suseconds_t = ::c_int;
pub type dev_t = u64;
pub type blksize_t = ::int32_t;
pub type fsblkcnt_t = ::uint64_t;
pub type fsfilcnt_t = ::uint64_t;

s! {
    pub struct dirent {
        pub d_fileno: ::ino_t,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [::c_char; 512],
    }

    pub struct glob_t {
        pub gl_pathc:   ::size_t,
        __unused1:      ::c_int,
        pub gl_offs:    ::size_t,
        __unused2:      ::c_int,
        pub gl_pathv:   *mut *mut ::c_char,

        __unused3: *mut ::c_void,

        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
        __unused6: *mut ::c_void,
        __unused7: *mut ::c_void,
        __unused8: *mut ::c_void,
    }

    pub struct sigset_t {
        __bits: [u32; 4],
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_mode: ::mode_t,
        pub st_ino: ::ino_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_atime: ::time_t,
        pub st_atimensec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtimensec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctimensec: ::c_long,
        pub st_birthtime: ::time_t,
        pub st_birthtimensec: ::c_long,
        pub st_size: ::off_t,
        pub st_blocks: ::blkcnt_t,
        pub st_blksize: ::blksize_t,
        pub st_flags: ::uint32_t,
        pub st_gen: ::uint32_t,
        pub st_spare: [::uint32_t; 2],
    }

    pub struct statvfs {
        pub f_flag: ::c_ulong,
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_iosize: ::c_ulong,

        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_bresvd: ::fsblkcnt_t,

        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fresvd: ::fsfilcnt_t,

        pub f_syncreads: ::uint64_t,
        pub f_syncwrites: ::uint64_t,

        pub f_asyncreads: ::uint64_t,
        pub f_asyncwrites: ::uint64_t,

        pub f_fsidx: ::fsid_t,
        pub f_fsid: ::c_ulong,
        pub f_namemax: ::c_ulong,
        pub f_owner: ::uid_t,

        pub f_spare: [::uint32_t; 4],

        pub f_fstypename: [::c_char; 32],
        pub f_mntonname: [::c_char; 1024],
        pub f_mntfromname: [::c_char; 1024],
    }

    pub struct fsid_t {
        __fsid_val: [::int32_t; 2],
    }

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: ::socklen_t,
        pub ai_canonname: *mut ::c_char,
        pub ai_addr: *mut ::sockaddr,
        pub ai_next: *mut ::addrinfo,
    }

    pub struct sockaddr_storage {
        pub ss_len: u8,
        pub ss_family: ::sa_family_t,
        __ss_pad1: [u8; 6],
        __ss_pad2: i64,
        __ss_pad3: [u8; 112],
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_code: ::c_int,
        pub si_errno: ::c_int,
        __pad1: ::c_int,
        __pad2: [u64; 14],
    }

    pub struct pthread_attr_t {
        pta_magic: ::c_uint,
        pta_flags: ::c_int,
        pta_private: *mut ::c_void,
    }

    pub struct pthread_mutex_t {
        ptm_magic: ::c_uint,
        ptm_errorcheck: ::c_uchar,
        ptm_pad1: [u8; 3],
        ptm_interlock: ::c_uchar,
        ptm_pad2: [u8; 3],
        ptm_owner: ::pthread_t,
        ptm_waiters: *mut u8,
        ptm_recursed: ::c_uint,
        ptm_spare2: *mut ::c_void,
    }

    pub struct pthread_mutexattr_t {
        ptma_magic: ::c_uint,
        ptma_private: *mut ::c_void,
    }

    pub struct pthread_cond_t {
        ptc_magic: ::c_uint,
        ptc_lock: ::c_uchar,
        ptc_waiters_first: *mut u8,
        ptc_waiters_last: *mut u8,
        ptc_mutex: *mut ::pthread_mutex_t,
        ptc_private: *mut ::c_void,
    }

    pub struct pthread_rwlock_t {
        ptr_magic: ::c_uint,
        ptr_interlock: ::c_uchar,
        ptr_rblocked_first: *mut u8,
        ptr_rblocked_last: *mut u8,
        ptr_wblocked_first: *mut u8,
        ptr_wblocked_last: *mut u8,
        ptr_nreaders: ::c_uint,
        ptr_owner: ::pthread_t,
        ptr_private: *mut ::c_void,
    }
}

pub const O_CLOEXEC: ::c_int = 0x400000;

pub const MS_SYNC : ::c_int = 0x4;
pub const MS_INVALIDATE : ::c_int = 0x2;

pub const RLIM_NLIMITS: ::c_int = 12;

pub const ENOATTR : ::c_int = 93;
pub const EILSEQ : ::c_int = 85;
pub const EOVERFLOW : ::c_int = 84;
pub const ECANCELED : ::c_int = 87;
pub const EIDRM : ::c_int = 82;
pub const ENOMSG : ::c_int = 83;
pub const ENOTSUP : ::c_int = 86;
pub const ELAST : ::c_int = 96;

pub const F_DUPFD_CLOEXEC : ::c_int = 12;

pub const IPV6_JOIN_GROUP: ::c_int = 12;
pub const IPV6_LEAVE_GROUP: ::c_int = 13;

pub const SO_SNDTIMEO: ::c_int = 0x100b;
pub const SO_RCVTIMEO: ::c_int = 0x100c;

pub const KERN_PROC : ::c_int = 14;
pub const O_DSYNC : ::c_int = 0x10000;

pub const MAP_RENAME : ::c_int = 0x20;
pub const MAP_NORESERVE : ::c_int = 0x40;
pub const MAP_HASSEMAPHORE : ::c_int = 0x200;

pub const _SC_IOV_MAX : ::c_int = 32;
pub const _SC_GETGR_R_SIZE_MAX : ::c_int = 47;
pub const _SC_GETPW_R_SIZE_MAX : ::c_int = 48;
pub const _SC_LOGIN_NAME_MAX : ::c_int = 37;
pub const _SC_MQ_PRIO_MAX : ::c_int = 55;
pub const _SC_THREADS : ::c_int = 41;
pub const _SC_THREAD_ATTR_STACKADDR : ::c_int = 61;
pub const _SC_THREAD_ATTR_STACKSIZE : ::c_int = 62;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS : ::c_int = 57;
pub const _SC_THREAD_KEYS_MAX : ::c_int = 58;
pub const _SC_THREAD_PRIO_INHERIT : ::c_int = 64;
pub const _SC_THREAD_PRIO_PROTECT : ::c_int = 65;
pub const _SC_THREAD_PRIORITY_SCHEDULING : ::c_int = 63;
pub const _SC_THREAD_PROCESS_SHARED : ::c_int = 66;
pub const _SC_THREAD_SAFE_FUNCTIONS : ::c_int = 67;
pub const _SC_THREAD_STACK_MIN : ::c_int = 59;
pub const _SC_THREAD_THREADS_MAX : ::c_int = 60;
pub const _SC_TTY_NAME_MAX : ::c_int = 68;
pub const _SC_ATEXIT_MAX : ::c_int = 40;
pub const _SC_CLK_TCK : ::c_int = 39;
pub const _SC_AIO_LISTIO_MAX : ::c_int = 51;
pub const _SC_AIO_MAX : ::c_int = 52;
pub const _SC_ASYNCHRONOUS_IO : ::c_int = 50;
pub const _SC_MAPPED_FILES : ::c_int = 33;
pub const _SC_MEMLOCK : ::c_int = 34;
pub const _SC_MEMLOCK_RANGE : ::c_int = 35;
pub const _SC_MEMORY_PROTECTION : ::c_int = 36;
pub const _SC_MESSAGE_PASSING : ::c_int = 53;
pub const _SC_MQ_OPEN_MAX : ::c_int = 54;
pub const _SC_PRIORITY_SCHEDULING : ::c_int = 56;
pub const _SC_SEMAPHORES : ::c_int = 42;
pub const _SC_SHARED_MEMORY_OBJECTS : ::c_int = 87;
pub const _SC_SYNCHRONIZED_IO : ::c_int = 31;
pub const _SC_TIMERS : ::c_int = 44;

pub const SIGSTKSZ: ::size_t = 0xa000;

pub const FD_SETSIZE: usize = 0x100;

pub const ST_NOSUID: ::c_ulong = 8;

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    ptm_magic: 0x33330003,
    ptm_errorcheck: 0,
    ptm_interlock: 0,
    ptm_waiters: 0 as *mut _,
    ptm_owner: 0,
    ptm_pad1: [0; 3],
    ptm_pad2: [0; 3],
    ptm_recursed: 0,
    ptm_spare2: 0 as *mut _,
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    ptc_magic: 0x55550005,
    ptc_lock: 0,
    ptc_waiters_first: 0 as *mut _,
    ptc_waiters_last: 0 as *mut _,
    ptc_mutex: 0 as *mut _,
    ptc_private: 0 as *mut _,
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    ptr_magic: 0x99990009,
    ptr_interlock: 0,
    ptr_rblocked_first: 0 as *mut _,
    ptr_rblocked_last: 0 as *mut _,
    ptr_wblocked_first: 0 as *mut _,
    ptr_wblocked_last: 0 as *mut _,
    ptr_nreaders: 0,
    ptr_owner: 0,
    ptr_private: 0 as *mut _,
};
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 2;
pub const KERN_PROC_ARGS: ::c_int = 48;

extern {
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn sysctl(name: *const ::c_int,
                  namelen: ::c_uint,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *const ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
    pub fn sysctlbyname(name: *const ::c_char,
                        oldp: *mut ::c_void,
                        oldlenp: *mut ::size_t,
                        newp: *const ::c_void,
                        newlen: ::size_t)
                        -> ::c_int;
}
