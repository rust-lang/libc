pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type caddr_t = *mut ::c_char;

// It must be c_long, but time.rs accepts only i32
//pub type clockid_t = ::c_long;
pub type clockid_t = ::c_int;
pub type blkcnt_t = ::c_long;
pub type clock_t = ::c_int;
pub type daddr_t = ::c_long;
pub type dev_t = ::c_ulong;
pub type fsblkcnt_t = ::c_ulong;
pub type fsfilcnt_t = ::c_ulong;
pub type ino_t = ::c_ulong;
pub type key_t = ::c_long;
pub type mode_t = ::c_uint;
pub type nlink_t = ::c_short;
pub type rlim_t = ::c_ulong;
pub type speed_t = ::c_uint;
pub type tcflag_t = ::c_uint;
pub type time_t = ::c_long;
pub type timer_t = ::c_long;
pub type wchar_t = ::c_uint;
pub type nfds_t = ::c_int;
pub type projid_t = ::c_int;

pub type suseconds_t = ::c_int;
pub type off_t = ::c_long;
pub type useconds_t = ::c_uint;

pub type socklen_t = ::c_uint;
pub type sa_family_t = ::c_uchar;
pub type in_port_t = ::c_ushort;
pub type in_addr_t = ::c_uint;

pub type pthread_t = ::c_uint;
pub type pthread_key_t = ::c_uint;
pub type blksize_t = ::c_long;
pub type nl_item = ::c_int;
pub type mqd_t = ::c_int;
pub type id_t = ::c_ulong;
pub type shmatt_t = ::c_ulong;

pub type sem_t = ::c_int;

s! {
    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct fd_set {
        fds_bits: [i64; FD_SETSIZE / 64],
    }

    pub struct dirent {
        pub d_offset: ::c_ulong,
        pub d_ino: ::ino_t,
        pub d_reclen: ::c_ushort,
        pub d_namlen: ::c_ushort,
        pub d_name: [::c_char; _D_NAME_MAX+1]
    }

    pub struct sigset_t {
        bits: [c_ulong; 4],
    }

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_cc: [::cc_t; ::NCCS]
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
        pub f_basetype: [::c_char; _FSTYPSIZ],
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        pub f_fstr: [::c_char; 32],
        pub f_filler: [::c_ulong; 16]
    }

    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
        pub int_n_sign_posn: ::c_char,
    }

    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int
    }

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: ::c_ulong,
        pub ai_canonname: *mut ::c_char,
        pub ai_addr: *mut ::sockaddr,
        pub ai_next: *mut addrinfo,
    }

    pub struct in_addr {
        pub s_addr: in_addr_t
    }

    pub struct sockaddr {
        pub sa_len: ::c_uchar,
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_len: ::c_uchar,
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [::c_char; 8]
    }

    pub struct sockaddr_in6 {
        pub sin6_len: ::c_uchar,
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: ::uint32_t,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: ::uint32_t
    }

    pub struct sockaddr_storage {
        pub __ss_len: ::c_uchar,
        pub ss_family: sa_family_t,
        __ss_pad1: [u8; 6],
        __ss_align: i64,
        __ss_pad2: [u8; 1265],
    }

    pub struct sockaddr_un {
        pub sun_len: ::c_uchar,
        pub sun_family: sa_family_t,
        pub sun_path: [::c_char; 1023]
    }

    pub struct sigaction {
        pub sa_handler: ::sighandler_t,
        pub sa_mask: sigset_t,
        pub sa_flags: ::c_int,
    }

    pub struct pthread_rwlockattr_t {
        __pthread_rwlockattrp: *mut ::c_void,
    }

    pub struct pthread_rwlock_t {
        __rw_word: [::c_long; 10],
    }

    pub struct pthread_cond_t {
        __cv_word: [::c_long; 6],
    }

    pub struct pthread_condattr_t {
        __pthread_condattrp: *mut ::c_void,
    }

    pub struct pthread_mutexattr_t {
        __pthread_mutexattr_t: *mut ::c_void,
    }

    pub struct pthread_attr_t {
        __pthread_attr_t: *mut ::c_void,
    }

    pub struct pthread_mutex_t {
        __mt_word: [::c_long; 8],
    }

    pub struct st_timespec {
        pub tv_sec: ::time_t,
        pub tv_nsec: ::c_int,
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_flag: ::c_ushort,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_ssize: ::c_int,
        pub st_atime: ::st_timespec,
        pub st_mtime: ::st_timespec,
        pub st_ctime: ::st_timespec,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_vfstype: ::c_int,
        pub st_vfs: ::c_uint,
        pub st_type: ::c_uint,
        pub st_gen: ::c_uint,
        pub st_reserved: [::c_char; 9],
        pub st_padto_ll: ::c_uint,
        pub st_size: off_t
    }

    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char
    }

    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

}

pub const FD_SETSIZE: usize = 256;
pub const _D_NAME_MAX: usize = 255;
pub const NCCS: usize = 16;
pub const _FSTYPSIZ: usize = 16;

pub const LC_CTYPE: ::c_int = 1;
pub const LC_NUMERIC: ::c_int = 3;
pub const LC_TIME: ::c_int = 4;
pub const LC_COLLATE: ::c_int = 0;
pub const LC_MONETARY: ::c_int = 2;
pub const LC_MESSAGES: ::c_int = 4;
pub const LC_ALL: ::c_int = -1;
pub const LC_CTYPE_MASK: ::c_int = 2;
pub const LC_NUMERIC_MASK: ::c_int = 16;
pub const LC_TIME_MASK: ::c_int = 32;
pub const LC_COLLATE_MASK: ::c_int = 1;
pub const LC_MONETARY_MASK: ::c_int = 8;
pub const LC_MESSAGES_MASK: ::c_int = 4;
pub const LC_ALL_MASK: ::c_int = LC_CTYPE_MASK
    | LC_NUMERIC_MASK
    | LC_TIME_MASK
    | LC_COLLATE_MASK
    | LC_MONETARY_MASK
    | LC_MESSAGES_MASK;

pub const DAY_1: ::nl_item = 13;
pub const DAY_2: ::nl_item = 14;
pub const DAY_3: ::nl_item = 15;
pub const DAY_4: ::nl_item = 16;
pub const DAY_5: ::nl_item = 17;
pub const DAY_6: ::nl_item = 18;
pub const DAY_7: ::nl_item = 19;

pub const ABDAY_1: ::nl_item = 6;
pub const ABDAY_2: ::nl_item = 7;
pub const ABDAY_3: ::nl_item = 8;
pub const ABDAY_4: ::nl_item = 9;
pub const ABDAY_5: ::nl_item = 10;
pub const ABDAY_6: ::nl_item = 11;
pub const ABDAY_7: ::nl_item = 12;

pub const MON_1: ::nl_item = 32;
pub const MON_2: ::nl_item = 33;
pub const MON_3: ::nl_item = 34;
pub const MON_4: ::nl_item = 35;
pub const MON_5: ::nl_item = 36;
pub const MON_6: ::nl_item = 37;
pub const MON_7: ::nl_item = 38;
pub const MON_8: ::nl_item = 39;
pub const MON_9: ::nl_item = 40;
pub const MON_10: ::nl_item = 41;
pub const MON_11: ::nl_item = 42;
pub const MON_12: ::nl_item = 43;

pub const ABMON_1: ::nl_item = 20;
pub const ABMON_2: ::nl_item = 21;
pub const ABMON_3: ::nl_item = 22;
pub const ABMON_4: ::nl_item = 23;
pub const ABMON_5: ::nl_item = 24;
pub const ABMON_6: ::nl_item = 25;
pub const ABMON_7: ::nl_item = 26;
pub const ABMON_8: ::nl_item = 27;
pub const ABMON_9: ::nl_item = 28;
pub const ABMON_10: ::nl_item = 29;
pub const ABMON_11: ::nl_item = 30;
pub const ABMON_12: ::nl_item = 31;

pub const RADIXCHAR: ::nl_item = 44;
pub const THOUSEP: ::nl_item = 45;
pub const YESSTR: ::nl_item = 46;
pub const NOSTR: ::nl_item = 47;
pub const CRNCYSTR: ::nl_item = 48;

pub const D_T_FMT: ::nl_item = 1;
pub const D_FMT: ::nl_item = 2;
pub const T_FMT: ::nl_item = 3;
pub const AM_STR: ::nl_item = 4;
pub const PM_STR: ::nl_item = 5;

pub const CODESET: ::nl_item = 49;
pub const T_FMT_AMPM: ::nl_item = 55;
pub const ERA: ::nl_item = 56;
pub const ERA_D_FMT: ::nl_item = 57;
pub const ERA_D_T_FMT: ::nl_item = 58;
pub const ERA_T_FMT: ::nl_item = 59;
pub const ALT_DIGITS: ::nl_item = 60;
pub const YESEXPR: ::nl_item = 61;
pub const NOEXPR: ::nl_item = 62;

pub const PATH_MAX: ::c_int = 1024;

pub const SA_ONSTACK: ::c_int = 0x00000001;
pub const SA_RESETHAND: ::c_int = 0x00000002;
pub const SA_RESTART: ::c_int = 0x00000008;
pub const SA_SIGINFO: ::c_int = 0x00000100;
pub const SA_NODEFER: ::c_int = 0x00000200;
pub const SA_NOCLDWAIT: ::c_int = 0x00000400;
pub const SA_NOCLDSTOP: ::c_int = 0x00000004;

pub const SS_ONSTACK: ::c_int = 0x00000001;
pub const SS_DISABLE: ::c_int = 0x00000002;

pub const IOCPARM_MASK: ::c_int = 0x7f; /* parameters must be < 128 bytes */
pub const IOC_VOID:  ::c_int = 0x20000000; /* no parameters */
pub const IOC_OUT:   ::c_int = 0x40000000; /* copy out parameters */
pub const IOC_IN:    ::c_int = 0x40000000<<1; /* copy in parameters */
pub const IOC_INOUT: ::c_int = IOC_IN|IOC_OUT;
pub const fn _IO(x: ::c_int, y: ::c_int) -> ::c_int {IOC_VOID|(x<<8)|y}
// In AIX header, they use sizeof, but it is not available yet.
// Workaround: define these function for int only.
pub const fn _IOR_int(x: ::c_int, y: ::c_int) -> ::c_int {IOC_OUT|((32&IOCPARM_MASK)<<16)|(x<<8)|y}
pub const fn _IOW_int(x: ::c_int, y: ::c_int) -> ::c_int {IOC_IN |((32&IOCPARM_MASK)<<16)|(x<<8)|y}

pub const FIOCLEX: ::c_int = _IO('f' as ::c_int, 1);
pub const FIONCLEX: ::c_int = _IO('f' as ::c_int, 2);
pub const FIONREAD: ::c_int = _IOR_int('f' as ::c_int, 127);
pub const FIONBIO: ::c_int = _IOW_int('f' as ::c_int, 126);
pub const FIOASYNC: ::c_int = _IOW_int('f' as ::c_int, 125);
pub const FIOSETOWN: ::c_int = _IOW_int('f' as ::c_int, 124);
pub const FIOGETOWN: ::c_int = _IOR_int('f' as ::c_int, 123);

pub const SIGCHLD: ::c_int = 20;
pub const SIGBUS: ::c_int = 10;
pub const SIG_BLOCK: ::c_int = 0;
pub const SIG_UNBLOCK: ::c_int = 1;
pub const SIG_SETMASK: ::c_int = 2;

pub const SIGEV_NONE: ::c_int = 1;
pub const SIGEV_SIGNAL: ::c_int = 2;
pub const SIGEV_THREAD: ::c_int = 3;

pub const IP_RECVDSTADDR: ::c_int = 7;
pub const IP_TTL: ::c_int = 4;
pub const IP_MULTICAST_IF: ::c_int = 9;
pub const IP_MULTICAST_TTL: ::c_int = 10;
pub const IP_MULTICAST_LOOP: ::c_int = 11;
pub const IPV6_UNICAST_HOPS: ::c_int = IP_TTL;
pub const IPV6_MULTICAST_IF: ::c_int = IP_MULTICAST_IF;
pub const IPV6_MULTICAST_HOPS: ::c_int = IP_MULTICAST_TTL;
pub const IPV6_MULTICAST_LOOP: ::c_int = IP_MULTICAST_LOOP;
pub const IPV6_RECVPKTINFO: ::c_int = 35;
pub const IPV6_V6ONLY: ::c_int = 37;

pub const ST_RDONLY: ::c_ulong = 0x0001;
pub const ST_NOSUID: ::c_ulong = 0x0040;

pub const NI_MAXHOST: ::socklen_t = 1025;

pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;
pub const RAND_MAX: ::c_int = 32767;
pub const EOF: ::c_int = -1;
pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
pub const _IOFBF: ::c_int = 0000;
pub const _IONBF: ::c_int = 0004;
pub const _IOLBF: ::c_int = 0100;
pub const BUFSIZ: ::c_uint = 4096;
pub const FOPEN_MAX: ::c_uint = 32767;
pub const FILENAME_MAX: ::c_uint = 255;
pub const L_tmpnam: ::c_uint = 21;
pub const TMP_MAX: ::c_uint = 16384;

pub const O_RDONLY: ::c_int = 0;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDWR: ::c_int = 2;
pub const O_NDELAY: ::c_int = _FNDELAY;
pub const O_APPEND: ::c_int = _FAPPEND;
pub const O_DSYNC: ::c_int = _FDATASYNC;
pub const O_CREAT: ::c_int = _FCREAT;
pub const O_EXCL: ::c_int = _FEXCL;
pub const O_NOCTTY: ::c_int = _FNOCTTY;
pub const O_TRUNC: ::c_int = _FTRUNC;
pub const O_NOFOLLOW: ::c_int = _FNOFOLLOW;
pub const O_DIRECTORY: ::c_int = _FSYNCALL;
pub const O_SEARCH: ::c_int = _FEXEC;
pub const O_EXEC: ::c_int = _FEXEC;
pub const O_CLOEXEC: ::c_int = _FCLOEXEC;
pub const O_ACCMODE: ::c_int = O_RDONLY | O_WRONLY | O_RDWR;
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
pub const F_DUPFD_CLOEXEC: ::c_int = 16;
pub const F_GETLK64: ::c_int = 11;
pub const F_SETLK64: ::c_int = 12;
pub const F_SETLKW64: ::c_int = 13;
pub const F_DUP2FD: ::c_int = 14;
pub const F_TSTLK: ::c_int = 15;
pub const F_GETLK: ::c_int = F_GETLK64;
pub const F_SETLK: ::c_int = F_SETLK64;
pub const F_SETLKW: ::c_int = F_SETLKW64;
pub const _S_IFMT:      mode_t =  0170000;
pub const _S_IFREG:     mode_t =  0100000;
pub const _S_IFDIR:     mode_t =  0040000;
pub const _S_IFBLK:     mode_t =  0060000;
pub const _S_IFCHR:     mode_t =  0020000;
pub const _S_IFIFO:     mode_t =  0010000;
pub const S_IFMT:       mode_t =  _S_IFMT;
pub const S_IFREG:      mode_t =  _S_IFREG;
pub const S_IFDIR:      mode_t =  _S_IFDIR;
pub const S_IFBLK:      mode_t =  _S_IFBLK;
pub const S_IFCHR:      mode_t =  _S_IFCHR;
pub const S_IFIFO:      mode_t =  _S_IFIFO;
pub const S_IRWXU:      mode_t =  0000700;
pub const S_IRUSR:      mode_t =  0000400;
pub const S_IWUSR:      mode_t =  0000200;
pub const S_IXUSR:      mode_t =  0000100;
pub const S_IRWXG:      mode_t =  0000070;
pub const S_IRGRP:      mode_t =  0000040;
pub const S_IWGRP:      mode_t =  0000020;
pub const S_IXGRP:      mode_t =  0000010;
pub const S_IRWXO:      mode_t =  0000007;
pub const S_IROTH:      mode_t =  0000004;
pub const S_IWOTH:      mode_t =  0000002;
pub const S_IXOTH:      mode_t =  0000001;
pub const S_IFLNK:      mode_t =  0120000;
pub const S_IFSOCK:     mode_t =  0140000;
pub const _FREAD:      ::c_int =  0x00000001;
pub const _FWRITE:     ::c_int =  0x00000002;
pub const _FNONBLOCK:  ::c_int =  0x00000004;
pub const _FAPPEND:    ::c_int =  0x00000008;
pub const _FSYNC:      ::c_int =  0x00000010;
pub const _FEXEC:      ::c_int =  0x00000020;
pub const _FSNAPSHOT:  ::c_int =  0x00000040;
pub const _FCIO:       ::c_int =  0x00000080;
pub const _FCREAT:     ::c_int =  0x00000100;
pub const _FTRUNC:     ::c_int =  0x00000200;
pub const _FEXCL:      ::c_int =  0x00000400;
pub const _FNOCTTY:    ::c_int =  0x00000800;
pub const _FRSHARE:    ::c_int =  0x00001000;
pub const _FDEFER:     ::c_int =  0x00002000;
pub const _FDELAY:     ::c_int =  0x00004000;
pub const _FNDELAY:    ::c_int =  0x00008000;
pub const _FNSHARE:    ::c_int =  0x00010000;
pub const _FASYNC:     ::c_int =  0x00020000;
pub const _FMOUNT:     ::c_int =  0x00040000;
pub const _FSYNCALL:   ::c_int =  0x00080000;
pub const _FNOCACHE:   ::c_int =  0x00100000;
pub const _FREADSYNC:  ::c_int =  0x00200000;
pub const _FDATASYNC:  ::c_int =  0x00400000;
pub const _FCLOEXEC:   ::c_int =  0x00800000;
pub const _FNOFOLLOW:  ::c_int =  0x01000000;
pub const _FTTYINIT:   ::c_int =  0x0;
pub const _FCLREAD:    ::c_int =  0x02000000;
pub const _FLARGEFILE: ::c_int =  0x04000000;
pub const _FDIRECT:    ::c_int =  0x08000000;
pub const _FDOCLONE:   ::c_int =  0x10000000;
pub const _FALT:       ::c_int =  0x20000000;
pub const _FKERNEL:    ::c_int =  0x40000000;
pub const _FMSYNC:     ::c_int =  0x80000000;
pub const _FRAW:       ::c_long = 0x0000000100000000;
pub const _FEFSON:     ::c_long = 0x0000000200000000;
pub const _FEFSOFF:    ::c_long = 0x0000000400000000;
pub const _FCIOR:      ::c_long = 0x0000000800000000;
pub const _FDEFERIND:  ::c_long = 0x0000001000000000;
pub const _FDATAFLUSH: ::c_long = 0x0000002000000000;

pub const SIGHUP: ::c_int = 1;
pub const SIGINT: ::c_int = 2;
pub const SIGQUIT: ::c_int = 3;
pub const SIGILL: ::c_int = 4;
pub const SIGABRT: ::c_int = 6;
pub const SIGEMT: ::c_int = 7;
pub const SIGFPE: ::c_int = 8;
pub const SIGKILL: ::c_int = 9;
pub const SIGSEGV: ::c_int = 11;
pub const SIGSYS: ::c_int = 12;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGTERM: ::c_int = 15;
pub const SIGUSR1: ::c_int = 30;
pub const SIGUSR2: ::c_int = 31;
pub const SIGPWR: ::c_int = 29;
pub const SIGWINCH: ::c_int = 28;
pub const SIGURG: ::c_int = 16;
pub const SIGPOLL: ::c_int = SIGIO;
pub const SIGIO: ::c_int = 23;
pub const SIGSTOP: ::c_int = 17;
pub const SIGTSTP: ::c_int = 18;
pub const SIGCONT: ::c_int = 19;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGVTALRM: ::c_int = 34;
pub const SIGPROF: ::c_int = 32;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;

pub const WNOHANG: ::c_int = 0x1;
pub const WUNTRACED: ::c_int = 0x2;
pub const WEXITED: ::c_int = 0x04;
pub const WCONTINUED: ::c_int = 0x01000000;
pub const WNOWAIT: ::c_int = 0x10;
pub const _W_STOPPED:    ::c_int = 0x00000040;
pub const _W_SLWTED:     ::c_int = 0x0000007c;
pub const _W_SEWTED:     ::c_int = 0x0000007d;
pub const _W_SFWTED:     ::c_int = 0x0000007e;
pub const _W_STRC:       ::c_int = 0x0000007f;

pub const AT_FDCWD: ::c_int = -2;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 1;
pub const AT_SYMLINK_FOLLOW: ::c_int = 2;
pub const AT_REMOVEDIR: ::c_int = 1;
pub const AT_EACCESS: ::c_int = 1;

pub const PROT_NONE: ::c_int = 0;
pub const PROT_READ: ::c_int = 0x1;
pub const PROT_WRITE: ::c_int = 0x2;
pub const PROT_EXEC: ::c_int = 0x4;

pub const MAP_FILE: ::c_int = 0x00;
pub const MAP_SHARED: ::c_int = 0x1;
pub const MAP_PRIVATE: ::c_int = 0x2;
pub const MAP_FIXED: ::c_int = 0x100;
pub const MAP_ANON: ::c_int = 0x10;
pub const MAP_ANONYMOUS: ::c_int = 0x10;

pub const MCL_CURRENT: ::c_int = 0x00000100;
pub const MCL_FUTURE: ::c_int = 0x00000200;

pub const MS_SYNC: ::c_int = 0x20;
pub const MS_ASYNC: ::c_int = 0x10;
pub const MS_INVALIDATE: ::c_int = 0x40;

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
pub const EAGAIN: ::c_int = 11;
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
pub const ENOMSG: ::c_int = 35;
pub const EIDRM: ::c_int = 36;
pub const ECHRNG: ::c_int = 37;
pub const EL2NSYNC: ::c_int = 38;
pub const EL3HLT: ::c_int = 39;
pub const EL3RST: ::c_int = 40;
pub const ELNRNG: ::c_int = 41;
pub const EUNATCH: ::c_int = 42;
pub const ENOCSI: ::c_int = 43;
pub const EL2HLT: ::c_int = 44;
pub const EDEADLK: ::c_int = 45;
pub const ENOLCK: ::c_int = 49;
pub const ECANCELED: ::c_int = 117;
pub const ENOTSUP: ::c_int = 124;
pub const EDQUOT: ::c_int = 88;
pub const EOWNERDEAD: ::c_int = 95;
pub const ENOTRECOVERABLE: ::c_int = 94;
pub const ENOSTR: ::c_int = 123;
pub const ENODATA: ::c_int = 122;
pub const ETIME: ::c_int = 119;
pub const ENOSR: ::c_int = 118;
pub const EREMOTE: ::c_int = 93;
pub const ENOLINK: ::c_int = 126;
pub const EPROTO: ::c_int = 121;
pub const EMULTIHOP: ::c_int = 125;
pub const EBADMSG: ::c_int = 120;
pub const ENAMETOOLONG: ::c_int = 86;
pub const EOVERFLOW: ::c_int = 127;
pub const EILSEQ: ::c_int = 116;
pub const ENOSYS: ::c_int = 109;
pub const ELOOP: ::c_int = 85;
pub const ERESTART: ::c_int = 82;
pub const ENOTEMPTY: ::c_int = EEXIST;
pub const EUSERS: ::c_int = 84;
pub const ENOTSOCK: ::c_int = 57;
pub const EDESTADDRREQ: ::c_int = 58;
pub const EMSGSIZE: ::c_int = 59;
pub const EPROTOTYPE: ::c_int = 60;
pub const ENOPROTOOPT: ::c_int = 61;
pub const EPROTONOSUPPORT: ::c_int = 62;
pub const ESOCKTNOSUPPORT: ::c_int = 63;
pub const EOPNOTSUPP: ::c_int = 64;
pub const EPFNOSUPPORT: ::c_int = 65;
pub const EAFNOSUPPORT: ::c_int = 66;
pub const EADDRINUSE: ::c_int = 67;
pub const EADDRNOTAVAIL: ::c_int = 68;
pub const ENETDOWN: ::c_int = 69;
pub const ENETUNREACH: ::c_int = 70;
pub const ENETRESET: ::c_int = 71;
pub const ECONNABORTED: ::c_int = 72;
pub const ECONNRESET: ::c_int = 73;
pub const ENOBUFS: ::c_int = 74;
pub const EISCONN: ::c_int = 75;
pub const ENOTCONN: ::c_int = 76;
pub const ESHUTDOWN: ::c_int = 77;
pub const ETOOMANYREFS: ::c_int = 115;
pub const ETIMEDOUT: ::c_int = 78;
pub const ECONNREFUSED: ::c_int = 79;
pub const EHOSTDOWN: ::c_int = 80;
pub const EHOSTUNREACH: ::c_int = 81;
pub const EWOULDBLOCK: ::c_int = EAGAIN;
pub const EALREADY: ::c_int = 56;
pub const EINPROGRESS: ::c_int = 55;
pub const ESTALE: ::c_int = 52;

pub const EAI_AGAIN: ::c_int = 2;
pub const EAI_BADFLAGS: ::c_int = 3;
pub const EAI_FAIL: ::c_int = 4;
pub const EAI_FAMILY: ::c_int = 5;
pub const EAI_MEMORY: ::c_int = 6;
pub const EAI_NODATA: ::c_int = 7;
pub const EAI_NONAME: ::c_int = 8;
pub const EAI_SERVICE: ::c_int = 9;
pub const EAI_SOCKTYPE: ::c_int = 10;
pub const EAI_SYSTEM: ::c_int = 11;
pub const EAI_OVERFLOW: ::c_int = 13;

pub const F_DUPFD: ::c_int = 0;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;
pub const SIGTRAP: ::c_int = 5;

pub const GLOB_APPEND: ::c_int = 0x01;
pub const GLOB_DOOFFS: ::c_int = 0x02;
pub const GLOB_ERR: ::c_int = 0x04;
pub const GLOB_MARK: ::c_int = 0x08;
pub const GLOB_NOCHECK: ::c_int = 0x10;
pub const GLOB_NOSORT: ::c_int = 0x20;
pub const GLOB_NOESCAPE: ::c_int = 0x80;
pub const GLOB_NOSPACE: ::c_int = 0x2000;
pub const GLOB_ABORTED: ::c_int = 0x1000;
pub const GLOB_NOMATCH: ::c_int = 0x4000;

pub const POLLIN: ::c_short = 0x0001;
pub const POLLPRI: ::c_short = 0x0004;
pub const POLLOUT: ::c_short = 0x0002;
pub const POLLERR: ::c_short = 0x4000;
pub const POLLHUP: ::c_short = 0x2000;
pub const POLLMSG: ::c_short = 0x0080;
pub const POLLSYNC: ::c_short = 0x8000;
pub const POLLNVAL: ::c_short = POLLSYNC;
pub const POLLNORM: ::c_short = POLLIN;
pub const POLLRDNORM: ::c_short = 0x0010;
pub const POLLWRNORM: ::c_short = POLLOUT;
pub const POLLRDBAND: ::c_short = 0x0020;
pub const POLLWRBAND: ::c_short = 0x0040;

pub const POSIX_MADV_NORMAL: ::c_int = 1;
pub const POSIX_MADV_RANDOM: ::c_int = 3;
pub const POSIX_MADV_SEQUENTIAL: ::c_int = 2;
pub const POSIX_MADV_WILLNEED: ::c_int = 4;
pub const POSIX_MADV_DONTNEED: ::c_int = 5;

pub const PAGESIZE: ::c_int = 4096;
pub const PTHREAD_CREATE_JOINABLE: ::c_int = 0;
pub const PTHREAD_CREATE_DETACHED: ::c_int = 1;
pub const PTHREAD_PROCESS_SHARED: ::c_int = 0;
pub const PTHREAD_PROCESS_PRIVATE: ::c_ushort = 1;
pub const PTHREAD_STACK_MIN: ::size_t = PAGESIZE as ::size_t * 4;

pub const SIGSTKSZ: ::size_t = 4096;

pub const TIMEOFDAY: ::c_int = 9;
pub const CLOCK_REALTIME: ::clockid_t = TIMEOFDAY as clockid_t;
pub const CLOCK_MONOTONIC: ::clockid_t = 10;
pub const TIMER_ABSTIME: ::c_int = 999;

pub const RLIMIT_CPU: ::c_int = 0;
pub const RLIMIT_FSIZE: ::c_int = 1;
pub const RLIMIT_DATA: ::c_int = 2;
pub const RLIMIT_STACK: ::c_int = 3;
pub const RLIMIT_CORE: ::c_int = 4;
pub const RLIMIT_NOFILE: ::c_int = 7;
pub const RLIMIT_AS: ::c_int = 6;

pub const RUSAGE_SELF: ::c_int = 0;
pub const RUSAGE_CHILDREN: ::c_int = -1;

pub const MADV_NORMAL: ::c_int = 0;
pub const MADV_RANDOM: ::c_int = 1;
pub const MADV_SEQUENTIAL: ::c_int = 2;
pub const MADV_WILLNEED: ::c_int = 3;
pub const MADV_DONTNEED: ::c_int = 4;

pub const AF_UNSPEC: ::c_int = 0;
pub const AF_UNIX: ::c_int = 1;
pub const AF_INET: ::c_int = 2;
pub const AF_IMPLINK: ::c_int = 3;
pub const AF_PUP: ::c_int = 4;
pub const AF_CHAOS: ::c_int = 5;
pub const AF_NS: ::c_int = 6;
pub const AF_ECMA: ::c_int = 8;
pub const AF_DATAKIT: ::c_int = 9;
pub const AF_CCITT: ::c_int = 10;
pub const AF_SNA: ::c_int = 11;
pub const AF_DECnet: ::c_int = 12;
pub const AF_DLI: ::c_int = 13;
pub const AF_LAT: ::c_int = 14;
pub const AF_HYLINK: ::c_int = 15;
pub const AF_APPLETALK: ::c_int = 16;
pub const AF_ISO: ::c_int = 7;
pub const AF_OSI: ::c_int = AF_ISO;
pub const AF_ROUTE: ::c_int = 17;
pub const AF_LINK: ::c_int = 18;
pub const AF_INET6: ::c_int = 24;

pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_RAW: ::c_int = 3;
pub const SOCK_RDM: ::c_int = 4;
pub const SOCK_SEQPACKET: ::c_int = 5;

pub const IP_HDRINCL: ::c_int = 2;
pub const IP_ADD_MEMBERSHIP: ::c_int = 12;
pub const IP_DROP_MEMBERSHIP: ::c_int = 13;
pub const IPV6_ADD_MEMBERSHIP: ::c_int = IP_ADD_MEMBERSHIP;
pub const IPV6_DROP_MEMBERSHIP: ::c_int = IP_DROP_MEMBERSHIP;
pub const IPV6_JOIN_GROUP: ::c_int = IP_ADD_MEMBERSHIP;
pub const IPV6_LEAVE_GROUP: ::c_int = IP_DROP_MEMBERSHIP;

pub const TCP_NODELAY: ::c_int = 0x01;
pub const TCP_MAXSEG: ::c_int = 0x02;
pub const TCP_KEEPALIVE: ::c_int = 0x8;

pub const SOL_SOCKET: ::c_int = 0xffff;
pub const SO_DEBUG: ::c_int = 0x0001;
pub const SO_ACCEPTCONN: ::c_int = 0x0002;
pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_BROADCAST: ::c_int = 0x0020;
pub const SO_USELOOPBACK: ::c_int = 0x0040;
pub const SO_LINGER: ::c_int = 0x0080;
pub const SO_OOBINLINE: ::c_int = 0x0100;
pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_SNDLOWAT: ::c_int = 0x1003;
pub const SO_RCVLOWAT: ::c_int = 0x1004;
pub const SO_SNDTIMEO: ::c_int = 0x1005;
pub const SO_RCVTIMEO: ::c_int = 0x1006;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_TYPE: ::c_int = 0x1008;

pub const SCM_RIGHTS: ::c_int = 0x01;

pub const MSG_OOB: ::c_int = 0x1;
pub const MSG_PEEK: ::c_int = 0x2;
pub const MSG_DONTROUTE: ::c_int = 0x4;
pub const MSG_EOR: ::c_int = 0x8;
pub const MSG_CTRUNC: ::c_int = 0x20;
pub const MSG_TRUNC: ::c_int = 0x10;
pub const MSG_WAITALL: ::c_int = 0x40;
pub const MSG_NOSIGNAL: ::c_int = 0x100;
pub const MSG_MAXIOVLEN: ::c_int = 16;

pub const IFF_UP: ::c_int = 0x1;
pub const IFF_BROADCAST: ::c_int = 0x2;
pub const IFF_DEBUG: ::c_int = 0x4;
pub const IFF_LOOPBACK: ::c_int = 0x8;
pub const IFF_POINTOPOINT: ::c_int = 0x10;
pub const IFF_NOTRAILERS: ::c_int = 0x20;
pub const IFF_RUNNING: ::c_int = 0x40;
pub const IFF_NOARP: ::c_int = 0x80;
pub const IFF_PROMISC: ::c_int = 0x100;
pub const IFF_ALLMULTI: ::c_int = 0x200;
pub const IFF_MULTICAST: ::c_int = 0x80000;

pub const IPC_ALLOC: ::c_int = 0100000;
pub const IPC_CREAT: ::c_int = 0020000;
pub const IPC_EXCL: ::c_int = 0002000;
pub const IPC_NOWAIT: ::c_int = 0004000;
pub const IPC_RMID: ::c_int = 0;
pub const IPC_SET: ::c_int = 101;

pub const SHUT_RD: ::c_int = 0;
pub const SHUT_WR: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;

pub const LOCK_SH: ::c_int = 1;
pub const LOCK_EX: ::c_int = 2;
pub const LOCK_NB: ::c_int = 4;
pub const LOCK_UN: ::c_int = 8;

pub const F_RDLCK: ::c_short = 01;
pub const F_WRLCK: ::c_short = 02;
pub const F_UNLCK: ::c_short = 03;

pub const O_SYNC: ::c_int = _FSYNC;
pub const O_NONBLOCK: ::c_int = _FNONBLOCK;

pub const IPPROTO_RAW: ::c_int = 255;

pub const _PC_LINK_MAX: ::c_int = 11;
pub const _PC_MAX_CANON: ::c_int = 12;
pub const _PC_MAX_INPUT: ::c_int = 13;
pub const _PC_NAME_MAX: ::c_int = 14;
pub const _PC_PATH_MAX: ::c_int = 16;
pub const _PC_PIPE_BUF: ::c_int = 17;
pub const _PC_NO_TRUNC: ::c_int = 15;
pub const _PC_VDISABLE: ::c_int = 18;
pub const _PC_CHOWN_RESTRICTED: ::c_int = 10;
pub const _PC_ASYNC_IO: ::c_int = 19;
pub const _PC_PRIO_IO: ::c_int = 21;
pub const _PC_SYNC_IO: ::c_int = 20;
pub const _PC_ALLOC_SIZE_MIN: ::c_int = 26;
pub const _PC_REC_INCR_XFER_SIZE: ::c_int = 27;
pub const _PC_REC_MAX_XFER_SIZE: ::c_int = 28;
pub const _PC_REC_MIN_XFER_SIZE: ::c_int = 29;
pub const _PC_REC_XFER_ALIGN: ::c_int = 30;
pub const _PC_SYMLINK_MAX: ::c_int = 25;
pub const _PC_2_SYMLINKS: ::c_int = 31;
pub const _PC_TIMESTAMP_RESOLUTION: ::c_int = 32;
pub const _PC_FILESIZEBITS: ::c_int = 22;
pub const _SC_ARG_MAX: ::c_int = 0;
pub const _SC_CHILD_MAX: ::c_int = 1;
pub const _SC_CLK_TCK: ::c_int = 2;
pub const _SC_NGROUPS_MAX: ::c_int = 3;
pub const _SC_OPEN_MAX: ::c_int = 4;
pub const _SC_JOB_CONTROL: ::c_int = 7;
pub const _SC_SAVED_IDS: ::c_int = 8;
pub const _SC_VERSION: ::c_int = 9;
pub const _SC_PASS_MAX: ::c_int = 45;
pub const _SC_PAGESIZE: ::c_int = _SC_PAGE_SIZE;
pub const _SC_PAGE_SIZE: ::c_int = 48;
pub const _SC_XOPEN_VERSION: ::c_int = 46;
pub const _SC_NPROCESSORS_CONF: ::c_int = 71;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 72;
pub const _SC_STREAM_MAX: ::c_int = 5;
pub const _SC_TZNAME_MAX: ::c_int = 6;
pub const _SC_AIO_LISTIO_MAX: ::c_int = 75;
pub const _SC_AIO_MAX: ::c_int = 76;
pub const _SC_AIO_PRIO_DELTA_MAX: ::c_int = 77;
pub const _SC_ASYNCHRONOUS_IO: ::c_int = 78;
pub const _SC_DELAYTIMER_MAX: ::c_int = 79;
pub const _SC_FSYNC: ::c_int = 80;
pub const _SC_MAPPED_FILES: ::c_int = 84;
pub const _SC_MEMLOCK: ::c_int = 85;
pub const _SC_MEMLOCK_RANGE: ::c_int = 86;
pub const _SC_MEMORY_PROTECTION: ::c_int = 87;
pub const _SC_MESSAGE_PASSING: ::c_int = 88;
pub const _SC_MQ_OPEN_MAX: ::c_int = 89;
pub const _SC_MQ_PRIO_MAX: ::c_int = 90;
pub const _SC_PRIORITIZED_IO: ::c_int = 91;
pub const _SC_PRIORITY_SCHEDULING: ::c_int = 92;
pub const _SC_REALTIME_SIGNALS: ::c_int = 93;
pub const _SC_RTSIG_MAX: ::c_int = 94;
pub const _SC_SEMAPHORES: ::c_int = 95;
pub const _SC_SEM_NSEMS_MAX: ::c_int = 96;
pub const _SC_SEM_VALUE_MAX: ::c_int = 97;
pub const _SC_SHARED_MEMORY_OBJECTS: ::c_int = 98;
pub const _SC_SIGQUEUE_MAX: ::c_int = 99;
pub const _SC_SYNCHRONIZED_IO: ::c_int = 100;
pub const _SC_TIMERS: ::c_int = 102;
pub const _SC_TIMER_MAX: ::c_int = 103;
pub const _SC_2_C_BIND: ::c_int = 51;
pub const _SC_2_C_DEV: ::c_int = 32;
pub const _SC_2_C_VERSION: ::c_int = 52;
pub const _SC_2_FORT_DEV: ::c_int = 33;
pub const _SC_2_FORT_RUN: ::c_int = 34;
pub const _SC_2_LOCALEDEF: ::c_int = 35;
pub const _SC_2_SW_DEV: ::c_int = 36;
pub const _SC_2_UPE: ::c_int = 53;
pub const _SC_2_VERSION: ::c_int = 31;
pub const _SC_BC_BASE_MAX: ::c_int = 23;
pub const _SC_BC_DIM_MAX: ::c_int = 24;
pub const _SC_BC_SCALE_MAX: ::c_int = 25;
pub const _SC_BC_STRING_MAX: ::c_int = 26;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 50;
pub const _SC_EXPR_NEST_MAX: ::c_int = 28;
pub const _SC_LINE_MAX: ::c_int = 29;
pub const _SC_RE_DUP_MAX: ::c_int = 30;
pub const _SC_XOPEN_CRYPT: ::c_int = 56;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 57;
pub const _SC_XOPEN_SHM: ::c_int = 55;
pub const _SC_2_CHAR_TERM: ::c_int = 54;
pub const _SC_XOPEN_XCU_VERSION: ::c_int = 109;
pub const _SC_ATEXIT_MAX: ::c_int = 47;
pub const _SC_IOV_MAX: ::c_int = 58;
pub const _SC_XOPEN_UNIX: ::c_int = 73;
pub const _SC_T_IOV_MAX: ::c_int = 0;
pub const _SC_PHYS_PAGES: ::c_int = 113;
pub const _SC_AVPHYS_PAGES: ::c_int = 114;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 101;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 81;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 82;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 83;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 68;
pub const _SC_THREAD_STACK_MIN: ::c_int = 69;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 70;
pub const _SC_TTY_NAME_MAX: ::c_int = 104;
pub const _SC_THREADS: ::c_int = 60;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 61;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 62;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 64;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 65;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 66;
pub const _SC_THREAD_PROCESS_SHARED: ::c_int = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 59;
pub const _SC_XOPEN_LEGACY: ::c_int = 112;
pub const _SC_XOPEN_REALTIME: ::c_int = 110;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 111;
pub const _SC_XBS5_ILP32_OFF32: ::c_int = 105;
pub const _SC_XBS5_ILP32_OFFBIG: ::c_int = 106;
pub const _SC_XBS5_LP64_OFF64: ::c_int = 107;
pub const _SC_XBS5_LPBIG_OFFBIG: ::c_int = 108;
pub const _SC_2_PBS: ::c_int = 132;
pub const _SC_2_PBS_ACCOUNTING: ::c_int = 133;
pub const _SC_2_PBS_CHECKPOINT: ::c_int = 134;
pub const _SC_2_PBS_LOCATE: ::c_int = 135;
pub const _SC_2_PBS_MESSAGE: ::c_int = 136;
pub const _SC_2_PBS_TRACK: ::c_int = 137;
pub const _SC_ADVISORY_INFO: ::c_int = 130;
pub const _SC_BARRIERS: ::c_int = 138;
pub const _SC_CLOCK_SELECTION: ::c_int = 139;
pub const _SC_CPUTIME: ::c_int = 140;
pub const _SC_HOST_NAME_MAX: ::c_int = 126;
pub const _SC_MONOTONIC_CLOCK: ::c_int = 141;
pub const _SC_READER_WRITER_LOCKS: ::c_int = 142;
pub const _SC_REGEXP: ::c_int = 127;
pub const _SC_SHELL: ::c_int = 128;
pub const _SC_SPAWN: ::c_int = 143;
pub const _SC_SPIN_LOCKS: ::c_int = 144;
pub const _SC_SPORADIC_SERVER: ::c_int = 145;
pub const _SC_SS_REPL_MAX: ::c_int = 156;
pub const _SC_SYMLOOP_MAX: ::c_int = 129;
pub const _SC_THREAD_CPUTIME: ::c_int = 146;
pub const _SC_THREAD_SPORADIC_SERVER: ::c_int = 147;
pub const _SC_TIMEOUTS: ::c_int = 148;
pub const _SC_TRACE: ::c_int = 149;
pub const _SC_TRACE_EVENT_FILTER: ::c_int = 150;
pub const _SC_TRACE_EVENT_NAME_MAX: ::c_int = 157;
pub const _SC_TRACE_INHERIT: ::c_int = 151;
pub const _SC_TRACE_LOG: ::c_int = 152;
pub const _SC_TRACE_NAME_MAX: ::c_int = 158;
pub const _SC_TRACE_SYS_MAX: ::c_int = 159;
pub const _SC_TRACE_USER_EVENT_MAX: ::c_int = 160;
pub const _SC_TYPED_MEMORY_OBJECTS: ::c_int = 153;
pub const _SC_V6_ILP32_OFF32: ::c_int = 121;
pub const _SC_V6_ILP32_OFFBIG: ::c_int = 122;
pub const _SC_V6_LP64_OFF64: ::c_int = 123;
pub const _SC_V6_LPBIG_OFFBIG: ::c_int = 124;
pub const _SC_XOPEN_STREAMS: ::c_int = 125;
pub const _SC_IPV6: ::c_int = 154;
pub const _SC_RAW_SOCKETS: ::c_int = 155;
pub const LOG_CRON: ::c_int = 9<<3;

pub const PTHREAD_MUTEX_NORMAL: ::c_int = 5;
pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 3;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 4;
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __mt_word: [0, 0, 0, 0, 2, 0, 0, 0]
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __cv_word: [0, 0, 0, 0, 2, 0]
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __rw_word: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

pub const RTLD_LAZY: ::c_int = 0x00000004;
pub const RTLD_NOW: ::c_int = 0x00000002;
pub const RTLD_GLOBAL: ::c_int = 0x00010000;
pub const RTLD_LOCAL: ::c_int = 0x00080000;
pub const RTLD_DEFAULT: *mut ::c_void = -1isize as *mut ::c_void;
pub const EMPTY: ::c_short = -1;

pub const RUN_LVL: ::c_short = 1;
pub const BOOT_TIME: ::c_short = 2;
pub const OLD_TIME: ::c_short = 3;
pub const NEW_TIME: ::c_short = 4;
pub const INIT_PROCESS: ::c_short = 5;
pub const LOGIN_PROCESS: ::c_short = 6;
pub const USER_PROCESS: ::c_short = 7;
pub const DEAD_PROCESS: ::c_short = 8;
pub const ACCOUNTING: ::c_short = 9;

pub const TCGETA: ::c_int = TIOC|5;
pub const TCSETA: ::c_int = TIOC|6;
pub const TCSETAW: ::c_int = TIOC|7;
pub const TCSETAF: ::c_int = TIOC|8;
pub const TCSBRK: ::c_int = TIOC|9;
pub const TCXONC: ::c_int = TIOC|11;
pub const TCFLSH: ::c_int = TIOC|12;
pub const TCGETS: ::c_int = TIOC|1;
pub const TCSETS: ::c_int = TIOC|2;
pub const TCSANOW: ::c_int = 0;
pub const TCSETSW: ::c_int = TIOC|3;
pub const TCSADRAIN: ::c_int = 1;
pub const TCSETSF: ::c_int = TIOC|4;
pub const TCSAFLUSH: ::c_int = 2;
pub const TCIFLUSH: ::c_int = 0;
pub const TCOFLUSH: ::c_int = 1;
pub const TCIOFLUSH: ::c_int = 2;
pub const TCOOFF: ::c_int = 0;
pub const TCOON: ::c_int = 1;
pub const TCIOFF: ::c_int = 2;
pub const TCION: ::c_int = 3;
pub const TIOC: ::c_int = ('T' as ::c_int) << 8;
//pub const TIOCGWINSZ: ::c_int = _IOR('t' as ::c_int, 104, struct winsize);
//pub const TIOCSWINSZ: ::c_int = _IOW('t' as ::c_int, 103, struct winsize);
pub const TIOCGETD: ::c_int = _IOR_int('t' as ::c_int, 0);
pub const TIOCSETD: ::c_int = _IOW_int('t' as ::c_int, 1);
pub const TIOCHPCL: ::c_int = _IO('t' as ::c_int, 2);
//pub const TIOCGETP: ::c_int = _IOR('t' as ::c_int, 8,struct sgttyb);
//pub const TIOCSETP: ::c_int = _IOW('t' as ::c_int, 9,struct sgttyb);
//pub const TIOCSETN: ::c_int = _IOW('t' as ::c_int,10,struct sgttyb);
pub const TIOCEXCL: ::c_int = _IO('t' as ::c_int, 13);
pub const TIOCNXCL: ::c_int = _IO('t' as ::c_int, 14);
pub const TIOCFLUSH: ::c_int = _IOW_int('t' as ::c_int, 16);
//pub const TIOCSETC: ::c_int = _IOW('t' as ::c_int,17,struct tchars);
//pub const TIOCGETC: ::c_int = _IOR('t' as ::c_int,18,struct tchars);
pub const TIOCLBIS: ::c_int = _IOW_int('t' as ::c_int, 127);
pub const TIOCLBIC: ::c_int = _IOW_int('t' as ::c_int, 126);
pub const TIOCLSET: ::c_int = _IOW_int('t' as ::c_int, 125);
pub const TIOCLGET: ::c_int = _IOR_int('t' as ::c_int, 124);
pub const TIOCSBRK: ::c_int = _IO('t' as ::c_int, 123);
pub const TIOCCBRK: ::c_int = _IO('t' as ::c_int, 122);
pub const TIOCSDTR: ::c_int = _IO('t' as ::c_int, 121);
pub const TIOCCDTR: ::c_int = _IO('t' as ::c_int, 120);
//pub const TIOCSLTC: ::c_int = _IOW('t' as ::c_int,117,struct ltchars);
//pub const TIOCGLTC: ::c_int = _IOR('t' as ::c_int,116,struct ltchars);
pub const TIOCOUTQ: ::c_int = _IOR_int('t' as ::c_int, 115);
pub const TIOCNOTTY: ::c_int = _IO('t' as ::c_int, 113);
pub const TIOCSTOP: ::c_int = _IO('t' as ::c_int, 111);
pub const TIOCSTART: ::c_int = _IO('t' as ::c_int, 110);
pub const TIOCGPGRP: ::c_int = _IOR_int('t' as ::c_int, 119);
pub const TIOCSPGRP: ::c_int = _IOW_int('t' as ::c_int, 118);
pub const TIOCGSID: ::c_int = _IOR_int('t' as ::c_int, 72);
//pub const TIOCSTI: ::c_int = _IOW('t' as ::c_int, 114, char);
pub const TIOCMSET: ::c_int = _IOW_int('t' as ::c_int, 109);
pub const TIOCMBIS: ::c_int = _IOW_int('t' as ::c_int, 108);
pub const TIOCMBIC: ::c_int = _IOW_int('t' as ::c_int, 107);
pub const TIOCMGET: ::c_int = _IOR_int('t' as ::c_int, 106);
pub const TIOCREMOTE: ::c_int = _IOW_int('t' as ::c_int, 105);
pub const CSIZE: ::tcflag_t = 0x00000030;
pub const CS5: ::tcflag_t = 0x00000000;
pub const CS6: ::tcflag_t = 0x00000010;
pub const CS7: ::tcflag_t = 0x00000020;
pub const CS8: ::tcflag_t = 0x00000030;
pub const CSTOPB: ::tcflag_t = 0x00000040;
pub const ECHO: ::tcflag_t = 0x20000;
pub const ECHOE: ::tcflag_t = 0x00000010;
pub const ECHOK: ::tcflag_t = 0x00000020;
pub const ECHONL: ::tcflag_t = 0x00000040;
pub const ECHOCTL: ::tcflag_t = 0x00020000;
pub const ECHOPRT: ::tcflag_t = 0x00040000;
pub const ECHOKE: ::tcflag_t = 0x00080000;
pub const IGNBRK: ::tcflag_t = 0x00000001;
pub const BRKINT: ::tcflag_t = 0x00000002;
pub const IGNPAR: ::tcflag_t = 0x00000004;
pub const PARMRK: ::tcflag_t = 0x00000008;
pub const INPCK: ::tcflag_t = 0x00000010;
pub const ISTRIP: ::tcflag_t = 0x00000020;
pub const INLCR: ::tcflag_t = 0x00000040;
pub const IGNCR: ::tcflag_t = 0x00000080;
pub const ICRNL: ::tcflag_t = 0x00000100;
pub const IXON: ::tcflag_t = 0x0001;
pub const IXOFF: ::tcflag_t = 0x00000400;
pub const IXANY: ::tcflag_t = 0x00001000;
pub const IMAXBEL: ::tcflag_t = 0x00010000;
pub const OPOST: ::tcflag_t = 0x00000001;
pub const ONLCR: ::tcflag_t = 0x00000004;
pub const OCRNL: ::tcflag_t = 0x00000008;
pub const ONOCR: ::tcflag_t = 0x00000010;
pub const ONLRET: ::tcflag_t = 0x00000020;
pub const CREAD: ::tcflag_t = 0x00000080;
pub const PARENB: ::tcflag_t = 0x00000100;
pub const PARODD: ::tcflag_t = 0x00000200;
pub const HUPCL: ::tcflag_t = 0x00000400;
pub const CLOCAL: ::tcflag_t = 0x00000800;
pub const ISIG: ::tcflag_t = 0x00000001;
pub const ICANON: ::tcflag_t = 0x00000002;
pub const IEXTEN: ::tcflag_t = 0x00200000;
pub const TOSTOP: ::tcflag_t = 0x00010000;
pub const FLUSHO: ::tcflag_t = 0x00100000;
pub const PENDIN: ::tcflag_t = 0x20000000;
pub const NOFLSH: ::tcflag_t = 0x00000080;
pub const I_NREAD: ::c_int = _IO('S' as ::c_int,1);
pub const I_PUSH: ::c_int = _IO('S' as ::c_int,2);
pub const I_POP: ::c_int = _IO('S' as ::c_int,3);
pub const I_LOOK: ::c_int = _IO('S' as ::c_int,4);
pub const I_FLUSH: ::c_int = _IO('S' as ::c_int,5);
pub const I_SRDOPT: ::c_int = _IO('S' as ::c_int,6);
pub const I_GRDOPT: ::c_int = _IO('S' as ::c_int,7);
pub const I_STR: ::c_int = _IO('S' as ::c_int,8);
pub const I_SETSIG: ::c_int = _IO('S' as ::c_int,9);
pub const I_GETSIG: ::c_int = _IO('S' as ::c_int,10);
pub const I_FIND: ::c_int = _IO('S' as ::c_int,11);
pub const I_LINK: ::c_int = _IO('S' as ::c_int,12);
pub const I_UNLINK: ::c_int = _IO('S' as ::c_int,13);
pub const I_PEEK: ::c_int = _IO('S' as ::c_int,15);
pub const I_FDINSERT: ::c_int = _IO('S' as ::c_int,16);
pub const I_SENDFD: ::c_int = _IO('S' as ::c_int,17);
pub const I_RECVFD: ::c_int = _IO('S' as ::c_int,18);
pub const I_SWROPT: ::c_int = _IO('S' as ::c_int,20);
pub const I_GWROPT: ::c_int = _IO('S' as ::c_int,21);
pub const I_LIST: ::c_int = _IO('S' as ::c_int,22);
pub const I_PLINK: ::c_int = _IO('S' as ::c_int,29);
pub const I_PUNLINK: ::c_int = _IO('S' as ::c_int,30);
pub const I_FLUSHBAND: ::c_int = _IO('S' as ::c_int,19);
pub const I_CKBAND: ::c_int = _IO('S' as ::c_int,24);
pub const I_GETBAND: ::c_int = _IO('S' as ::c_int,25);
pub const I_ATMARK: ::c_int = _IO('S' as ::c_int,23);
pub const I_SETCLTIME: ::c_int = _IO('S' as ::c_int,27);
pub const I_GETCLTIME: ::c_int = _IO('S' as ::c_int,28);
pub const I_CANPUT: ::c_int = _IO('S' as ::c_int,26);
pub const PRIO_PROCESS: ::c_int = 0;
pub const PRIO_PGRP: ::c_int = 1;
pub const PRIO_USER: ::c_int = 2;

safe_f! {
    pub {const} fn WIFSTOPPED(status: ::c_int) -> bool {
        (status & _W_STOPPED) != 0
    }

    pub {const} fn WSTOPSIG(status: ::c_int) -> ::c_int {
        if WIFSTOPPED(status) {
            (((status as ::c_uint) >> 8) & 0xff) as ::c_int
        } else {
            -1
        }
    }

    pub {const} fn WIFEXITED(status: ::c_int) -> bool {
        (status & 0xFF) == 0
    }

    pub {const} fn WEXITSTATUS(status: ::c_int) -> ::c_int {
        if WIFEXITED(status) {
            (((status as ::c_uint) >> 8) & 0xff) as ::c_int
        } else {
            -1
        }
    }

    pub {const} fn WIFSIGNALED(status: ::c_int) -> bool {
        !WIFEXITED(status) && !WIFSTOPPED(status)
    }

    pub {const} fn WTERMSIG(status: ::c_int) -> ::c_int {
        if WIFSIGNALED(status) {
            (((status as ::c_uint) >> 16) & 0xff) as ::c_int
        } else {
            -1
        }
    }

    pub {const} fn WIFCONTINUED(status: ::c_int) -> bool {
        (status & WCONTINUED) != 0
    }

    pub {const} fn WCOREDUMP(status: ::c_int) -> bool {
        status & 0x80 == 0x80
    }
}

extern "C" {
    pub fn clock_gettime(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;

    pub fn recvfrom(
        socket: ::c_int,
        buf: *mut ::c_void,
        len: ::size_t,
        flags: ::c_int,
        addr: *mut ::sockaddr,
        addrlen: *mut ::socklen_t,
    ) -> ::ssize_t;

    pub fn strerror_r(
        errnum: ::c_int,
        buf: *mut c_char,
        buflen: ::size_t,
    ) -> ::c_int;

    pub fn errno() -> *mut ::c_int;

    pub fn bind(
        socket: ::c_int,
        address: *const ::sockaddr,
        address_len: ::socklen_t,
    ) -> ::c_int;

    pub fn writev(
        fd: ::c_int,
        iov: *const ::iovec,
        iovcnt: ::c_int,
    ) -> ::ssize_t;
    pub fn readv(
        fd: ::c_int,
        iov: *const ::iovec,
        iovcnt: ::c_int,
    ) -> ::ssize_t;

    pub fn pthread_create(
        native: *mut ::pthread_t,
        attr: *const ::pthread_attr_t,
        f: extern "C" fn(*mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;
    pub fn pthread_sigmask(
        how: ::c_int,
        set: *const sigset_t,
        oldset: *mut sigset_t,
    ) -> ::c_int;
    pub fn pthread_condattr_setclock(
        attr: *mut pthread_condattr_t,
        clock_id: ::clockid_t,
    ) -> ::c_int;

    pub fn setgroups(ngroups: ::c_int, ptr: *const ::gid_t) -> ::c_int;
    pub fn ioctl(fildes: ::c_int, request: ::c_int, ...) -> ::c_int;
    pub fn getpwuid_r(
        uid: ::uid_t,
        pwd: *mut passwd,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;
}
