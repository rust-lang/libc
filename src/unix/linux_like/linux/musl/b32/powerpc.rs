pub type c_char = u8;
pub type wchar_t = i32;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __st_rdev_padding: ::c_short,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_long; 2],
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __st_rdev_padding: ::c_short,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_long; 2],
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: ::size_t
    }

    pub struct ipc_perm {
        pub __ipc_perm_key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::mode_t,
        pub __seq: ::c_int,
        __pad1: ::c_int,
        __pad2: ::c_longlong,
        __pad3: ::c_longlong
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        __unused1: ::c_int,
        pub shm_atime: ::time_t,
        __unused2: ::c_int,
        pub shm_dtime: ::time_t,
        __unused3: ::c_int,
        pub shm_ctime: ::time_t,
        __unused4: ::c_int,
        pub shm_segsz: ::size_t,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::c_ulong,
        __pad1: ::c_ulong,
        __pad2: ::c_ulong,
    }

    pub struct msqid_ds {
        pub msg_perm: ::ipc_perm,
        __unused1: ::c_int,
        pub msg_stime: ::time_t,
        __unused2: ::c_int,
        pub msg_rtime: ::time_t,
        __unused3: ::c_int,
        pub msg_ctime: ::time_t,
        __msg_cbytes: ::c_ulong,
        pub msg_qnum: ::msgqnum_t,
        pub msg_qbytes: ::msglen_t,
        pub msg_lspid: ::pid_t,
        pub msg_lrpid: ::pid_t,
        __pad1: ::c_ulong,
        __pad2: ::c_ulong,
    }

    pub struct statfs {
        pub f_type: ::c_ulong,
        pub f_bsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_flags: ::c_ulong,
        pub f_spare: [::c_ulong; 4],
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub _pad: [::c_int; 29],
        _align: [usize; 0],
    }

    pub struct statfs64 {
        pub f_type: ::c_ulong,
        pub f_bsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_flags: ::c_ulong,
        pub f_spare: [::c_ulong; 4],
    }

    pub struct statvfs64 {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_bavail: u64,
        pub f_files: u64,
        pub f_ffree: u64,
        pub f_favail: u64,
        #[cfg(target_endian = "little")]
        pub f_fsid: ::c_ulong,
        __f_unused: ::c_int,
        #[cfg(target_endian = "big")]
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }
}

pub const MADV_SOFT_OFFLINE: ::c_int = 101;
pub const SIGSTKSZ: ::size_t = 10240;
pub const MINSIGSTKSZ: ::size_t = 4096;

pub const O_DIRECT: ::c_int = 0x20000;
pub const O_DIRECTORY: ::c_int = 0x4000;
pub const O_NOFOLLOW: ::c_int = 0x8000;
pub const O_ASYNC: ::c_int = 0x2000;
pub const O_LARGEFILE: ::c_int = 0x10000;

pub const FIOCLEX: ::c_int = 0x20006601;
pub const FIONCLEX: ::c_int = 0x20006602;
pub const FIONBIO: ::c_int = 0x8004667E;

pub const RLIMIT_RSS: ::c_int = 5;
pub const RLIMIT_NOFILE: ::c_int = 7;
pub const RLIMIT_AS: ::c_int = 9;
pub const RLIMIT_NPROC: ::c_int = 6;
pub const RLIMIT_MEMLOCK: ::c_int = 8;
pub const RLIMIT_NLIMITS: ::c_int = 15;

pub const MCL_CURRENT: ::c_int = 0x2000;
pub const MCL_FUTURE: ::c_int = 0x4000;
pub const CBAUD: ::tcflag_t = 0o0000377;
pub const TAB1: ::c_int = 0x00000400;
pub const TAB2: ::c_int = 0x00000800;
pub const TAB3: ::c_int = 0x00000C00;
pub const CR1: ::c_int = 0x00001000;
pub const CR2: ::c_int = 0x00002000;
pub const CR3: ::c_int = 0x00003000;
pub const FF1: ::c_int = 0x00004000;
pub const BS1: ::c_int = 0x00008000;
pub const VT1: ::c_int = 0x00010000;
pub const VWERASE: usize = 10;
pub const VREPRINT: usize = 11;
pub const VSUSP: usize = 12;
pub const VSTART: usize = 13;
pub const VSTOP: usize = 14;
pub const VDISCARD: usize = 16;
pub const VTIME: usize = 7;
pub const IXON: ::tcflag_t = 0x00000200;
pub const IXOFF: ::tcflag_t = 0x00000400;
pub const ONLCR: ::tcflag_t = 0x00000002;
pub const CSIZE: ::tcflag_t = 0x00000300;
pub const CS6: ::tcflag_t = 0x00000100;
pub const CS7: ::tcflag_t = 0x00000200;
pub const CS8: ::tcflag_t = 0x00000300;
pub const CSTOPB: ::tcflag_t = 0x00000400;
pub const CREAD: ::tcflag_t = 0x00000800;
pub const PARENB: ::tcflag_t = 0x00001000;
pub const PARODD: ::tcflag_t = 0x00002000;
pub const HUPCL: ::tcflag_t = 0x00004000;
pub const CLOCAL: ::tcflag_t = 0x00008000;
pub const ECHOKE: ::tcflag_t = 0x00000001;
pub const ECHOE: ::tcflag_t = 0x00000002;
pub const ECHOK: ::tcflag_t = 0x00000004;
pub const ECHONL: ::tcflag_t = 0x00000010;
pub const ECHOPRT: ::tcflag_t = 0x00000020;
pub const ECHOCTL: ::tcflag_t = 0x00000040;
pub const ISIG: ::tcflag_t = 0x00000080;
pub const ICANON: ::tcflag_t = 0x00000100;
pub const PENDIN: ::tcflag_t = 0x20000000;
pub const NOFLSH: ::tcflag_t = 0x80000000;
pub const CIBAUD: ::tcflag_t = 0o00077600000;
pub const CBAUDEX: ::tcflag_t = 0o000020;
pub const VSWTC: usize = 9;
pub const OLCUC: ::tcflag_t = 0o000004;
pub const NLDLY: ::tcflag_t = 0o001400;
pub const CRDLY: ::tcflag_t = 0o030000;
pub const TABDLY: ::tcflag_t = 0o006000;
pub const BSDLY: ::tcflag_t = 0o100000;
pub const FFDLY: ::tcflag_t = 0o040000;
pub const VTDLY: ::tcflag_t = 0o200000;
pub const XTABS: ::tcflag_t = 0o006000;
pub const B57600: ::speed_t = 0o000020;
pub const B115200: ::speed_t = 0o000021;
pub const B230400: ::speed_t = 0o000022;
pub const B460800: ::speed_t = 0o000023;
pub const B500000: ::speed_t = 0o000024;
pub const B576000: ::speed_t = 0o000025;
pub const B921600: ::speed_t = 0o000026;
pub const B1000000: ::speed_t = 0o000027;
pub const B1152000: ::speed_t = 0o000030;
pub const B1500000: ::speed_t = 0o000031;
pub const B2000000: ::speed_t = 0o000032;
pub const B2500000: ::speed_t = 0o000033;
pub const B3000000: ::speed_t = 0o000034;
pub const B3500000: ::speed_t = 0o000035;
pub const B4000000: ::speed_t = 0o000036;

pub const O_APPEND: ::c_int = 1024;
pub const O_CREAT: ::c_int = 64;
pub const O_EXCL: ::c_int = 128;
pub const O_NOCTTY: ::c_int = 256;
pub const O_NONBLOCK: ::c_int = 2048;
pub const O_SYNC: ::c_int = 1052672;
pub const O_RSYNC: ::c_int = 1052672;
pub const O_DSYNC: ::c_int = 4096;

pub const SOCK_NONBLOCK: ::c_int = 2048;

pub const MAP_ANON: ::c_int = 0x0020;
pub const MAP_GROWSDOWN: ::c_int = 0x0100;
pub const MAP_DENYWRITE: ::c_int = 0x0800;
pub const MAP_EXECUTABLE: ::c_int = 0x01000;
pub const MAP_LOCKED: ::c_int = 0x00080;
pub const MAP_NORESERVE: ::c_int = 0x00040;
pub const MAP_POPULATE: ::c_int = 0x08000;
pub const MAP_NONBLOCK: ::c_int = 0x010000;
pub const MAP_STACK: ::c_int = 0x020000;

pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_SEQPACKET: ::c_int = 5;

pub const SOL_SOCKET: ::c_int = 1;

pub const EDEADLK: ::c_int = 35;
pub const ENAMETOOLONG: ::c_int = 36;
pub const ENOLCK: ::c_int = 37;
pub const ENOSYS: ::c_int = 38;
pub const ENOTEMPTY: ::c_int = 39;
pub const ELOOP: ::c_int = 40;
pub const ENOMSG: ::c_int = 42;
pub const EIDRM: ::c_int = 43;
pub const ECHRNG: ::c_int = 44;
pub const EL2NSYNC: ::c_int = 45;
pub const EL3HLT: ::c_int = 46;
pub const EL3RST: ::c_int = 47;
pub const ELNRNG: ::c_int = 48;
pub const EUNATCH: ::c_int = 49;
pub const ENOCSI: ::c_int = 50;
pub const EL2HLT: ::c_int = 51;
pub const EBADE: ::c_int = 52;
pub const EBADR: ::c_int = 53;
pub const EXFULL: ::c_int = 54;
pub const ENOANO: ::c_int = 55;
pub const EBADRQC: ::c_int = 56;
pub const EBADSLT: ::c_int = 57;
pub const EDEADLOCK: ::c_int = 58;
pub const EMULTIHOP: ::c_int = 72;
pub const EBADMSG: ::c_int = 74;
pub const EOVERFLOW: ::c_int = 75;
pub const ENOTUNIQ: ::c_int = 76;
pub const EBADFD: ::c_int = 77;
pub const EREMCHG: ::c_int = 78;
pub const ELIBACC: ::c_int = 79;
pub const ELIBBAD: ::c_int = 80;
pub const ELIBSCN: ::c_int = 81;
pub const ELIBMAX: ::c_int = 82;
pub const ELIBEXEC: ::c_int = 83;
pub const EILSEQ: ::c_int = 84;
pub const ERESTART: ::c_int = 85;
pub const ESTRPIPE: ::c_int = 86;
pub const EUSERS: ::c_int = 87;
pub const ENOTSOCK: ::c_int = 88;
pub const EDESTADDRREQ: ::c_int = 89;
pub const EMSGSIZE: ::c_int = 90;
pub const EPROTOTYPE: ::c_int = 91;
pub const ENOPROTOOPT: ::c_int = 92;
pub const EPROTONOSUPPORT: ::c_int = 93;
pub const ESOCKTNOSUPPORT: ::c_int = 94;
pub const EOPNOTSUPP: ::c_int = 95;
pub const ENOTSUP: ::c_int = EOPNOTSUPP;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const EAFNOSUPPORT: ::c_int = 97;
pub const EADDRINUSE: ::c_int = 98;
pub const EADDRNOTAVAIL: ::c_int = 99;
pub const ENETDOWN: ::c_int = 100;
pub const ENETUNREACH: ::c_int = 101;
pub const ENETRESET: ::c_int = 102;
pub const ECONNABORTED: ::c_int = 103;
pub const ECONNRESET: ::c_int = 104;
pub const ENOBUFS: ::c_int = 105;
pub const EISCONN: ::c_int = 106;
pub const ENOTCONN: ::c_int = 107;
pub const ESHUTDOWN: ::c_int = 108;
pub const ETOOMANYREFS: ::c_int = 109;
pub const ETIMEDOUT: ::c_int = 110;
pub const ECONNREFUSED: ::c_int = 111;
pub const EHOSTDOWN: ::c_int = 112;
pub const EHOSTUNREACH: ::c_int = 113;
pub const EALREADY: ::c_int = 114;
pub const EINPROGRESS: ::c_int = 115;
pub const ESTALE: ::c_int = 116;
pub const EUCLEAN: ::c_int = 117;
pub const ENOTNAM: ::c_int = 118;
pub const ENAVAIL: ::c_int = 119;
pub const EISNAM: ::c_int = 120;
pub const EREMOTEIO: ::c_int = 121;
pub const EDQUOT: ::c_int = 122;
pub const ENOMEDIUM: ::c_int = 123;
pub const EMEDIUMTYPE: ::c_int = 124;
pub const ECANCELED: ::c_int = 125;
pub const ENOKEY: ::c_int = 126;
pub const EKEYEXPIRED: ::c_int = 127;
pub const EKEYREVOKED: ::c_int = 128;
pub const EKEYREJECTED: ::c_int = 129;
pub const EOWNERDEAD: ::c_int = 130;
pub const ENOTRECOVERABLE: ::c_int = 131;
pub const ERFKILL: ::c_int = 132;
pub const EHWPOISON: ::c_int = 133;

pub const SO_REUSEADDR: ::c_int = 2;
pub const SO_TYPE: ::c_int = 3;
pub const SO_ERROR: ::c_int = 4;
pub const SO_DONTROUTE: ::c_int = 5;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_SNDBUF: ::c_int = 7;
pub const SO_RCVBUF: ::c_int = 8;
pub const SO_KEEPALIVE: ::c_int = 9;
pub const SO_OOBINLINE: ::c_int = 10;
pub const SO_NO_CHECK: ::c_int = 11;
pub const SO_PRIORITY: ::c_int = 12;
pub const SO_LINGER: ::c_int = 13;
pub const SO_BSDCOMPAT: ::c_int = 14;
pub const SO_REUSEPORT: ::c_int = 15;
pub const SO_RCVLOWAT: ::c_int = 16;
pub const SO_SNDLOWAT: ::c_int = 17;
pub const SO_RCVTIMEO: ::c_int = 18;
pub const SO_SNDTIMEO: ::c_int = 19;
pub const SO_PASSCRED: ::c_int = 20;
pub const SO_PEERCRED: ::c_int = 21;
pub const SO_ACCEPTCONN: ::c_int = 30;
pub const SO_SNDBUFFORCE: ::c_int = 32;
pub const SO_RCVBUFFORCE: ::c_int = 33;
pub const SO_PROTOCOL: ::c_int = 38;
pub const SO_DOMAIN: ::c_int = 39;

pub const SA_ONSTACK: ::c_int = 0x08000000;
pub const SA_SIGINFO: ::c_int = 0x00000004;
pub const SA_NOCLDWAIT: ::c_int = 0x00000002;

pub const SIGCHLD: ::c_int = 17;
pub const SIGBUS: ::c_int = 7;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF: ::c_int = 27;
pub const SIGWINCH: ::c_int = 28;
pub const SIGUSR1: ::c_int = 10;
pub const SIGUSR2: ::c_int = 12;
pub const SIGCONT: ::c_int = 18;
pub const SIGSTOP: ::c_int = 19;
pub const SIGTSTP: ::c_int = 20;
pub const SIGURG: ::c_int = 23;
pub const SIGIO: ::c_int = 29;
pub const SIGSYS: ::c_int = 31;
pub const SIGSTKFLT: ::c_int = 16;
pub const SIGPOLL: ::c_int = 29;
pub const SIGPWR: ::c_int = 30;
pub const SIG_SETMASK: ::c_int = 2;
pub const SIG_BLOCK: ::c_int = 0x000000;
pub const SIG_UNBLOCK: ::c_int = 0x01;

pub const EXTPROC: ::tcflag_t = 0x10000000;

pub const MAP_HUGETLB: ::c_int = 0x040000;

pub const F_GETLK: ::c_int = 12;
pub const F_GETOWN: ::c_int = 9;
pub const F_SETLK: ::c_int = 13;
pub const F_SETLKW: ::c_int = 14;
pub const F_SETOWN: ::c_int = 8;
pub const F_OFD_GETLK: ::c_int = 36;
pub const F_OFD_SETLK: ::c_int = 37;
pub const F_OFD_SETLKW: ::c_int = 38;

pub const VEOF: usize = 4;
pub const VEOL: usize = 6;
pub const VEOL2: usize = 8;
pub const VMIN: usize = 5;
pub const IEXTEN: ::tcflag_t = 0x00000400;
pub const TOSTOP: ::tcflag_t = 0x00400000;
pub const FLUSHO: ::tcflag_t = 0x00800000;

pub const TCGETS: ::c_int = 0x402C7413;
pub const TCSETS: ::c_int = 0x802C7414;
pub const TCSETSW: ::c_int = 0x802C7415;
pub const TCSETSF: ::c_int = 0x802C7416;
pub const TCGETA: ::c_int = 0x40147417;
pub const TCSETA: ::c_int = 0x80147418;
pub const TCSETAW: ::c_int = 0x80147419;
pub const TCSETAF: ::c_int = 0x8014741C;
pub const TCSBRK: ::c_int = 0x2000741D;
pub const TCXONC: ::c_int = 0x2000741E;
pub const TCFLSH: ::c_int = 0x2000741F;
pub const TIOCGSOFTCAR: ::c_int = 0x5419;
pub const TIOCSSOFTCAR: ::c_int = 0x541A;
pub const TIOCLINUX: ::c_int = 0x541C;
pub const TIOCGSERIAL: ::c_int = 0x541E;
pub const TIOCEXCL: ::c_int = 0x540C;
pub const TIOCNXCL: ::c_int = 0x540D;
pub const TIOCSCTTY: ::c_int = 0x540E;
pub const TIOCGPGRP: ::c_int = 0x40047477;
pub const TIOCSPGRP: ::c_int = 0x80047476;
pub const TIOCOUTQ: ::c_int = 0x40047473;
pub const TIOCSTI: ::c_int = 0x5412;
pub const TIOCGWINSZ: ::c_int = 0x40087468;
pub const TIOCSWINSZ: ::c_int = 0x80087467;
pub const TIOCMGET: ::c_int = 0x5415;
pub const TIOCMBIS: ::c_int = 0x5416;
pub const TIOCMBIC: ::c_int = 0x5417;
pub const TIOCMSET: ::c_int = 0x5418;
pub const FIONREAD: ::c_int = 0x4004667F;
pub const TIOCCONS: ::c_int = 0x541D;

pub const TIOCGRS485: ::c_int = 0x542e;
pub const TIOCSRS485: ::c_int = 0x542f;

pub const POLLWRNORM: ::c_short = 0x100;
pub const POLLWRBAND: ::c_short = 0x200;

pub const TIOCM_LE: ::c_int = 0x001;
pub const TIOCM_DTR: ::c_int = 0x002;
pub const TIOCM_RTS: ::c_int = 0x004;
pub const TIOCM_ST: ::c_int = 0x008;
pub const TIOCM_SR: ::c_int = 0x010;
pub const TIOCM_CTS: ::c_int = 0x020;
pub const TIOCM_CAR: ::c_int = 0x040;
pub const TIOCM_RNG: ::c_int = 0x080;
pub const TIOCM_DSR: ::c_int = 0x100;
pub const TIOCM_CD: ::c_int = TIOCM_CAR;
pub const TIOCM_RI: ::c_int = TIOCM_RNG;

extern "C" {
    pub fn getrandom(
        buf: *mut ::c_void,
        buflen: ::size_t,
        flags: ::c_uint,
    ) -> ::ssize_t;
}
