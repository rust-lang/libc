//! Apple (ios/darwin)-specific definitions
//!
//! This covers *-apple-* triples currently

pub type clock_t = c_ulong;
pub type time_t = c_long;
pub type suseconds_t = i32;
pub type dev_t = i32;
pub type ino_t = u64;
pub type mode_t = u16;
pub type nlink_t = u16;
pub type blksize_t = i32;
pub type rlim_t = u64;
pub type mach_timebase_info_data_t = mach_timebase_info;
pub type pthread_key_t = c_ulong;
pub type sigset_t = u32;
pub type fsblkcnt_t = ::c_uint;
pub type fsfilcnt_t = ::c_uint;
pub type speed_t = ::c_ulong;
pub type tcflag_t = ::c_ulong;

pub enum timezone {}

s! {
    pub struct glob_t {
        pub gl_pathc:  ::size_t,
        __unused1: ::c_int,
        pub gl_offs:   ::size_t,
        __unused2: ::c_int,
        pub gl_pathv:  *mut *mut ::c_char,

        __unused3: *mut ::c_void,

        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
        __unused6: *mut ::c_void,
        __unused7: *mut ::c_void,
        __unused8: *mut ::c_void,
    }

    pub struct sockaddr_storage {
        pub ss_len: u8,
        pub ss_family: ::sa_family_t,
        __ss_pad1: [u8; 6],
        __ss_align: i64,
        __ss_pad2: [u8; 112],
    }

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: ::socklen_t,
        pub ai_canonname: *mut ::c_char,
        pub ai_addr: *mut ::sockaddr,
        pub ai_next: *mut addrinfo,
    }

    pub struct mach_timebase_info {
        pub numer: u32,
        pub denom: u32,
    }

    pub struct stat {
        pub st_dev: dev_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_ino: ino_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: dev_t,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        pub st_birthtime: time_t,
        pub st_birthtime_nsec: c_long,
        pub st_size: ::off_t,
        pub st_blocks: ::blkcnt_t,
        pub st_blksize: blksize_t,
        pub st_flags: ::uint32_t,
        pub st_gen: ::uint32_t,
        pub st_lspare: ::int32_t,
        pub st_qspare: [::int64_t; 2],
    }

    pub struct dirent {
        pub d_ino: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [::c_char; 1024],
    }

    pub struct pthread_mutex_t {
        __sig: ::c_long,
        __opaque: [u8; __PTHREAD_MUTEX_SIZE__],
    }

    pub struct pthread_mutexattr_t {
        __sig: ::c_long,
        __opaque: [u8; 8],
    }

    pub struct pthread_cond_t {
        __sig: ::c_long,
        __opaque: [u8; __PTHREAD_COND_SIZE__],
    }

    pub struct pthread_rwlock_t {
        __sig: ::c_long,
        __opaque: [u8; __PTHREAD_RWLOCK_SIZE__],
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub si_pid: ::pid_t,
        pub si_uid: ::uid_t,
        pub si_status: ::c_int,
        pub si_addr: *mut ::c_void,
        _pad: [usize; 9],
    }

    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: sigset_t,
        pub sa_flags: ::c_int,
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_size: ::size_t,
        pub ss_flags: ::c_int,
    }

    pub struct fstore_t {
        pub fst_flags: ::c_uint,
        pub fst_posmode: ::c_int,
        pub fst_offset: ::off_t,
        pub fst_length: ::off_t,
        pub fst_bytesalloc: ::off_t,
    }

    pub struct radvisory {
        pub ra_offset: ::off_t,
        pub ra_count: ::c_int,
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
    }

    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: ::sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [::c_char; 8],
    }

    pub struct statfs {
        pub f_bsize: ::uint32_t,
        pub f_iosize: ::int32_t,
        pub f_blocks: ::uint64_t,
        pub f_bfree: ::uint64_t,
        pub f_bavail: ::uint64_t,
        pub f_files: ::uint64_t,
        pub f_ffree: ::uint64_t,
        pub f_fsid: ::fsid_t,
        pub f_owner: ::uid_t,
        pub f_type: ::uint32_t,
        pub f_flags: ::uint32_t,
        pub f_fssubtype: ::uint32_t,
        pub f_fstypename: [::c_char; 16],
        pub f_mntonname: [::c_char; 1024],
        pub f_mntfromname: [::c_char; 1024],
        pub f_reserved: [::uint32_t; 8],
    }

    // FIXME: this should have align 4 but it's got align 8 on 64-bit
    pub struct kevent {
        pub ident: ::uintptr_t,
        pub filter: ::int16_t,
        pub flags: ::uint16_t,
        pub fflags: ::uint32_t,
        pub data: ::intptr_t,
        pub udata: *mut ::c_void,
    }

    pub struct kevent64_s {
        pub ident: ::uint64_t,
        pub filter: ::int16_t,
        pub flags: ::uint16_t,
        pub fflags: ::uint32_t,
        pub data: ::int64_t,
        pub udata: ::uint64_t,
        pub ext: [::uint64_t; 2],
    }

    pub struct dqblk {
        pub dqb_bhardlimit: ::uint64_t,
        pub dqb_bsoftlimit: ::uint64_t,
        pub dqb_curbytes: ::uint64_t,
        pub dqb_ihardlimit: ::uint32_t,
        pub dqb_isoftlimit: ::uint32_t,
        pub dqb_curinodes: ::uint32_t,
        pub dqb_btime: ::uint32_t,
        pub dqb_itime: ::uint32_t,
        pub dqb_id: ::uint32_t,
        pub dqb_spare: [::uint32_t; 4],
    }

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_cc: [::cc_t; ::NCCS],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }

    pub struct flock {
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_pid: ::pid_t,
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
    }

    pub struct sf_hdtr {
        pub headers: *mut ::iovec,
        pub hdr_cnt: ::c_int,
        pub trailers: *mut ::iovec,
        pub trl_cnt: ::c_int,
    }
}

pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;
pub const RAND_MAX: ::c_int = 2147483647;
pub const EOF: ::c_int = -1;
pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
pub const _IOFBF: ::c_int = 0;
pub const _IONBF: ::c_int = 2;
pub const _IOLBF: ::c_int = 1;
pub const BUFSIZ: ::c_uint = 1024;
pub const FOPEN_MAX: ::c_uint = 20;
pub const FILENAME_MAX: ::c_uint = 1024;
pub const L_tmpnam: ::c_uint = 1024;
pub const TMP_MAX: ::c_uint = 308915776;
pub const _PC_NAME_MAX: ::c_int = 4;

pub const O_RDONLY: ::c_int = 0;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDWR: ::c_int = 2;
pub const O_APPEND: ::c_int = 8;
pub const O_CREAT: ::c_int = 512;
pub const O_EXCL: ::c_int = 2048;
pub const O_NOCTTY: ::c_int = 131072;
pub const O_TRUNC: ::c_int = 1024;
pub const O_CLOEXEC: ::c_int = 0x1000000;
pub const O_DIRECTORY: ::c_int = 0x100000;
pub const S_IFIFO: mode_t = 4096;
pub const S_IFCHR: mode_t = 8192;
pub const S_IFBLK: mode_t = 24576;
pub const S_IFDIR: mode_t = 16384;
pub const S_IFREG: mode_t = 32768;
pub const S_IFLNK: mode_t = 40960;
pub const S_IFSOCK: mode_t = 49152;
pub const S_IFMT: mode_t = 61440;
pub const S_IEXEC: mode_t = 64;
pub const S_IWRITE: mode_t = 128;
pub const S_IREAD: mode_t = 256;
pub const S_IRWXU: mode_t = 448;
pub const S_IXUSR: mode_t = 64;
pub const S_IWUSR: mode_t = 128;
pub const S_IRUSR: mode_t = 256;
pub const S_IRWXG: mode_t = 56;
pub const S_IXGRP: mode_t = 8;
pub const S_IWGRP: mode_t = 16;
pub const S_IRGRP: mode_t = 32;
pub const S_IRWXO: mode_t = 7;
pub const S_IXOTH: mode_t = 1;
pub const S_IWOTH: mode_t = 2;
pub const S_IROTH: mode_t = 4;
pub const F_OK: ::c_int = 0;
pub const R_OK: ::c_int = 4;
pub const W_OK: ::c_int = 2;
pub const X_OK: ::c_int = 1;
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;
pub const F_LOCK: ::c_int = 1;
pub const F_TEST: ::c_int = 3;
pub const F_TLOCK: ::c_int = 2;
pub const F_ULOCK: ::c_int = 0;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const SIGHUP: ::c_int = 1;
pub const SIGINT: ::c_int = 2;
pub const SIGQUIT: ::c_int = 3;
pub const SIGILL: ::c_int = 4;
pub const SIGABRT: ::c_int = 6;
pub const SIGFPE: ::c_int = 8;
pub const SIGKILL: ::c_int = 9;
pub const SIGSEGV: ::c_int = 11;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGTERM: ::c_int = 15;

pub const PROT_NONE: ::c_int = 0;
pub const PROT_READ: ::c_int = 1;
pub const PROT_WRITE: ::c_int = 2;
pub const PROT_EXEC: ::c_int = 4;

pub const MAP_FILE: ::c_int = 0x0000;
pub const MAP_SHARED: ::c_int = 0x0001;
pub const MAP_PRIVATE: ::c_int = 0x0002;
pub const MAP_FIXED: ::c_int = 0x0010;
pub const MAP_ANON: ::c_int = 0x1000;

pub const MAP_FAILED: *mut ::c_void = !0 as *mut ::c_void;

pub const MCL_CURRENT: ::c_int = 0x0001;
pub const MCL_FUTURE: ::c_int = 0x0002;

pub const MS_ASYNC: ::c_int = 0x0001;
pub const MS_INVALIDATE: ::c_int = 0x0002;
pub const MS_SYNC: ::c_int = 0x0010;

pub const MS_KILLPAGES: ::c_int = 0x0004;
pub const MS_DEACTIVATE: ::c_int = 0x0008;

pub const EPERM: ::c_int = 1;
pub const ENOENT: ::c_int = 2;
pub const ESRCH: ::c_int = 3;
pub const EINTR: ::c_int = 4;
pub const EIO: ::c_int = 5;
pub const ENXIO: ::c_int = 6;
pub const E2BIG: ::c_int = 7;
pub const ENOEXEC: ::c_int = 8;
pub const EBADF: ::c_int = 9;
pub const ECHILD: ::c_int = 10;
pub const EDEADLK: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
pub const ENOTBLK: ::c_int = 15;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const EXDEV: ::c_int = 18;
pub const ENODEV: ::c_int = 19;
pub const ENOTDIR: ::c_int = 20;
pub const EISDIR: ::c_int = 21;
pub const EINVAL: ::c_int = 22;
pub const ENFILE: ::c_int = 23;
pub const EMFILE: ::c_int = 24;
pub const ENOTTY: ::c_int = 25;
pub const ETXTBSY: ::c_int = 26;
pub const EFBIG: ::c_int = 27;
pub const ENOSPC: ::c_int = 28;
pub const ESPIPE: ::c_int = 29;
pub const EROFS: ::c_int = 30;
pub const EMLINK: ::c_int = 31;
pub const EPIPE: ::c_int = 32;
pub const EDOM: ::c_int = 33;
pub const ERANGE: ::c_int = 34;
pub const EAGAIN: ::c_int = 35;
pub const EWOULDBLOCK: ::c_int = EAGAIN;
pub const EINPROGRESS: ::c_int = 36;
pub const EALREADY: ::c_int = 37;
pub const ENOTSOCK: ::c_int = 38;
pub const EDESTADDRREQ: ::c_int = 39;
pub const EMSGSIZE: ::c_int = 40;
pub const EPROTOTYPE: ::c_int = 41;
pub const ENOPROTOOPT: ::c_int = 42;
pub const EPROTONOSUPPORT: ::c_int = 43;
pub const ESOCKTNOSUPPORT: ::c_int = 44;
pub const ENOTSUP: ::c_int = 45;
pub const EPFNOSUPPORT: ::c_int = 46;
pub const EAFNOSUPPORT: ::c_int = 47;
pub const EADDRINUSE: ::c_int = 48;
pub const EADDRNOTAVAIL: ::c_int = 49;
pub const ENETDOWN: ::c_int = 50;
pub const ENETUNREACH: ::c_int = 51;
pub const ENETRESET: ::c_int = 52;
pub const ECONNABORTED: ::c_int = 53;
pub const ECONNRESET: ::c_int = 54;
pub const ENOBUFS: ::c_int = 55;
pub const EISCONN: ::c_int = 56;
pub const ENOTCONN: ::c_int = 57;
pub const ESHUTDOWN: ::c_int = 58;
pub const ETOOMANYREFS: ::c_int = 59;
pub const ETIMEDOUT: ::c_int = 60;
pub const ECONNREFUSED: ::c_int = 61;
pub const ELOOP: ::c_int = 62;
pub const ENAMETOOLONG: ::c_int = 63;
pub const EHOSTDOWN: ::c_int = 64;
pub const EHOSTUNREACH: ::c_int = 65;
pub const ENOTEMPTY: ::c_int = 66;
pub const EPROCLIM: ::c_int = 67;
pub const EUSERS: ::c_int = 68;
pub const EDQUOT: ::c_int = 69;
pub const ESTALE: ::c_int = 70;
pub const EREMOTE: ::c_int = 71;
pub const EBADRPC: ::c_int = 72;
pub const ERPCMISMATCH: ::c_int = 73;
pub const EPROGUNAVAIL: ::c_int = 74;
pub const EPROGMISMATCH: ::c_int = 75;
pub const EPROCUNAVAIL: ::c_int = 76;
pub const ENOLCK: ::c_int = 77;
pub const ENOSYS: ::c_int = 78;
pub const EFTYPE: ::c_int = 79;
pub const EAUTH: ::c_int = 80;
pub const ENEEDAUTH: ::c_int = 81;
pub const EPWROFF: ::c_int = 82;
pub const EDEVERR: ::c_int = 83;
pub const EOVERFLOW: ::c_int = 84;
pub const EBADEXEC: ::c_int = 85;
pub const EBADARCH: ::c_int = 86;
pub const ESHLIBVERS: ::c_int = 87;
pub const EBADMACHO: ::c_int = 88;
pub const ECANCELED: ::c_int = 89;
pub const EIDRM: ::c_int = 90;
pub const ENOMSG: ::c_int = 91;
pub const EILSEQ: ::c_int = 92;
pub const ENOATTR: ::c_int = 93;
pub const EBADMSG: ::c_int = 94;
pub const EMULTIHOP: ::c_int = 95;
pub const ENODATA: ::c_int = 96;
pub const ENOLINK: ::c_int = 97;
pub const ENOSR: ::c_int = 98;
pub const ENOSTR: ::c_int = 99;
pub const EPROTO: ::c_int = 100;
pub const ETIME: ::c_int = 101;
pub const EOPNOTSUPP: ::c_int = 102;
pub const ENOPOLICY: ::c_int = 103;
pub const ENOTRECOVERABLE: ::c_int = 104;
pub const EOWNERDEAD: ::c_int = 105;
pub const EQFULL: ::c_int = 106;
pub const ELAST: ::c_int = 106;

pub const F_DUPFD: ::c_int = 0;
pub const F_DUPFD_CLOEXEC: ::c_int = 67;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;
pub const F_PREALLOCATE: ::c_int = 42;
pub const F_RDADVISE: ::c_int = 44;
pub const F_RDAHEAD: ::c_int = 45;
pub const F_NOCACHE: ::c_int = 48;
pub const F_GETPATH: ::c_int = 50;
pub const F_FULLFSYNC: ::c_int = 51;
pub const F_FREEZE_FS: ::c_int = 53;
pub const F_THAW_FS: ::c_int = 54;
pub const F_GLOBAL_NOCACHE: ::c_int = 55;
pub const F_NODIRECT: ::c_int = 62;

pub const F_ALLOCATECONTIG: ::c_uint = 0x02;
pub const F_ALLOCATEALL: ::c_uint = 0x04;

pub const F_PEOFPOSMODE: ::c_int = 3;
pub const F_VOLPOSMODE: ::c_int = 4;

pub const O_ACCMODE: ::c_int = 3;

pub const SIGTRAP: ::c_int = 5;

pub const GLOB_APPEND  : ::c_int = 0x0001;
pub const GLOB_DOOFFS  : ::c_int = 0x0002;
pub const GLOB_ERR     : ::c_int = 0x0004;
pub const GLOB_MARK    : ::c_int = 0x0008;
pub const GLOB_NOCHECK : ::c_int = 0x0010;
pub const GLOB_NOSORT  : ::c_int = 0x0020;
pub const GLOB_NOESCAPE: ::c_int = 0x2000;

pub const GLOB_NOSPACE : ::c_int = -1;
pub const GLOB_ABORTED : ::c_int = -2;
pub const GLOB_NOMATCH : ::c_int = -3;

pub const POSIX_MADV_NORMAL: ::c_int = 0;
pub const POSIX_MADV_RANDOM: ::c_int = 1;
pub const POSIX_MADV_SEQUENTIAL: ::c_int = 2;
pub const POSIX_MADV_WILLNEED: ::c_int = 3;
pub const POSIX_MADV_DONTNEED: ::c_int = 4;

pub const _SC_IOV_MAX: ::c_int = 56;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 70;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 71;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 73;
pub const _SC_MQ_PRIO_MAX: ::c_int = 75;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 82;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 83;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 85;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 86;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 87;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 88;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 89;
pub const _SC_THREAD_PROCESS_SHARED: ::c_int = 90;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 91;
pub const _SC_THREAD_STACK_MIN: ::c_int = 93;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 94;
pub const _SC_THREADS: ::c_int = 96;
pub const _SC_TTY_NAME_MAX: ::c_int = 101;
pub const _SC_ATEXIT_MAX: ::c_int = 107;
pub const _SC_XOPEN_CRYPT: ::c_int = 108;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 109;
pub const _SC_XOPEN_LEGACY: ::c_int = 110;
pub const _SC_XOPEN_REALTIME: ::c_int = 111;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 112;
pub const _SC_XOPEN_SHM: ::c_int = 113;
pub const _SC_XOPEN_UNIX: ::c_int = 115;
pub const _SC_XOPEN_VERSION: ::c_int = 116;
pub const _SC_XOPEN_XCU_VERSION: ::c_int = 121;

pub const PTHREAD_CREATE_JOINABLE: ::c_int = 1;
pub const PTHREAD_CREATE_DETACHED: ::c_int = 2;
pub const PTHREAD_STACK_MIN: ::size_t = 8192;

pub const RLIMIT_CPU: ::c_int = 0;
pub const RLIMIT_FSIZE: ::c_int = 1;
pub const RLIMIT_DATA: ::c_int = 2;
pub const RLIMIT_STACK: ::c_int = 3;
pub const RLIMIT_CORE: ::c_int = 4;
pub const RLIMIT_AS: ::c_int = 5;
pub const RLIMIT_MEMLOCK: ::c_int = 6;
pub const RLIMIT_NPROC: ::c_int = 7;
pub const RLIMIT_NOFILE: ::c_int = 8;
pub const RLIM_NLIMITS: ::c_int = 9;
pub const _RLIMIT_POSIX_FLAG: ::c_int = 0x1000;

pub const RLIM_INFINITY: rlim_t = 0x7fff_ffff_ffff_ffff;

pub const RUSAGE_SELF: ::c_int = 0;
pub const RUSAGE_CHILDREN: ::c_int = -1;

pub const MADV_NORMAL: ::c_int = 0;
pub const MADV_RANDOM: ::c_int = 1;
pub const MADV_SEQUENTIAL: ::c_int = 2;
pub const MADV_WILLNEED: ::c_int = 3;
pub const MADV_DONTNEED: ::c_int = 4;
pub const MADV_FREE: ::c_int = 5;
pub const MADV_ZERO_WIRED_PAGES: ::c_int = 6;
pub const MADV_FREE_REUSABLE: ::c_int = 7;
pub const MADV_FREE_REUSE: ::c_int = 8;
pub const MADV_CAN_REUSE: ::c_int = 9;

pub const MINCORE_INCORE: ::c_int =  0x1;
pub const MINCORE_REFERENCED: ::c_int = 0x2;
pub const MINCORE_MODIFIED: ::c_int = 0x4;
pub const MINCORE_REFERENCED_OTHER: ::c_int = 0x8;
pub const MINCORE_MODIFIED_OTHER: ::c_int = 0x10;

pub const AF_UNIX: ::c_int = 1;
pub const AF_INET: ::c_int = 2;
pub const AF_INET6: ::c_int = 30;
pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_RAW: ::c_int = 3;
pub const IPPROTO_TCP: ::c_int = 6;
pub const IPPROTO_IP: ::c_int = 0;
pub const IPPROTO_IPV6: ::c_int = 41;
pub const IP_MULTICAST_TTL: ::c_int = 10;
pub const IP_MULTICAST_LOOP: ::c_int = 11;
pub const IP_TTL: ::c_int = 4;
pub const IP_HDRINCL: ::c_int = 2;
pub const IP_ADD_MEMBERSHIP: ::c_int = 12;
pub const IP_DROP_MEMBERSHIP: ::c_int = 13;
pub const IPV6_JOIN_GROUP: ::c_int = 12;
pub const IPV6_LEAVE_GROUP: ::c_int = 13;

pub const TCP_NODELAY: ::c_int = 0x01;
pub const TCP_KEEPALIVE: ::c_int = 0x10;
pub const SOL_SOCKET: ::c_int = 0xffff;

pub const SO_DEBUG: ::c_int = 0x01;
pub const SO_ACCEPTCONN: ::c_int = 0x0002;
pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_BROADCAST: ::c_int = 0x0020;
pub const SO_USELOOPBACK: ::c_int = 0x0040;
pub const SO_LINGER: ::c_int = 0x0080;
pub const SO_OOBINLINE: ::c_int = 0x0100;
pub const SO_REUSEPORT: ::c_int = 0x0200;
pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_SNDLOWAT: ::c_int = 0x1003;
pub const SO_RCVLOWAT: ::c_int = 0x1004;
pub const SO_SNDTIMEO: ::c_int = 0x1005;
pub const SO_RCVTIMEO: ::c_int = 0x1006;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_TYPE: ::c_int = 0x1008;

pub const IFF_LOOPBACK: ::c_int = 0x8;

pub const SHUT_RD: ::c_int = 0;
pub const SHUT_WR: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;

pub const LOCK_SH: ::c_int = 1;
pub const LOCK_EX: ::c_int = 2;
pub const LOCK_NB: ::c_int = 4;
pub const LOCK_UN: ::c_int = 8;

pub const O_DSYNC: ::c_int = 4194304;
pub const O_SYNC: ::c_int = 128;
pub const O_NONBLOCK: ::c_int = 4;

pub const MAP_COPY: ::c_int = 0x0002;
pub const MAP_RENAME: ::c_int = 0x0020;
pub const MAP_NORESERVE: ::c_int = 0x0040;
pub const MAP_NOEXTEND: ::c_int = 0x0100;
pub const MAP_HASSEMAPHORE: ::c_int = 0x0200;
pub const MAP_NOCACHE: ::c_int = 0x0400;
pub const MAP_JIT: ::c_int = 0x0800;

pub const IPPROTO_RAW: ::c_int = 255;

pub const SO_NREAD: ::c_int = 0x1020;
pub const SO_NKE: ::c_int = 0x1021;
pub const SO_NOSIGPIPE: ::c_int = 0x1022;
pub const SO_NOADDRERR: ::c_int = 0x1023;
pub const SO_NWRITE: ::c_int = 0x1024;
pub const SO_DONTTRUNC: ::c_int = 0x2000;
pub const SO_WANTMORE: ::c_int = 0x4000;
pub const SO_WANTOOBFLAG: ::c_int = 0x8000;

pub const _SC_ARG_MAX: ::c_int = 1;
pub const _SC_CHILD_MAX: ::c_int = 2;
pub const _SC_CLK_TCK: ::c_int = 3;
pub const _SC_NGROUPS_MAX: ::c_int = 4;
pub const _SC_OPEN_MAX: ::c_int = 5;
pub const _SC_JOB_CONTROL: ::c_int = 6;
pub const _SC_SAVED_IDS: ::c_int = 7;
pub const _SC_VERSION: ::c_int = 8;
pub const _SC_BC_BASE_MAX: ::c_int = 9;
pub const _SC_BC_DIM_MAX: ::c_int = 10;
pub const _SC_BC_SCALE_MAX: ::c_int = 11;
pub const _SC_BC_STRING_MAX: ::c_int = 12;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 13;
pub const _SC_EXPR_NEST_MAX: ::c_int = 14;
pub const _SC_LINE_MAX: ::c_int = 15;
pub const _SC_RE_DUP_MAX: ::c_int = 16;
pub const _SC_2_VERSION: ::c_int = 17;
pub const _SC_2_C_BIND: ::c_int = 18;
pub const _SC_2_C_DEV: ::c_int = 19;
pub const _SC_2_CHAR_TERM: ::c_int = 20;
pub const _SC_2_FORT_DEV: ::c_int = 21;
pub const _SC_2_FORT_RUN: ::c_int = 22;
pub const _SC_2_LOCALEDEF: ::c_int = 23;
pub const _SC_2_SW_DEV: ::c_int = 24;
pub const _SC_2_UPE: ::c_int = 25;
pub const _SC_STREAM_MAX: ::c_int = 26;
pub const _SC_TZNAME_MAX: ::c_int = 27;
pub const _SC_ASYNCHRONOUS_IO: ::c_int = 28;
pub const _SC_PAGESIZE: ::c_int = 29;
pub const _SC_MEMLOCK: ::c_int = 30;
pub const _SC_MEMLOCK_RANGE: ::c_int = 31;
pub const _SC_MEMORY_PROTECTION: ::c_int = 32;
pub const _SC_MESSAGE_PASSING: ::c_int = 33;
pub const _SC_PRIORITIZED_IO: ::c_int = 34;
pub const _SC_PRIORITY_SCHEDULING: ::c_int = 35;
pub const _SC_REALTIME_SIGNALS: ::c_int = 36;
pub const _SC_SEMAPHORES: ::c_int = 37;
pub const _SC_FSYNC: ::c_int = 38;
pub const _SC_SHARED_MEMORY_OBJECTS: ::c_int = 39;
pub const _SC_SYNCHRONIZED_IO: ::c_int = 40;
pub const _SC_TIMERS: ::c_int = 41;
pub const _SC_AIO_LISTIO_MAX: ::c_int = 42;
pub const _SC_AIO_MAX: ::c_int = 43;
pub const _SC_AIO_PRIO_DELTA_MAX: ::c_int = 44;
pub const _SC_DELAYTIMER_MAX: ::c_int = 45;
pub const _SC_MQ_OPEN_MAX: ::c_int = 46;
pub const _SC_MAPPED_FILES: ::c_int = 47;
pub const _SC_RTSIG_MAX: ::c_int = 48;
pub const _SC_SEM_NSEMS_MAX: ::c_int = 49;
pub const _SC_SEM_VALUE_MAX: ::c_int = 50;
pub const _SC_SIGQUEUE_MAX: ::c_int = 51;
pub const _SC_TIMER_MAX: ::c_int = 52;
pub const _SC_NPROCESSORS_CONF: ::c_int = 57;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 58;
pub const _SC_2_PBS: ::c_int = 59;
pub const _SC_2_PBS_ACCOUNTING: ::c_int = 60;
pub const _SC_2_PBS_CHECKPOINT: ::c_int = 61;
pub const _SC_2_PBS_LOCATE: ::c_int = 62;
pub const _SC_2_PBS_MESSAGE: ::c_int = 63;
pub const _SC_2_PBS_TRACK: ::c_int = 64;
pub const _SC_ADVISORY_INFO: ::c_int = 65;
pub const _SC_BARRIERS: ::c_int = 66;
pub const _SC_CLOCK_SELECTION: ::c_int = 67;
pub const _SC_CPUTIME: ::c_int = 68;
pub const _SC_FILE_LOCKING: ::c_int = 69;
pub const _SC_HOST_NAME_MAX: ::c_int = 72;
pub const _SC_MONOTONIC_CLOCK: ::c_int = 74;
pub const _SC_READER_WRITER_LOCKS: ::c_int = 76;
pub const _SC_REGEXP: ::c_int = 77;
pub const _SC_SHELL: ::c_int = 78;
pub const _SC_SPAWN: ::c_int = 79;
pub const _SC_SPIN_LOCKS: ::c_int = 80;
pub const _SC_SPORADIC_SERVER: ::c_int = 81;
pub const _SC_THREAD_CPUTIME: ::c_int = 84;
pub const _SC_THREAD_SPORADIC_SERVER: ::c_int = 92;
pub const _SC_TIMEOUTS: ::c_int = 95;
pub const _SC_TRACE: ::c_int = 97;
pub const _SC_TRACE_EVENT_FILTER: ::c_int = 98;
pub const _SC_TRACE_INHERIT: ::c_int = 99;
pub const _SC_TRACE_LOG: ::c_int = 100;
pub const _SC_TYPED_MEMORY_OBJECTS: ::c_int = 102;
pub const _SC_V6_ILP32_OFF32: ::c_int = 103;
pub const _SC_V6_ILP32_OFFBIG: ::c_int = 104;
pub const _SC_V6_LP64_OFF64: ::c_int = 105;
pub const _SC_V6_LPBIG_OFFBIG: ::c_int = 106;
pub const _SC_IPV6: ::c_int = 118;
pub const _SC_RAW_SOCKETS: ::c_int = 119;
pub const _SC_SYMLOOP_MAX: ::c_int = 120;
pub const _SC_PAGE_SIZE: ::c_int = _SC_PAGESIZE;
pub const _SC_XOPEN_STREAMS: ::c_int = 114;
pub const _SC_XBS5_ILP32_OFF32: ::c_int = 122;
pub const _SC_XBS5_ILP32_OFFBIG: ::c_int = 123;
pub const _SC_XBS5_LP64_OFF64: ::c_int = 124;
pub const _SC_XBS5_LPBIG_OFFBIG: ::c_int = 125;
pub const _SC_SS_REPL_MAX: ::c_int = 126;
pub const _SC_TRACE_EVENT_NAME_MAX: ::c_int = 127;
pub const _SC_TRACE_NAME_MAX: ::c_int = 128;
pub const _SC_TRACE_SYS_MAX: ::c_int = 129;
pub const _SC_TRACE_USER_EVENT_MAX: ::c_int = 130;
pub const _SC_PASS_MAX: ::c_int = 131;

pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 2;
pub const _PTHREAD_MUTEX_SIG_init: ::c_long = 0x32AAABA7;
pub const _PTHREAD_COND_SIG_init: ::c_long = 0x3CB0B1BB;
pub const _PTHREAD_RWLOCK_SIG_init: ::c_long = 0x2DA8B3B4;
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __sig: _PTHREAD_MUTEX_SIG_init,
    __opaque: [0; __PTHREAD_MUTEX_SIZE__],
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __sig: _PTHREAD_COND_SIG_init,
    __opaque: [0; __PTHREAD_COND_SIZE__],
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __sig: _PTHREAD_RWLOCK_SIG_init,
    __opaque: [0; __PTHREAD_RWLOCK_SIZE__],
};

pub const SIGSTKSZ: ::size_t = 131072;

pub const FD_SETSIZE: usize = 1024;

pub const ST_NOSUID: ::c_ulong = 2;

pub const HW_AVAILCPU: ::c_int = 25;

pub const EVFILT_AIO: ::int16_t = 0xfffd;
pub const EVFILT_PROC: ::int16_t = 0xfffb;
pub const EVFILT_READ: ::int16_t = 0xffff;
pub const EVFILT_SIGNAL: ::int16_t = 0xfffa;
pub const EVFILT_SYSCOUNT: ::int16_t = 0xe;
pub const EVFILT_TIMER: ::int16_t = 0xfff9;
pub const EVFILT_VNODE: ::int16_t = 0xfffc;
pub const EVFILT_WRITE: ::int16_t = 0xfffe;
pub const EVFILT_FS: ::int16_t = 0xfff7;
pub const EVFILT_MACHPORT: ::int16_t = 0xfff8;
pub const EVFILT_USER: ::int16_t = 0xfff6;
pub const EVFILT_VM: ::int16_t = 0xfff4;

pub const EV_DISPATCH: ::uint16_t = 0x80;
pub const EV_FLAG0: ::uint16_t = 0x1000;
pub const EV_OOBAND: ::uint16_t = 0x2000;
pub const EV_POLL: ::uint16_t = 0x1000;
pub const EV_RECEIPT: ::uint16_t = 0x40;

pub const NOTE_ABSOLUTE: ::uint32_t = 0x8;
pub const NOTE_EXITSTATUS: ::uint32_t = 0x04000000;
pub const NOTE_EXIT_REPARENTED: ::uint32_t = 0x00080000;
pub const NOTE_FFAND: ::uint32_t = 0x40000000;
pub const NOTE_FFCOPY: ::uint32_t = 0xc0000000;
pub const NOTE_FFCTRLMASK: ::uint32_t = 0xc0000000;
pub const NOTE_FFLAGSMASK: ::uint32_t = 0x00ffffff;
pub const NOTE_FFNOP: ::uint32_t = 0x0;
pub const NOTE_FFOR: ::uint32_t = 0x80000000;
pub const NOTE_NONE: ::uint32_t = 0x80;
pub const NOTE_NSECONDS: ::uint32_t = 0x4;
pub const NOTE_REAP: ::uint32_t = 0x10000000;
pub const NOTE_SECONDS: ::uint32_t = 0x1;
pub const NOTE_SIGNAL: ::uint32_t = 0x8000000;
pub const NOTE_TRIGGER: ::uint32_t = 0x01000000;
pub const NOTE_USECONDS: ::uint32_t = 0x2;
pub const NOTE_VM_ERROR: ::uint32_t = 0x10000000;
pub const NOTE_VM_PRESSURE: ::uint32_t = 0x80000000;
pub const NOTE_VM_PRESSURE_SUDDEN_TERMINATE: ::uint32_t = 0x20000000;
pub const NOTE_VM_PRESSURE_TERMINATE: ::uint32_t = 0x40000000;
pub const NOTE_PCTRLMASK: ::uint32_t = 0xfff00000;

pub const NL0: ::c_int  = 0x00000000;
pub const NL1: ::c_int  = 0x00000100;
pub const TAB0: ::c_int = 0x00000000;
pub const TAB1: ::c_int = 0x00000400;
pub const TAB2: ::c_int = 0x00000800;
pub const CR0: ::c_int  = 0x00000000;
pub const CR1: ::c_int  = 0x00001000;
pub const CR2: ::c_int  = 0x00002000;
pub const CR3: ::c_int  = 0x00003000;
pub const FF0: ::c_int  = 0x00000000;
pub const FF1: ::c_int  = 0x00004000;
pub const BS0: ::c_int  = 0x00000000;
pub const BS1: ::c_int  = 0x00008000;
pub const TAB3: ::c_int = 0x00000004;
pub const VT0: ::c_int  = 0x00000000;
pub const VT1: ::c_int  = 0x00010000;
pub const IUTF8: ::tcflag_t = 0x00004000;
pub const CRTSCTS: ::tcflag_t = 0x00030000;

pub const NI_MAXHOST: ::socklen_t = 1025;

pub const Q_GETQUOTA: ::c_int = 0x300;
pub const Q_SETQUOTA: ::c_int = 0x400;

pub const RTLD_LOCAL: ::c_int = 0x4;
pub const RTLD_FIRST: ::c_int = 0x100;
pub const RTLD_NODELETE: ::c_int = 0x80;
pub const RTLD_NOLOAD: ::c_int = 0x10;
pub const RTLD_GLOBAL: ::c_int = 0x8;

extern {
    pub fn getnameinfo(sa: *const ::sockaddr,
                       salen: ::socklen_t,
                       host: *mut ::c_char,
                       hostlen: ::socklen_t,
                       serv: *mut ::c_char,
                       sevlen: ::socklen_t,
                       flags: ::c_int) -> ::c_int;
    pub fn mincore(addr: *const ::c_void, len: ::size_t,
                   vec: *mut ::c_char) -> ::c_int;
    pub fn sysctlnametomib(name: *const ::c_char,
                           mibp: *mut ::c_int,
                           sizep: *mut ::size_t)
                           -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "mprotect$UNIX2003")]
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn shm_open(name: *const ::c_char, oflag: ::c_int, ...) -> ::c_int;
    pub fn sysctl(name: *mut ::c_int,
                  namelen: ::c_uint,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *mut ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
    pub fn sysctlbyname(name: *const ::c_char,
                        oldp: *mut ::c_void,
                        oldlenp: *mut ::size_t,
                        newp: *mut ::c_void,
                        newlen: ::size_t)
                        -> ::c_int;
    pub fn mach_absolute_time() -> u64;
    pub fn mach_timebase_info(info: *mut ::mach_timebase_info) -> ::c_int;
    pub fn pthread_setname_np(name: *const ::c_char) -> ::c_int;
    pub fn pthread_get_stackaddr_np(thread: ::pthread_t) -> *mut ::c_void;
    pub fn pthread_get_stacksize_np(thread: ::pthread_t) -> ::size_t;
    pub fn __error() -> *mut ::c_int;
    pub fn backtrace(buf: *mut *mut ::c_void,
                     sz: ::c_int) -> ::c_int;
    #[cfg_attr(target_os = "macos", link_name = "statfs$INODE64")]
    pub fn statfs(path: *const ::c_char, buf: *mut statfs) -> ::c_int;
    #[cfg_attr(target_os = "macos", link_name = "fstatfs$INODE64")]
    pub fn fstatfs(fd: ::c_int, buf: *mut statfs) -> ::c_int;
    pub fn kevent(kq: ::c_int,
                  changelist: *const ::kevent,
                  nchanges: ::c_int,
                  eventlist: *mut ::kevent,
                  nevents: ::c_int,
                  timeout: *const ::timespec) -> ::c_int;
    pub fn kevent64(kq: ::c_int,
                    changelist: *const ::kevent64_s,
                    nchanges: ::c_int,
                    eventlist: *mut ::kevent64_s,
                    nevents: ::c_int,
                    flags: ::c_uint,
                    timeout: *const ::timespec) -> ::c_int;
    pub fn mount(src: *const ::c_char,
                 target: *const ::c_char,
                 flags: ::c_int,
                 data: *mut ::c_void) -> ::c_int;
    pub fn ptrace(requeset: ::c_int,
                  pid: ::pid_t,
                  addr: *mut ::c_char,
                  data: ::c_int) -> ::c_int;
    pub fn quotactl(special: *const ::c_char,
                    cmd: ::c_int,
                    id: ::c_int,
                    data: *mut ::c_char) -> ::c_int;
    pub fn sethostname(name: *const ::c_char, len: ::c_int) -> ::c_int;
    pub fn sendfile(fd: ::c_int,
                    s: ::c_int,
                    offset: ::off_t,
                    len: *mut ::off_t,
                    hdtr: *mut ::sf_hdtr,
                    flags: ::c_int) -> ::c_int;
}

cfg_if! {
    if #[cfg(any(target_arch = "arm", target_arch = "x86"))] {
        mod b32;
        pub use self::b32::*;
    } else if #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] {
        mod b64;
        pub use self::b64::*;
    } else {
        // unknown arch...
    }
}
