//! Android-specific definitions for linux-like values

pub type c_char = u8;
pub type c_long = i32;
pub type c_ulong = u32;
pub type clock_t = i32;
pub type time_t = i32;
pub type suseconds_t = i32;
pub type wchar_t = u32;
pub type off_t = i32;
pub type ino_t = u32;
pub type blkcnt_t = u32;
pub type blksize_t = u32;
pub type dev_t = u32;
pub type mode_t = u16;
pub type nlink_t = u16;
pub type useconds_t = i32;
pub type socklen_t = i32;
pub type pthread_t = c_long;
pub type pthread_mutexattr_t = ::c_long;
pub type sigset_t = c_ulong;

s! {
    pub struct stat {
        pub st_dev: ::c_ulonglong,
        __pad0: [::c_uchar; 4],
        __st_ino: ::ino_t,
        pub st_mode: ::c_uint,
        pub st_nlink: ::c_uint,
        pub st_uid: ::c_ulong,
        pub st_gid: ::c_ulong,
        pub st_rdev: ::c_ulonglong,
        __pad3: [::c_uchar; 4],
        pub st_size: ::c_longlong,
        pub st_blksize: blksize_t,
        pub st_blocks: ::c_ulonglong,
        pub st_atime: ::c_ulong,
        pub st_atime_nsec: ::c_ulong,
        pub st_mtime: ::c_ulong,
        pub st_mtime_nsec: ::c_ulong,
        pub st_ctime: ::c_ulong,
        pub st_ctime_nsec: ::c_ulong,
        pub st_ino: ::c_ulonglong,
    }

    pub struct dirent {
        pub d_ino: u64,
        pub d_off: i64,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }

    pub struct pthread_attr_t {
        pub flags: ::uint32_t,
        pub stack_base: *mut ::c_void,
        pub stack_size: ::size_t,
        pub guard_size: ::size_t,
        pub sched_policy: ::int32_t,
        pub sched_priority: ::int32_t,
    }

    pub struct pthread_mutex_t { value: ::c_int }

    pub struct pthread_cond_t { value: ::c_int }

    pub struct pthread_rwlock_t {
        lock: pthread_mutex_t,
        cond: pthread_cond_t,
        numLocks: ::c_int,
        writerThreadId: ::c_int,
        pendingReaders: ::c_int,
        pendingWriters: ::c_int,
        reserved: [*mut ::c_void; 4],
    }

    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: ::size_t
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub _pad: [::c_int; 29],
    }
}

pub const BUFSIZ: ::c_uint = 1024;
pub const FILENAME_MAX: ::c_uint = 1024;
pub const FOPEN_MAX: ::c_uint = 20;
pub const L_tmpnam: ::c_uint = 1024;
pub const TMP_MAX: ::c_uint = 308915776;
pub const _PC_NAME_MAX: ::c_int = 4;

pub const FIONBIO: ::c_int = 0x5421;

pub const _SC_ARG_MAX: ::c_int = 0;
pub const _SC_BC_BASE_MAX: ::c_int = 1;
pub const _SC_BC_DIM_MAX: ::c_int = 2;
pub const _SC_BC_SCALE_MAX: ::c_int = 3;
pub const _SC_BC_STRING_MAX: ::c_int = 4;
pub const _SC_CHILD_MAX: ::c_int = 5;
pub const _SC_CLK_TCK: ::c_int = 6;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 7;
pub const _SC_EXPR_NEST_MAX: ::c_int = 8;
pub const _SC_LINE_MAX: ::c_int = 9;
pub const _SC_NGROUPS_MAX: ::c_int = 10;
pub const _SC_OPEN_MAX: ::c_int = 11;
pub const _SC_2_C_BIND: ::c_int = 13;
pub const _SC_2_C_DEV: ::c_int = 14;
pub const _SC_2_C_VERSION: ::c_int = 15;
pub const _SC_2_CHAR_TERM: ::c_int = 16;
pub const _SC_2_FORT_DEV: ::c_int = 17;
pub const _SC_2_FORT_RUN: ::c_int = 18;
pub const _SC_2_LOCALEDEF: ::c_int = 19;
pub const _SC_2_SW_DEV: ::c_int = 20;
pub const _SC_2_UPE: ::c_int = 21;
pub const _SC_2_VERSION: ::c_int = 22;
pub const _SC_JOB_CONTROL: ::c_int = 23;
pub const _SC_SAVED_IDS: ::c_int = 24;
pub const _SC_VERSION: ::c_int = 25;
pub const _SC_RE_DUP_MAX: ::c_int = 26;
pub const _SC_STREAM_MAX: ::c_int = 27;
pub const _SC_TZNAME_MAX: ::c_int = 28;
pub const _SC_XOPEN_CRYPT: ::c_int = 29;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 30;
pub const _SC_XOPEN_SHM: ::c_int = 31;
pub const _SC_XOPEN_VERSION: ::c_int = 32;
pub const _SC_XOPEN_XCU_VERSION: ::c_int = 33;
pub const _SC_XOPEN_REALTIME: ::c_int = 34;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 35;
pub const _SC_XOPEN_LEGACY: ::c_int = 36;
pub const _SC_ATEXIT_MAX: ::c_int = 37;
pub const _SC_IOV_MAX: ::c_int = 38;
pub const _SC_PAGESIZE: ::c_int = 39;
pub const _SC_XOPEN_UNIX: ::c_int = 41;
pub const _SC_MQ_PRIO_MAX: ::c_int = 51;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 71;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 72;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 73;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 74;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 75;
pub const _SC_THREAD_STACK_MIN: ::c_int = 76;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 77;
pub const _SC_TTY_NAME_MAX: ::c_int = 78;
pub const _SC_THREADS: ::c_int = 79;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 80;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 81;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 82;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 83;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 84;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 85;

pub const PTHREAD_STACK_MIN: ::size_t = 8192;
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    value: 0,
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    value: 0,
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    lock: PTHREAD_MUTEX_INITIALIZER,
    cond: PTHREAD_COND_INITIALIZER,
    numLocks: 0,
    writerThreadId: 0,
    pendingReaders: 0,
    pendingWriters: 0,
    reserved: [0 as *mut _; 4],
};
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 1;

pub const FIOCLEX: ::c_ulong = 0x5451;

pub const SA_ONSTACK: ::c_ulong = 0x08000000;
pub const SA_SIGINFO: ::c_ulong = 0x00000004;
pub const SA_NOCLDWAIT: ::c_ulong = 0x00000002;

pub const SIGCHLD: ::c_int = 17;
pub const SIGBUS: ::c_int = 7;
pub const SIG_SETMASK: ::c_int = 2;

pub const O_ACCMODE: ::c_int = 3;
pub const O_SYNC: ::c_int = 0x1000;

pub const RUSAGE_CHILDREN: ::c_int = -1;

extern {
    pub fn madvise(addr: *const ::c_void, len: ::size_t, advice: ::c_int)
                   -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;
    pub fn putenv(string: *const ::c_char) -> ::c_int;
    pub fn readlink(path: *const ::c_char,
                    buf: *mut ::c_char,
                    bufsz: ::size_t)
                    -> ::c_int;
    pub fn msync(addr: *const ::c_void, len: ::size_t,
                 flags: ::c_int) -> ::c_int;
    pub fn mprotect(addr: *const ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn sysconf(name: ::c_int) -> ::c_long;
    pub fn usleep(secs: ::c_ulong) -> ::c_int;
    pub fn recvfrom(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                    flags: ::c_uint, addr: *const ::sockaddr,
                    addrlen: *mut ::socklen_t) -> ::ssize_t;
    pub fn send(socket: ::c_int, buf: *const ::c_void, len: ::size_t,
                flags: ::c_uint) -> ::ssize_t;
    pub fn recv(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                flags: ::c_uint) -> ::ssize_t;
    pub fn getnameinfo(sa: *const ::sockaddr,
                       salen: ::socklen_t,
                       host: *mut ::c_char,
                       hostlen: ::size_t,
                       serv: *mut ::c_char,
                       sevlen: ::size_t,
                       flags: ::c_int) -> ::c_int;
}

cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        mod b32;
        pub use self::b32::*;
    } else if #[cfg(target_pointer_width = "64")] {
        mod b64;
        pub use self::b64::*;
    } else {
        // ...
    }
}


