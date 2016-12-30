pub type fsblkcnt_t = ::c_ulong;
pub type fsfilcnt_t = ::c_ulong;
pub type rlim_t = c_ulong;
pub type __priority_which_t = ::c_uint;

s! {
    pub struct aiocb {
        pub aio_fildes: ::c_int,
        pub aio_lio_opcode: ::c_int,
        pub aio_reqprio: ::c_int,
        pub aio_buf: *mut ::c_void,
        pub aio_nbytes: ::size_t,
        pub aio_sigevent: ::sigevent,
        __next_prio: *mut aiocb,
        __abs_prio: ::c_int,
        __policy: ::c_int,
        __error_code: ::c_int,
        __return_value: ::ssize_t,
        pub aio_offset: off_t,
        #[cfg(target_pointer_width = "32")]
        __unused1: [::c_char; 4],
        __glibc_reserved: [::c_char; 32]
    }

    pub struct __exit_status {
        pub e_termination: ::c_short,
        pub e_exit: ::c_short,
    }

    pub struct __timeval {
        pub tv_sec: ::int32_t,
        pub tv_usec: ::int32_t,
    }

    pub struct utmpx {
        pub ut_type: ::c_short,
        pub ut_pid: ::pid_t,
        pub ut_line: [::c_char; __UT_LINESIZE],
        pub ut_id: [::c_char; 4],

        pub ut_user: [::c_char; __UT_NAMESIZE],
        pub ut_host: [::c_char; __UT_HOSTSIZE],
        pub ut_exit: __exit_status,

        #[cfg(any(target_arch = "aarch64", target_arch = "sparc64", target_pointer_width = "32"))]
        pub ut_session: ::c_long,
        #[cfg(any(target_arch = "aarch64", target_arch = "sparc64", target_pointer_width = "32"))]
        pub ut_tv: ::timeval,

        #[cfg(not(any(target_arch = "aarch64", target_arch = "sparc64", target_pointer_width = "32")))]
        pub ut_session: ::int32_t,
        #[cfg(not(any(target_arch = "aarch64", target_arch = "sparc64", target_pointer_width = "32")))]
        pub ut_tv: __timeval,

        pub ut_addr_v6: [::int32_t; 4],
        __glibc_reserved: [::c_char; 20],
    }

    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: ::sigset_t,
        #[cfg(target_arch = "sparc64")]
        __reserved0: ::c_int,
        pub sa_flags: ::c_int,
        _restorer: *mut ::c_void,
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
        _align: [usize; 0],
    }

    pub struct glob64_t {
        pub gl_pathc: ::size_t,
        pub gl_pathv: *mut *mut ::c_char,
        pub gl_offs: ::size_t,
        pub gl_flags: ::c_int,

        __unused1: *mut ::c_void,
        __unused2: *mut ::c_void,
        __unused3: *mut ::c_void,
        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
    }

    pub struct ucred {
        pub pid: ::pid_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
    }

    pub struct statfs {
        pub f_type: __fsword_t,
        pub f_bsize: __fsword_t,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,

        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_fsid: ::fsid_t,

        pub f_namelen: __fsword_t,
        pub f_frsize: __fsword_t,
        f_spare: [__fsword_t; 5],
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::size_t,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::size_t,
        pub msg_flags: ::c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: ::size_t,
        pub cmsg_level: ::c_int,
        pub cmsg_type: ::c_int,
    }

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; ::NCCS],
        #[cfg(not(target_arch = "sparc64"))]
        pub c_ispeed: ::speed_t,
        #[cfg(not(target_arch = "sparc64"))]
        pub c_ospeed: ::speed_t,
    }

    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_pid: ::pid_t,
    }

    // FIXME this is actually a union
    pub struct sem_t {
        #[cfg(target_pointer_width = "32")]
        __size: [::c_char; 16],
        #[cfg(target_pointer_width = "64")]
        __size: [::c_char; 32],
        __align: [::c_long; 0],
    }
}

pub const __UT_LINESIZE: usize = 32;
pub const __UT_NAMESIZE: usize = 32;
pub const __UT_HOSTSIZE: usize = 256;
pub const EMPTY: ::c_short = 0;
pub const RUN_LVL: ::c_short = 1;
pub const BOOT_TIME: ::c_short = 2;
pub const NEW_TIME: ::c_short = 3;
pub const OLD_TIME: ::c_short = 4;
pub const INIT_PROCESS: ::c_short = 5;
pub const LOGIN_PROCESS: ::c_short = 6;
pub const USER_PROCESS: ::c_short = 7;
pub const DEAD_PROCESS: ::c_short = 8;
pub const ACCOUNTING: ::c_short = 9;

pub const RLIMIT_RSS: ::c_int = 5;
#[cfg(not(target_arch = "sparc64"))]
pub const RLIMIT_NOFILE: ::c_int = 7;
#[cfg(target_arch = "sparc64")]
pub const RLIMIT_NOFILE: ::c_int = 6;
pub const RLIMIT_AS: ::c_int = 9;
#[cfg(not(target_arch = "sparc64"))]
pub const RLIMIT_NPROC: ::c_int = 6;
#[cfg(target_arch = "sparc64")]
pub const RLIMIT_NPROC: ::c_int = 7;
pub const RLIMIT_MEMLOCK: ::c_int = 8;
pub const RLIM_INFINITY: ::rlim_t = !0;
pub const RLIMIT_RTTIME: ::c_int = 15;
pub const RLIMIT_NLIMITS: ::c_int = 16;

#[cfg(not(target_arch = "sparc64"))]
pub const O_APPEND: ::c_int = 1024;
#[cfg(target_arch = "sparc64")]
pub const O_APPEND: ::c_int = 0x8;
#[cfg(not(target_arch = "sparc64"))]
pub const O_CREAT: ::c_int = 64;
#[cfg(target_arch = "sparc64")]
pub const O_CREAT: ::c_int = 0x200;
#[cfg(not(target_arch = "sparc64"))]
pub const O_EXCL: ::c_int = 128;
#[cfg(target_arch = "sparc64")]
pub const O_EXCL: ::c_int = 0x800;
#[cfg(not(target_arch = "sparc64"))]
pub const O_NOCTTY: ::c_int = 256;
#[cfg(target_arch = "sparc64")]
pub const O_NOCTTY: ::c_int = 0x8000;
#[cfg(not(target_arch = "sparc64"))]
pub const O_NONBLOCK: ::c_int = 2048;
#[cfg(target_arch = "sparc64")]
pub const O_NONBLOCK: ::c_int = 0x4000;
#[cfg(not(target_arch = "sparc64"))]
pub const O_SYNC: ::c_int = 1052672;
#[cfg(target_arch = "sparc64")]
pub const O_SYNC: ::c_int = 0x802000;
#[cfg(not(target_arch = "sparc64"))]
pub const O_RSYNC: ::c_int = 1052672;
#[cfg(target_arch = "sparc64")]
pub const O_RSYNC: ::c_int = 0x802000;
#[cfg(not(target_arch = "sparc64"))]
pub const O_DSYNC: ::c_int = 4096;
#[cfg(target_arch = "sparc64")]
pub const O_DSYNC: ::c_int = 0x2000;
#[cfg(not(target_arch = "sparc64"))]
pub const O_FSYNC: ::c_int = 0x101000;
#[cfg(target_arch = "sparc64")]
pub const O_FSYNC: ::c_int = 0x802000;

pub const SOCK_NONBLOCK: ::c_int = O_NONBLOCK;

pub const LC_PAPER: ::c_int = 7;
pub const LC_NAME: ::c_int = 8;
pub const LC_ADDRESS: ::c_int = 9;
pub const LC_TELEPHONE: ::c_int = 10;
pub const LC_MEASUREMENT: ::c_int = 11;
pub const LC_IDENTIFICATION: ::c_int = 12;
pub const LC_PAPER_MASK: ::c_int = (1 << LC_PAPER);
pub const LC_NAME_MASK: ::c_int = (1 << LC_NAME);
pub const LC_ADDRESS_MASK: ::c_int = (1 << LC_ADDRESS);
pub const LC_TELEPHONE_MASK: ::c_int = (1 << LC_TELEPHONE);
pub const LC_MEASUREMENT_MASK: ::c_int = (1 << LC_MEASUREMENT);
pub const LC_IDENTIFICATION_MASK: ::c_int = (1 << LC_IDENTIFICATION);
pub const LC_ALL_MASK: ::c_int = ::LC_CTYPE_MASK
                               | ::LC_NUMERIC_MASK
                               | ::LC_TIME_MASK
                               | ::LC_COLLATE_MASK
                               | ::LC_MONETARY_MASK
                               | ::LC_MESSAGES_MASK
                               | LC_PAPER_MASK
                               | LC_NAME_MASK
                               | LC_ADDRESS_MASK
                               | LC_TELEPHONE_MASK
                               | LC_MEASUREMENT_MASK
                               | LC_IDENTIFICATION_MASK;

pub const MAP_ANON: ::c_int = 0x0020;
pub const MAP_ANONYMOUS: ::c_int = 0x0020;
#[cfg(not(target_arch = "sparc64"))]
pub const MAP_GROWSDOWN: ::c_int = 0x0100;
#[cfg(target_arch = "sparc64")]
pub const MAP_GROWSDOWN: ::c_int = 0x0200;
pub const MAP_DENYWRITE: ::c_int = 0x0800;
pub const MAP_EXECUTABLE: ::c_int = 0x01000;
pub const MAP_POPULATE: ::c_int = 0x08000;
pub const MAP_NONBLOCK: ::c_int = 0x010000;
pub const MAP_STACK: ::c_int = 0x020000;

#[cfg(not(target_arch = "sparc64"))]
pub const EDEADLK: ::c_int = 35;
#[cfg(target_arch = "sparc64")]
pub const EDEADLK: ::c_int = 78;
#[cfg(not(target_arch = "sparc64"))]
pub const ENAMETOOLONG: ::c_int = 36;
#[cfg(target_arch = "sparc64")]
pub const ENAMETOOLONG: ::c_int = 63;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOLCK: ::c_int = 37;
#[cfg(target_arch = "sparc64")]
pub const ENOLCK: ::c_int = 79;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOSYS: ::c_int = 38;
#[cfg(target_arch = "sparc64")]
pub const ENOSYS: ::c_int = 90;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOTEMPTY: ::c_int = 39;
#[cfg(target_arch = "sparc64")]
pub const ENOTEMPTY: ::c_int = 66;
#[cfg(not(target_arch = "sparc64"))]
pub const ELOOP: ::c_int = 40;
#[cfg(target_arch = "sparc64")]
pub const ELOOP: ::c_int = 62;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOMSG: ::c_int = 42;
#[cfg(target_arch = "sparc64")]
pub const ENOMSG: ::c_int = 75;
#[cfg(not(target_arch = "sparc64"))]
pub const EIDRM: ::c_int = 43;
#[cfg(target_arch = "sparc64")]
pub const EIDRM: ::c_int = 77;
#[cfg(not(target_arch = "sparc64"))]
pub const ECHRNG: ::c_int = 44;
#[cfg(target_arch = "sparc64")]
pub const ECHRNG: ::c_int = 94;
#[cfg(not(target_arch = "sparc64"))]
pub const EL2NSYNC: ::c_int = 45;
#[cfg(target_arch = "sparc64")]
pub const EL2NSYNC: ::c_int = 95;
#[cfg(not(target_arch = "sparc64"))]
pub const EL3HLT: ::c_int = 46;
#[cfg(target_arch = "sparc64")]
pub const EL3HLT: ::c_int = 96;
#[cfg(not(target_arch = "sparc64"))]
pub const EL3RST: ::c_int = 47;
#[cfg(target_arch = "sparc64")]
pub const EL3RST: ::c_int = 97;
#[cfg(not(target_arch = "sparc64"))]
pub const ELNRNG: ::c_int = 48;
#[cfg(target_arch = "sparc64")]
pub const ELNRNG: ::c_int = 98;
#[cfg(not(target_arch = "sparc64"))]
pub const EUNATCH: ::c_int = 49;
#[cfg(target_arch = "sparc64")]
pub const EUNATCH: ::c_int = 99;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOCSI: ::c_int = 50;
#[cfg(target_arch = "sparc64")]
pub const ENOCSI: ::c_int = 100;
#[cfg(not(target_arch = "sparc64"))]
pub const EL2HLT: ::c_int = 51;
#[cfg(target_arch = "sparc64")]
pub const EL2HLT: ::c_int = 101;
#[cfg(not(target_arch = "sparc64"))]
pub const EBADE: ::c_int = 52;
#[cfg(target_arch = "sparc64")]
pub const EBADE: ::c_int = 102;
#[cfg(not(target_arch = "sparc64"))]
pub const EBADR: ::c_int = 53;
#[cfg(target_arch = "sparc64")]
pub const EBADR: ::c_int = 103;
#[cfg(not(target_arch = "sparc64"))]
pub const EXFULL: ::c_int = 54;
#[cfg(target_arch = "sparc64")]
pub const EXFULL: ::c_int = 104;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOANO: ::c_int = 55;
#[cfg(target_arch = "sparc64")]
pub const ENOANO: ::c_int = 105;
#[cfg(not(target_arch = "sparc64"))]
pub const EBADRQC: ::c_int = 56;
#[cfg(target_arch = "sparc64")]
pub const EBADRQC: ::c_int = 106;
#[cfg(not(target_arch = "sparc64"))]
pub const EBADSLT: ::c_int = 57;
#[cfg(target_arch = "sparc64")]
pub const EBADSLT: ::c_int = 107;
#[cfg(not(target_arch = "sparc64"))]
pub const EMULTIHOP: ::c_int = 72;
#[cfg(target_arch = "sparc64")]
pub const EMULTIHOP: ::c_int = 87;
#[cfg(not(target_arch = "sparc64"))]
pub const EOVERFLOW: ::c_int = 75;
#[cfg(target_arch = "sparc64")]
pub const EOVERFLOW: ::c_int = 92;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOTUNIQ: ::c_int = 76;
#[cfg(target_arch = "sparc64")]
pub const ENOTUNIQ: ::c_int = 115;
#[cfg(not(target_arch = "sparc64"))]
pub const EBADFD: ::c_int = 77;
#[cfg(target_arch = "sparc64")]
pub const EBADFD: ::c_int = 93;
#[cfg(not(target_arch = "sparc64"))]
pub const EBADMSG: ::c_int = 74;
#[cfg(target_arch = "sparc64")]
pub const EBADMSG: ::c_int = 76;
#[cfg(not(target_arch = "sparc64"))]
pub const EREMCHG: ::c_int = 78;
#[cfg(target_arch = "sparc64")]
pub const EREMCHG: ::c_int = 89;
#[cfg(not(target_arch = "sparc64"))]
pub const ELIBACC: ::c_int = 79;
#[cfg(target_arch = "sparc64")]
pub const ELIBACC: ::c_int = 114;
#[cfg(not(target_arch = "sparc64"))]
pub const ELIBBAD: ::c_int = 80;
#[cfg(target_arch = "sparc64")]
pub const ELIBBAD: ::c_int = 112;
#[cfg(not(target_arch = "sparc64"))]
pub const ELIBSCN: ::c_int = 81;
#[cfg(target_arch = "sparc64")]
pub const ELIBSCN: ::c_int = 124;
#[cfg(not(target_arch = "sparc64"))]
pub const ELIBMAX: ::c_int = 82;
#[cfg(target_arch = "sparc64")]
pub const ELIBMAX: ::c_int = 123;
#[cfg(not(target_arch = "sparc64"))]
pub const ELIBEXEC: ::c_int = 83;
#[cfg(target_arch = "sparc64")]
pub const ELIBEXEC: ::c_int = 110;
#[cfg(not(target_arch = "sparc64"))]
pub const EILSEQ: ::c_int = 84;
#[cfg(target_arch = "sparc64")]
pub const EILSEQ: ::c_int = 122;
#[cfg(not(target_arch = "sparc64"))]
pub const ERESTART: ::c_int = 85;
#[cfg(target_arch = "sparc64")]
pub const ERESTART: ::c_int = 116;
#[cfg(not(target_arch = "sparc64"))]
pub const ESTRPIPE: ::c_int = 86;
#[cfg(target_arch = "sparc64")]
pub const ESTRPIPE: ::c_int = 91;
#[cfg(not(target_arch = "sparc64"))]
pub const EUSERS: ::c_int = 87;
#[cfg(target_arch = "sparc64")]
pub const EUSERS: ::c_int = 68;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOTSOCK: ::c_int = 88;
#[cfg(target_arch = "sparc64")]
pub const ENOTSOCK: ::c_int = 38;
#[cfg(not(target_arch = "sparc64"))]
pub const EDESTADDRREQ: ::c_int = 89;
#[cfg(target_arch = "sparc64")]
pub const EDESTADDRREQ: ::c_int = 39;
#[cfg(not(target_arch = "sparc64"))]
pub const EMSGSIZE: ::c_int = 90;
#[cfg(target_arch = "sparc64")]
pub const EMSGSIZE: ::c_int = 40;
#[cfg(not(target_arch = "sparc64"))]
pub const EPROTOTYPE: ::c_int = 91;
#[cfg(target_arch = "sparc64")]
pub const EPROTOTYPE: ::c_int = 41;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOPROTOOPT: ::c_int = 92;
#[cfg(target_arch = "sparc64")]
pub const ENOPROTOOPT: ::c_int = 42;
#[cfg(not(target_arch = "sparc64"))]
pub const EPROTONOSUPPORT: ::c_int = 93;
#[cfg(target_arch = "sparc64")]
pub const EPROTONOSUPPORT: ::c_int = 43;
#[cfg(not(target_arch = "sparc64"))]
pub const ESOCKTNOSUPPORT: ::c_int = 94;
#[cfg(target_arch = "sparc64")]
pub const ESOCKTNOSUPPORT: ::c_int = 44;
#[cfg(not(target_arch = "sparc64"))]
pub const EOPNOTSUPP: ::c_int = 95;
#[cfg(target_arch = "sparc64")]
pub const EOPNOTSUPP: ::c_int = 45;
pub const ENOTSUP: ::c_int = EOPNOTSUPP;
#[cfg(not(target_arch = "sparc64"))]
pub const EPFNOSUPPORT: ::c_int = 96;
#[cfg(target_arch = "sparc64")]
pub const EPFNOSUPPORT: ::c_int = 46;
#[cfg(not(target_arch = "sparc64"))]
pub const EAFNOSUPPORT: ::c_int = 97;
#[cfg(target_arch = "sparc64")]
pub const EAFNOSUPPORT: ::c_int = 47;
#[cfg(not(target_arch = "sparc64"))]
pub const EADDRINUSE: ::c_int = 98;
#[cfg(target_arch = "sparc64")]
pub const EADDRINUSE: ::c_int = 48;
#[cfg(not(target_arch = "sparc64"))]
pub const EADDRNOTAVAIL: ::c_int = 99;
#[cfg(target_arch = "sparc64")]
pub const EADDRNOTAVAIL: ::c_int = 49;
#[cfg(not(target_arch = "sparc64"))]
pub const ENETDOWN: ::c_int = 100;
#[cfg(target_arch = "sparc64")]
pub const ENETDOWN: ::c_int = 50;
#[cfg(not(target_arch = "sparc64"))]
pub const ENETUNREACH: ::c_int = 101;
#[cfg(target_arch = "sparc64")]
pub const ENETUNREACH: ::c_int = 51;
#[cfg(not(target_arch = "sparc64"))]
pub const ENETRESET: ::c_int = 102;
#[cfg(target_arch = "sparc64")]
pub const ENETRESET: ::c_int = 52;
#[cfg(not(target_arch = "sparc64"))]
pub const ECONNABORTED: ::c_int = 103;
#[cfg(target_arch = "sparc64")]
pub const ECONNABORTED: ::c_int = 53;
#[cfg(not(target_arch = "sparc64"))]
pub const ECONNRESET: ::c_int = 104;
#[cfg(target_arch = "sparc64")]
pub const ECONNRESET: ::c_int = 54;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOBUFS: ::c_int = 105;
#[cfg(target_arch = "sparc64")]
pub const ENOBUFS: ::c_int = 55;
#[cfg(not(target_arch = "sparc64"))]
pub const EISCONN: ::c_int = 106;
#[cfg(target_arch = "sparc64")]
pub const EISCONN: ::c_int = 56;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOTCONN: ::c_int = 107;
#[cfg(target_arch = "sparc64")]
pub const ENOTCONN: ::c_int = 57;
#[cfg(not(target_arch = "sparc64"))]
pub const ESHUTDOWN: ::c_int = 108;
#[cfg(target_arch = "sparc64")]
pub const ESHUTDOWN: ::c_int = 58;
#[cfg(not(target_arch = "sparc64"))]
pub const ETOOMANYREFS: ::c_int = 109;
#[cfg(target_arch = "sparc64")]
pub const ETOOMANYREFS: ::c_int = 59;
#[cfg(not(target_arch = "sparc64"))]
pub const ETIMEDOUT: ::c_int = 110;
#[cfg(target_arch = "sparc64")]
pub const ETIMEDOUT: ::c_int = 60;
#[cfg(not(target_arch = "sparc64"))]
pub const ECONNREFUSED: ::c_int = 111;
#[cfg(target_arch = "sparc64")]
pub const ECONNREFUSED: ::c_int = 61;
#[cfg(not(target_arch = "sparc64"))]
pub const EHOSTDOWN: ::c_int = 112;
#[cfg(target_arch = "sparc64")]
pub const EHOSTDOWN: ::c_int = 64;
#[cfg(not(target_arch = "sparc64"))]
pub const EHOSTUNREACH: ::c_int = 113;
#[cfg(target_arch = "sparc64")]
pub const EHOSTUNREACH: ::c_int = 65;
#[cfg(not(target_arch = "sparc64"))]
pub const EALREADY: ::c_int = 114;
#[cfg(target_arch = "sparc64")]
pub const EALREADY: ::c_int = 37;
#[cfg(not(target_arch = "sparc64"))]
pub const EINPROGRESS: ::c_int = 115;
#[cfg(target_arch = "sparc64")]
pub const EINPROGRESS: ::c_int = 36;
#[cfg(not(target_arch = "sparc64"))]
pub const ESTALE: ::c_int = 116;
#[cfg(target_arch = "sparc64")]
pub const ESTALE: ::c_int = 70;
pub const EUCLEAN: ::c_int = 117;
pub const ENOTNAM: ::c_int = 118;
pub const ENAVAIL: ::c_int = 119;
pub const EISNAM: ::c_int = 120;
pub const EREMOTEIO: ::c_int = 121;
#[cfg(not(target_arch = "sparc64"))]
pub const EDQUOT: ::c_int = 122;
#[cfg(target_arch = "sparc64")]
pub const EDQUOT: ::c_int = 69;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOMEDIUM: ::c_int = 123;
#[cfg(target_arch = "sparc64")]
pub const ENOMEDIUM: ::c_int = 125;
#[cfg(not(target_arch = "sparc64"))]
pub const EMEDIUMTYPE: ::c_int = 124;
#[cfg(target_arch = "sparc64")]
pub const EMEDIUMTYPE: ::c_int = 126;
#[cfg(not(target_arch = "sparc64"))]
pub const ECANCELED: ::c_int = 125;
#[cfg(target_arch = "sparc64")]
pub const ECANCELED: ::c_int = 127;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOKEY: ::c_int = 126;
#[cfg(target_arch = "sparc64")]
pub const ENOKEY: ::c_int = 128;
#[cfg(not(target_arch = "sparc64"))]
pub const EKEYEXPIRED: ::c_int = 127;
#[cfg(target_arch = "sparc64")]
pub const EKEYEXPIRED: ::c_int = 129;
#[cfg(not(target_arch = "sparc64"))]
pub const EKEYREVOKED: ::c_int = 128;
#[cfg(target_arch = "sparc64")]
pub const EKEYREVOKED: ::c_int = 130;
#[cfg(not(target_arch = "sparc64"))]
pub const EKEYREJECTED: ::c_int = 129;
#[cfg(target_arch = "sparc64")]
pub const EKEYREJECTED: ::c_int = 131;
#[cfg(not(target_arch = "sparc64"))]
pub const EOWNERDEAD: ::c_int = 130;
#[cfg(target_arch = "sparc64")]
pub const EOWNERDEAD: ::c_int = 132;
#[cfg(not(target_arch = "sparc64"))]
pub const ENOTRECOVERABLE: ::c_int = 131;
#[cfg(target_arch = "sparc64")]
pub const ENOTRECOVERABLE: ::c_int = 133;
#[cfg(not(target_arch = "sparc64"))]
pub const EHWPOISON: ::c_int = 133;
#[cfg(target_arch = "sparc64")]
pub const EHWPOISON: ::c_int = 135;
#[cfg(not(target_arch = "sparc64"))]
pub const ERFKILL: ::c_int = 132;
#[cfg(target_arch = "sparc64")]
pub const ERFKILL: ::c_int = 134;

pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_SEQPACKET: ::c_int = 5;

#[cfg(not(target_arch = "sparc64"))]
pub const SOL_SOCKET: ::c_int = 1;
#[cfg(target_arch = "sparc64")]
pub const SOL_SOCKET: ::c_int = 0xffff;

#[cfg(not(target_arch = "sparc64"))]
pub const SO_REUSEADDR: ::c_int = 2;
#[cfg(target_arch = "sparc64")]
pub const SO_REUSEADDR: ::c_int = 4;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_TYPE: ::c_int = 3;
#[cfg(target_arch = "sparc64")]
pub const SO_TYPE: ::c_int = 0x1008;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_ERROR: ::c_int = 4;
#[cfg(target_arch = "sparc64")]
pub const SO_ERROR: ::c_int = 0x1007;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_DONTROUTE: ::c_int = 5;
#[cfg(target_arch = "sparc64")]
pub const SO_DONTROUTE: ::c_int = 16;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_BROADCAST: ::c_int = 6;
#[cfg(target_arch = "sparc64")]
pub const SO_BROADCAST: ::c_int = 32;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_SNDBUF: ::c_int = 7;
#[cfg(target_arch = "sparc64")]
pub const SO_SNDBUF: ::c_int = 0x1001;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_RCVBUF: ::c_int = 8;
#[cfg(target_arch = "sparc64")]
pub const SO_RCVBUF: ::c_int = 0x1002;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_KEEPALIVE: ::c_int = 9;
#[cfg(target_arch = "sparc64")]
pub const SO_KEEPALIVE: ::c_int = 8;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_OOBINLINE: ::c_int = 10;
#[cfg(target_arch = "sparc64")]
pub const SO_OOBINLINE: ::c_int = 0x100;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_LINGER: ::c_int = 13;
#[cfg(target_arch = "sparc64")]
pub const SO_LINGER: ::c_int = 128;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_REUSEPORT: ::c_int = 15;
#[cfg(target_arch = "sparc64")]
pub const SO_REUSEPORT: ::c_int = 0x200;
#[cfg(not(target_arch = "sparc64"))]
pub const SO_ACCEPTCONN: ::c_int = 30;
#[cfg(target_arch = "sparc64")]
pub const SO_ACCEPTCONN: ::c_int = 0x8000;

pub const TCP_COOKIE_TRANSACTIONS: ::c_int = 15;
pub const TCP_THIN_LINEAR_TIMEOUTS: ::c_int = 16;
pub const TCP_THIN_DUPACK: ::c_int = 17;
pub const TCP_USER_TIMEOUT: ::c_int = 18;
pub const TCP_REPAIR: ::c_int = 19;
pub const TCP_REPAIR_QUEUE: ::c_int = 20;
pub const TCP_QUEUE_SEQ: ::c_int = 21;
pub const TCP_REPAIR_OPTIONS: ::c_int = 22;
pub const TCP_FASTOPEN: ::c_int = 23;
pub const TCP_TIMESTAMP: ::c_int = 24;

#[cfg(not(target_arch = "sparc64"))]
pub const SA_ONSTACK: ::c_int = 0x08000000;
#[cfg(target_arch = "sparc64")]
pub const SA_ONSTACK: ::c_int = 1;
#[cfg(not(target_arch = "sparc64"))]
pub const SA_SIGINFO: ::c_int = 0x00000004;
#[cfg(target_arch = "sparc64")]
pub const SA_SIGINFO: ::c_int = 0x200;
#[cfg(not(target_arch = "sparc64"))]
pub const SA_NOCLDWAIT: ::c_int = 0x00000002;
#[cfg(target_arch = "sparc64")]
pub const SA_NOCLDWAIT: ::c_int = 0x100;

#[cfg(not(target_arch = "sparc64"))]
pub const SIGCHLD: ::c_int = 17;
#[cfg(target_arch = "sparc64")]
pub const SIGCHLD: ::c_int = 20;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGBUS: ::c_int = 7;
#[cfg(target_arch = "sparc64")]
pub const SIGBUS: ::c_int = 10;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGUSR1: ::c_int = 10;
#[cfg(target_arch = "sparc64")]
pub const SIGUSR1: ::c_int = 30;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGUSR2: ::c_int = 12;
#[cfg(target_arch = "sparc64")]
pub const SIGUSR2: ::c_int = 31;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGCONT: ::c_int = 18;
#[cfg(target_arch = "sparc64")]
pub const SIGCONT: ::c_int = 19;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGSTOP: ::c_int = 19;
#[cfg(target_arch = "sparc64")]
pub const SIGSTOP: ::c_int = 17;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGTSTP: ::c_int = 20;
#[cfg(target_arch = "sparc64")]
pub const SIGTSTP: ::c_int = 18;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGURG: ::c_int = 23;
#[cfg(target_arch = "sparc64")]
pub const SIGURG: ::c_int = 16;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGIO: ::c_int = 29;
#[cfg(target_arch = "sparc64")]
pub const SIGIO: ::c_int = 23;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGSYS: ::c_int = 31;
#[cfg(target_arch = "sparc64")]
pub const SIGSYS: ::c_int = 12;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGSTKFLT: ::c_int = 16;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGUNUSED: ::c_int = 31;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF: ::c_int = 27;
pub const SIGWINCH: ::c_int = 28;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGPOLL: ::c_int = 29;
#[cfg(target_arch = "sparc64")]
pub const SIGPOLL: ::c_int = 23;
#[cfg(not(target_arch = "sparc64"))]
pub const SIGPWR: ::c_int = 30;
#[cfg(target_arch = "sparc64")]
pub const SIGPWR: ::c_int = 29;
#[cfg(not(target_arch = "sparc64"))]
pub const SIG_SETMASK: ::c_int = 2;
#[cfg(target_arch = "sparc64")]
pub const SIG_SETMASK: ::c_int = 4;
#[cfg(not(target_arch = "sparc64"))]
pub const SIG_BLOCK: ::c_int = 0x000000;
#[cfg(target_arch = "sparc64")]
pub const SIG_BLOCK: ::c_int = 1;
#[cfg(not(target_arch = "sparc64"))]
pub const SIG_UNBLOCK: ::c_int = 0x01;
#[cfg(target_arch = "sparc64")]
pub const SIG_UNBLOCK: ::c_int = 2;

pub const SIGEV_THREAD_ID: ::c_int = 4;

pub const POLLRDNORM: ::c_short = 0x040;
#[cfg(not(target_arch = "sparc64"))]
pub const POLLWRNORM: ::c_short = 0x100;
#[cfg(target_arch = "sparc64")]
pub const POLLWRNORM: ::c_short = 4;
pub const POLLRDBAND: ::c_short = 0x080;
#[cfg(not(target_arch = "sparc64"))]
pub const POLLWRBAND: ::c_short = 0x200;
#[cfg(target_arch = "sparc64")]
pub const POLLWRBAND: ::c_short = 0x100;

pub const FALLOC_FL_KEEP_SIZE: ::c_int = 0x01;
pub const FALLOC_FL_PUNCH_HOLE: ::c_int = 0x02;

pub const BUFSIZ: ::c_uint = 8192;
pub const TMP_MAX: ::c_uint = 238328;
pub const FOPEN_MAX: ::c_uint = 16;
pub const POSIX_FADV_DONTNEED: ::c_int = 4;
pub const POSIX_FADV_NOREUSE: ::c_int = 5;
pub const POSIX_MADV_DONTNEED: ::c_int = 4;
pub const _SC_2_C_VERSION: ::c_int = 96;
pub const O_ACCMODE: ::c_int = 3;
#[cfg(not(target_arch = "sparc64"))]
pub const O_ASYNC: ::c_int = 0x2000;
#[cfg(target_arch = "sparc64")]
pub const O_ASYNC: ::c_int = 0x40;
#[cfg(not(target_arch = "sparc64"))]
pub const O_NDELAY: ::c_int = 0x800;
#[cfg(target_arch = "sparc64")]
pub const O_NDELAY: ::c_int = 0x4004;
pub const ST_RELATIME: ::c_ulong = 4096;
pub const NI_MAXHOST: ::socklen_t = 1025;

pub const ADFS_SUPER_MAGIC: ::c_long = 0x0000adf5;
pub const AFFS_SUPER_MAGIC: ::c_long = 0x0000adff;
pub const CODA_SUPER_MAGIC: ::c_long = 0x73757245;
pub const CRAMFS_MAGIC: ::c_long = 0x28cd3d45;
pub const EFS_SUPER_MAGIC: ::c_long = 0x00414a53;
pub const EXT2_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const EXT3_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const EXT4_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const HPFS_SUPER_MAGIC: ::c_long = 0xf995e849;
pub const HUGETLBFS_MAGIC: ::c_long = 0x958458f6;
pub const ISOFS_SUPER_MAGIC: ::c_long = 0x00009660;
pub const JFFS2_SUPER_MAGIC: ::c_long = 0x000072b6;
pub const MINIX_SUPER_MAGIC: ::c_long = 0x0000137f;
pub const MINIX_SUPER_MAGIC2: ::c_long = 0x0000138f;
pub const MINIX2_SUPER_MAGIC: ::c_long = 0x00002468;
pub const MINIX2_SUPER_MAGIC2: ::c_long = 0x00002478;
pub const MSDOS_SUPER_MAGIC: ::c_long = 0x00004d44;
pub const NCP_SUPER_MAGIC: ::c_long = 0x0000564c;
pub const NFS_SUPER_MAGIC: ::c_long = 0x00006969;
pub const OPENPROM_SUPER_MAGIC: ::c_long = 0x00009fa1;
pub const PROC_SUPER_MAGIC: ::c_long = 0x00009fa0;
pub const QNX4_SUPER_MAGIC: ::c_long = 0x0000002f;
pub const REISERFS_SUPER_MAGIC: ::c_long = 0x52654973;
pub const SMB_SUPER_MAGIC: ::c_long = 0x0000517b;
pub const TMPFS_MAGIC: ::c_long = 0x01021994;
pub const USBDEVICE_SUPER_MAGIC: ::c_long = 0x00009fa2;

pub const VEOF: usize = 4;
pub const IUTF8: ::tcflag_t = 0x00004000;

pub const CPU_SETSIZE: ::c_int = 0x400;

pub const QFMT_VFS_V1: ::c_int = 4;

pub const PTRACE_TRACEME: ::c_uint = 0;
pub const PTRACE_PEEKTEXT: ::c_uint = 1;
pub const PTRACE_PEEKDATA: ::c_uint = 2;
pub const PTRACE_PEEKUSER: ::c_uint = 3;
pub const PTRACE_POKETEXT: ::c_uint = 4;
pub const PTRACE_POKEDATA: ::c_uint = 5;
pub const PTRACE_POKEUSER: ::c_uint = 6;
pub const PTRACE_CONT: ::c_uint = 7;
pub const PTRACE_KILL: ::c_uint = 8;
pub const PTRACE_SINGLESTEP: ::c_uint = 9;
pub const PTRACE_ATTACH: ::c_uint = 16;
#[cfg(not(target_arch = "sparc64"))]
pub const PTRACE_DETACH: ::c_uint = 17;
#[cfg(target_arch = "sparc64")]
pub const PTRACE_DETACH: ::c_uint = 11;
pub const PTRACE_SYSCALL: ::c_uint = 24;
pub const PTRACE_SETOPTIONS: ::c_uint = 0x4200;
pub const PTRACE_GETEVENTMSG: ::c_uint = 0x4201;
pub const PTRACE_GETSIGINFO: ::c_uint = 0x4202;
pub const PTRACE_SETSIGINFO: ::c_uint = 0x4203;
pub const PTRACE_GETREGSET: ::c_uint = 0x4204;
pub const PTRACE_SETREGSET: ::c_uint = 0x4205;
pub const PTRACE_SEIZE: ::c_uint = 0x4206;
pub const PTRACE_INTERRUPT: ::c_uint = 0x4207;
pub const PTRACE_LISTEN: ::c_uint = 0x4208;
pub const PTRACE_PEEKSIGINFO: ::c_uint = 0x4209;

pub const MADV_DODUMP: ::c_int = 17;
pub const MADV_DONTDUMP: ::c_int = 16;

pub const EPOLLWAKEUP: ::c_int = 0x20000000;

pub const MADV_HUGEPAGE: ::c_int = 14;
pub const MADV_NOHUGEPAGE: ::c_int = 15;
pub const MAP_HUGETLB: ::c_int = 0x040000;

#[cfg(not(target_arch = "sparc64"))]
pub const EFD_NONBLOCK: ::c_int = 0x800;
#[cfg(target_arch = "sparc64")]
pub const EFD_NONBLOCK: ::c_int = 0x4000;

#[cfg(not(target_arch = "sparc64"))]
pub const F_GETLK: ::c_int = 5;
#[cfg(target_arch = "sparc64")]
pub const F_GETLK: ::c_int = 7;
#[cfg(not(target_arch = "sparc64"))]
pub const F_GETOWN: ::c_int = 9;
#[cfg(target_arch = "sparc64")]
pub const F_GETOWN: ::c_int = 5;
#[cfg(not(target_arch = "sparc64"))]
pub const F_SETOWN: ::c_int = 8;
#[cfg(target_arch = "sparc64")]
pub const F_SETOWN: ::c_int = 6;
#[cfg(not(target_arch = "sparc64"))]
pub const F_SETLK: ::c_int = 6;
#[cfg(target_arch = "sparc64")]
pub const F_SETLK: ::c_int = 8;
#[cfg(not(target_arch = "sparc64"))]
pub const F_SETLKW: ::c_int = 7;
#[cfg(target_arch = "sparc64")]
pub const F_SETLKW: ::c_int = 9;

pub const SEEK_DATA: ::c_int = 3;
pub const SEEK_HOLE: ::c_int = 4;

#[cfg(not(target_arch = "sparc64"))]
pub const SFD_NONBLOCK: ::c_int = 0x0800;
#[cfg(target_arch = "sparc64")]
pub const SFD_NONBLOCK: ::c_int = 0x4000;

pub const TCSANOW: ::c_int = 0;
pub const TCSADRAIN: ::c_int = 1;
pub const TCSAFLUSH: ::c_int = 2;

pub const TIOCGSOFTCAR: ::c_ulong = 0x40047464;
pub const TIOCSSOFTCAR: ::c_ulong = 0x80047465;
pub const TIOCLINUX: ::c_ulong = 0x541C;
pub const TIOCGSERIAL: ::c_ulong = 0x541E;
#[cfg(not(target_arch = "sparc64"))]
pub const TIOCEXCL: ::c_ulong = 0x540C;
#[cfg(target_arch = "sparc64")]
pub const TIOCEXCL: ::c_ulong = 0x2000740d;
#[cfg(not(target_arch = "sparc64"))]
pub const TIOCNXCL: ::c_ulong = 0x540D;
#[cfg(target_arch = "sparc64")]
pub const TIOCNXCL: ::c_ulong = 0x2000740e;
#[cfg(not(target_arch = "sparc64"))]
pub const TIOCSCTTY: ::c_ulong = 0x540E;
#[cfg(target_arch = "sparc64")]
pub const TIOCSCTTY: ::c_ulong = 0x20007484;
#[cfg(not(target_arch = "sparc64"))]
pub const TIOCSTI: ::c_ulong = 0x5412;
#[cfg(target_arch = "sparc64")]
pub const TIOCSTI: ::c_ulong = 0x80017472;
#[cfg(not(target_arch = "sparc64"))]
pub const TIOCMGET: ::c_ulong = 0x5415;
#[cfg(target_arch = "sparc64")]
pub const TIOCMGET: ::c_ulong = 0x4004746a;
#[cfg(not(target_arch = "sparc64"))]
pub const TIOCMBIS: ::c_ulong = 0x5416;
#[cfg(target_arch = "sparc64")]
pub const TIOCMBIS: ::c_ulong = 0x8004746c;
#[cfg(not(target_arch = "sparc64"))]
pub const TIOCMBIC: ::c_ulong = 0x5417;
#[cfg(target_arch = "sparc64")]
pub const TIOCMBIC: ::c_ulong = 0x8004746b;
#[cfg(not(target_arch = "sparc64"))]
pub const TIOCMSET: ::c_ulong = 0x5418;
#[cfg(target_arch = "sparc64")]
pub const TIOCMSET: ::c_ulong = 0x8004746d;
#[cfg(not(target_arch = "sparc64"))]
pub const TIOCCONS: ::c_ulong = 0x541D;
#[cfg(target_arch = "sparc64")]
pub const TIOCCONS: ::c_ulong = 0x20007424;

pub const RTLD_DEEPBIND: ::c_int = 0x8;
pub const RTLD_GLOBAL: ::c_int = 0x100;
pub const RTLD_NOLOAD: ::c_int = 0x4;

pub const LINUX_REBOOT_MAGIC1: ::c_int = 0xfee1dead;
pub const LINUX_REBOOT_MAGIC2: ::c_int = 672274793;
pub const LINUX_REBOOT_MAGIC2A: ::c_int = 85072278;
pub const LINUX_REBOOT_MAGIC2B: ::c_int = 369367448;
pub const LINUX_REBOOT_MAGIC2C: ::c_int = 537993216;

pub const LINUX_REBOOT_CMD_RESTART: ::c_int = 0x01234567;
pub const LINUX_REBOOT_CMD_HALT: ::c_int = 0xCDEF0123;
pub const LINUX_REBOOT_CMD_CAD_ON: ::c_int = 0x89ABCDEF;
pub const LINUX_REBOOT_CMD_CAD_OFF: ::c_int = 0x00000000;
pub const LINUX_REBOOT_CMD_POWER_OFF: ::c_int = 0x4321FEDC;
pub const LINUX_REBOOT_CMD_RESTART2: ::c_int = 0xA1B2C3D4;
pub const LINUX_REBOOT_CMD_SW_SUSPEND: ::c_int = 0xD000FCE2;
pub const LINUX_REBOOT_CMD_KEXEC: ::c_int = 0x45584543;

pub const NETLINK_ROUTE: ::c_int = 0;
pub const NETLINK_UNUSED: ::c_int = 1;
pub const NETLINK_USERSOCK: ::c_int = 2;
pub const NETLINK_FIREWALL: ::c_int = 3;
pub const NETLINK_SOCK_DIAG: ::c_int = 4;
pub const NETLINK_NFLOG: ::c_int = 5;
pub const NETLINK_XFRM: ::c_int = 6;
pub const NETLINK_SELINUX: ::c_int = 7;
pub const NETLINK_ISCSI: ::c_int = 8;
pub const NETLINK_AUDIT: ::c_int = 9;
pub const NETLINK_FIB_LOOKUP: ::c_int = 10;
pub const NETLINK_CONNECTOR: ::c_int = 11;
pub const NETLINK_NETFILTER: ::c_int = 12;
pub const NETLINK_IP6_FW: ::c_int = 13;
pub const NETLINK_DNRTMSG: ::c_int = 14;
pub const NETLINK_KOBJECT_UEVENT: ::c_int = 15;
pub const NETLINK_GENERIC: ::c_int = 16;
pub const NETLINK_SCSITRANSPORT: ::c_int = 18;
pub const NETLINK_ECRYPTFS: ::c_int = 19;
pub const NETLINK_RDMA: ::c_int = 20;
pub const NETLINK_CRYPTO: ::c_int = 21;
pub const NETLINK_INET_DIAG: ::c_int = NETLINK_SOCK_DIAG;

pub const MAX_LINKS: ::c_int = 32;

pub const NLM_F_REQUEST: ::c_int = 1;
pub const NLM_F_MULTI: ::c_int = 2;
pub const NLM_F_ACK: ::c_int = 4;
pub const NLM_F_ECHO: ::c_int = 8;
pub const NLM_F_DUMP_INTR: ::c_int = 16;
pub const NLM_F_DUMP_FILTERED: ::c_int = 32;

pub const NLM_F_ROOT: ::c_int = 0x100;
pub const NLM_F_MATCH: ::c_int = 0x200;
pub const NLM_F_ATOMIC: ::c_int = 0x400;
pub const NLM_F_DUMP: ::c_int = NLM_F_ROOT | NLM_F_MATCH;

pub const NLM_F_REPLACE: ::c_int = 0x100;
pub const NLM_F_EXCL: ::c_int = 0x200;
pub const NLM_F_CREATE: ::c_int = 0x400;
pub const NLM_F_APPEND: ::c_int = 0x800;

pub const NLMSG_NOOP: ::c_int = 0x1;
pub const NLMSG_ERROR: ::c_int = 0x2;
pub const NLMSG_DONE: ::c_int = 0x3;
pub const NLMSG_OVERRUN: ::c_int = 0x4;
pub const NLMSG_MIN_TYPE: ::c_int = 0x10;

pub const NETLINK_ADD_MEMBERSHIP: ::c_int = 1;
pub const NETLINK_DROP_MEMBERSHIP: ::c_int = 2;
pub const NETLINK_PKTINFO: ::c_int = 3;
pub const NETLINK_BROADCAST_ERROR: ::c_int = 4;
pub const NETLINK_NO_ENOBUFS: ::c_int = 5;
pub const NETLINK_RX_RING: ::c_int = 6;
pub const NETLINK_TX_RING: ::c_int = 7;
pub const NETLINK_LISTEN_ALL_NSID: ::c_int = 8;
pub const NETLINK_LIST_MEMBERSHIPS: ::c_int = 9;
pub const NETLINK_CAP_ACK: ::c_int = 10;

pub const NLA_F_NESTED: ::c_int = 1 << 15;
pub const NLA_F_NET_BYTEORDER: ::c_int = 1 << 14;
pub const NLA_TYPE_MASK: ::c_int = !(NLA_F_NESTED | NLA_F_NET_BYTEORDER);

cfg_if! {
    if #[cfg(any(target_arch = "arm", target_arch = "x86",
                 target_arch = "x86_64"))] {
        pub const PTHREAD_STACK_MIN: ::size_t = 16384;
    } else if #[cfg(target_arch = "sparc64")] {
        pub const PTHREAD_STACK_MIN: ::size_t = 0x6000;
    } else {
        pub const PTHREAD_STACK_MIN: ::size_t = 131072;
    }
}

extern {
    pub fn utmpxname(file: *const ::c_char) -> ::c_int;
    pub fn getutxent() -> *mut utmpx;
    pub fn getutxid(ut: *const utmpx) -> *mut utmpx;
    pub fn getutxline(ut: *const utmpx) -> *mut utmpx;
    pub fn pututxline(ut: *const utmpx) -> *mut utmpx;
    pub fn setutxent();
    pub fn endutxent();
    pub fn getpt() -> ::c_int;
}

#[link(name = "util")]
extern {
    pub fn sysctl(name: *mut ::c_int,
                  namelen: ::c_int,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *mut ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
    pub fn backtrace(buf: *mut *mut ::c_void,
                     sz: ::c_int) -> ::c_int;
    pub fn glob64(pattern: *const ::c_char,
                  flags: ::c_int,
                  errfunc: ::dox::Option<extern fn(epath: *const ::c_char,
                                                   errno: ::c_int)
                                                   -> ::c_int>,
                  pglob: *mut glob64_t) -> ::c_int;
    pub fn globfree64(pglob: *mut glob64_t);
    pub fn ptrace(request: ::c_uint, ...) -> ::c_long;
    pub fn pthread_attr_getaffinity_np(attr: *const ::pthread_attr_t,
                                       cpusetsize: ::size_t,
                                       cpuset: *mut ::cpu_set_t) -> ::c_int;
    pub fn pthread_attr_setaffinity_np(attr: *mut ::pthread_attr_t,
                                       cpusetsize: ::size_t,
                                       cpuset: *const ::cpu_set_t) -> ::c_int;
    pub fn getpriority(which: ::__priority_which_t, who: ::id_t) -> ::c_int;
    pub fn setpriority(which: ::__priority_which_t, who: ::id_t,
                                       prio: ::c_int) -> ::c_int;
    pub fn pthread_getaffinity_np(thread: ::pthread_t,
                                  cpusetsize: ::size_t,
                                  cpuset: *mut ::cpu_set_t) -> ::c_int;
    pub fn pthread_setaffinity_np(thread: ::pthread_t,
                                  cpusetsize: ::size_t,
                                  cpuset: *const ::cpu_set_t) -> ::c_int;
    pub fn sched_getcpu() -> ::c_int;
}

cfg_if! {
    if #[cfg(any(target_arch = "x86",
                 target_arch = "arm",
                 target_arch = "powerpc"))] {
        mod b32;
        pub use self::b32::*;
    } else if #[cfg(any(target_arch = "x86_64",
                        target_arch = "aarch64",
                        target_arch = "powerpc64",
                        target_arch = "sparc64"))] {
        mod b64;
        pub use self::b64::*;
    } else {
        // Unknown target_arch
    }
}
