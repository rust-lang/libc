pub type c_char = u8;
pub type wchar_t = u32;
pub type stat64 = ::stat;

s! {

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::c_ulonglong,
        pub st_mode: ::c_uint,
        pub st_nlink: ::c_uint,
        pub st_uid: ::c_uint,
        pub st_gid: ::c_uint,
        pub st_rdev: ::c_ulonglong,
        __st_rdev_padding: ::c_ulong,
        pub st_size: ::c_longlong,
        pub st_blksize: ::blksize_t,
        __st_blksize_padding: ::c_int,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,

        __unused: [::c_int;2],
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
        pub __seq: ::c_ushort,
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

    pub struct msqid_ds {
        pub msg_perm: ::ipc_perm,
        pub msg_stime: ::time_t,
        __unused1: ::c_int,
        pub msg_rtime: ::time_t,
        __unused2: ::c_int,
        pub msg_ctime: ::time_t,
        __unused3: ::c_int,
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
        pub f_fsid: ::c_ulong,
        __f_unused: ::c_int,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct termios2 {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; 19],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }
}

pub const AF_FILE: ::c_int = 1;
pub const AF_KCM: ::c_int = 41;
pub const AF_MAX: ::c_int = 43;
pub const AF_QIPCRTR: ::c_int = 42;
pub const EADDRINUSE: ::c_int = 98;
pub const EADDRNOTAVAIL: ::c_int = 99;
pub const EAFNOSUPPORT: ::c_int = 97;
pub const EALREADY: ::c_int = 114;
pub const EBADE: ::c_int = 52;
pub const EBADMSG: ::c_int = 74;
pub const EBADR: ::c_int = 53;
pub const EBADRQC: ::c_int = 56;
pub const EBADSLT: ::c_int = 57;
pub const ECANCELED: ::c_int = 125;
pub const ECHRNG: ::c_int = 44;
pub const ECONNABORTED: ::c_int = 103;
pub const ECONNREFUSED: ::c_int = 111;
pub const ECONNRESET: ::c_int = 104;
pub const EDEADLK: ::c_int = 35;
pub const EDEADLOCK: ::c_int = 35;
pub const EDESTADDRREQ: ::c_int = 89;
pub const EDQUOT: ::c_int = 122;
pub const EHOSTDOWN: ::c_int = 112;
pub const EHOSTUNREACH: ::c_int = 113;
pub const EHWPOISON: ::c_int = 133;
pub const EIDRM: ::c_int = 43;
pub const EILSEQ: ::c_int = 84;
pub const EINPROGRESS: ::c_int = 115;
pub const EISCONN: ::c_int = 106;
pub const EISNAM: ::c_int = 120;
pub const EKEYEXPIRED: ::c_int = 127;
pub const EKEYREJECTED: ::c_int = 129;
pub const EKEYREVOKED: ::c_int = 128;
pub const EL2HLT: ::c_int = 51;
pub const EL2NSYNC: ::c_int = 45;
pub const EL3HLT: ::c_int = 46;
pub const EL3RST: ::c_int = 47;
pub const ELIBACC: ::c_int = 79;
pub const ELIBBAD: ::c_int = 80;
pub const ELIBEXEC: ::c_int = 83;
pub const ELIBMAX: ::c_int = 82;
pub const ELIBSCN: ::c_int = 81;
pub const ELNRNG: ::c_int = 48;
pub const ELOOP: ::c_int = 40;
pub const EMEDIUMTYPE: ::c_int = 124;
pub const EMSGSIZE: ::c_int = 90;
pub const EMULTIHOP: ::c_int = 72;
pub const ENAMETOOLONG: ::c_int = 36;
pub const ENAVAIL: ::c_int = 119;
pub const ENETDOWN: ::c_int = 100;
pub const ENETRESET: ::c_int = 102;
pub const ENETUNREACH: ::c_int = 101;
pub const ENOANO: ::c_int = 55;
pub const ENOBUFS: ::c_int = 105;
pub const ENOCSI: ::c_int = 50;
pub const ENOKEY: ::c_int = 126;
pub const ENOLCK: ::c_int = 37;
pub const ENOMEDIUM: ::c_int = 123;
pub const ENOMSG: ::c_int = 42;
pub const ENOPROTOOPT: ::c_int = 92;
pub const ENOSYS: ::c_int = 38;
pub const ENOTCONN: ::c_int = 107;
pub const ENOTEMPTY: ::c_int = 39;
pub const ENOTNAM: ::c_int = 118;
pub const ENOTRECOVERABLE: ::c_int = 131;
pub const ENOTSOCK: ::c_int = 88;
pub const ENOTSUP: ::c_int = 95;
pub const ENOTUNIQ: ::c_int = 76;
pub const EOPNOTSUPP: ::c_int = 95;
pub const EOVERFLOW: ::c_int = 75;
pub const EOWNERDEAD: ::c_int = 130;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const EREMCHG: ::c_int = 78;
pub const ERESTART: ::c_int = 85;
pub const ERFKILL: ::c_int = 132;
pub const ESHUTDOWN: ::c_int = 108;
pub const ESOCKTNOSUPPORT: ::c_int = 94;
pub const ESTALE: ::c_int = 116;
pub const ESTRPIPE: ::c_int = 86;
pub const ETOOMANYREFS: ::c_int = 109;
pub const EUCLEAN: ::c_int = 117;
pub const EUNATCH: ::c_int = 49;
pub const EUSERS: ::c_int = 87;
pub const EXFULL: ::c_int = 54;
pub const EXTPROC: ::c_int = 65536;
pub const F_EXLCK: ::c_int = 4;
pub const F_GETLK: ::c_int = 12;
pub const F_GETOWN: ::c_int = 9;
pub const F_GETOWNER_UIDS: ::c_int = 17;
pub const F_GETOWN_EX: ::c_int = 16;
pub const F_GETSIG: ::c_int = 11;
pub const FIOASYNC: ::c_int = 21586;
pub const FIOCLEX: ::c_int = 21585;
pub const FIONBIO: ::c_int = 21537;
pub const FIONCLEX: ::c_int = 21584;
pub const FIONREAD: ::c_int = 21531;
pub const FIOQSIZE: ::c_int = 21600;
pub const F_LINUX_SPECIFIC_BASE: ::c_int = 1024;
pub const FLUSHO: ::c_int = 4096;
pub const F_OFD_GETLK: ::c_int = 36;
pub const F_OFD_SETLK: ::c_int = 37;
pub const F_OFD_SETLKW: ::c_int = 38;
pub const F_OWNER_PGRP: ::c_int = 2;
pub const F_OWNER_PID: ::c_int = 1;
pub const F_OWNER_TID: ::c_int = 0;
pub const F_SETLK: ::c_int = 13;
pub const F_SETLKW: ::c_int = 14;
pub const F_SETOWN: ::c_int = 8;
pub const F_SETOWN_EX: ::c_int = 15;
pub const F_SETSIG: ::c_int = 10;
pub const F_SHLCK: ::c_int = 8;
pub const IEXTEN: ::c_int = 32768;
pub const MAP_ANON: ::c_int = 32;
pub const MAP_DENYWRITE: ::c_int = 2048;
pub const MAP_EXECUTABLE: ::c_int = 4096;
pub const MAP_GROWSDOWN: ::c_int = 256;
pub const MAP_HUGETLB: ::c_int = 262144;
pub const MAP_LOCKED: ::c_int = 8192;
pub const MAP_NONBLOCK: ::c_int = 65536;
pub const MAP_NORESERVE: ::c_int = 16384;
pub const MAP_POPULATE: ::c_int = 32768;
pub const MAP_STACK: ::c_int = 131072;
pub const MAP_UNINITIALIZED: ::c_int = 0;
pub const O_APPEND: ::c_int = 1024;
pub const O_ASYNC: ::c_int = 8192;
pub const O_CREAT: ::c_int = 64;
pub const O_DIRECT: ::c_int = 16384;
pub const O_DIRECTORY: ::c_int = 65536;
pub const O_DSYNC: ::c_int = 4096;
pub const O_EXCL: ::c_int = 128;
pub const O_LARGEFILE: ::c_int = 32768;
pub const O_NOCTTY: ::c_int = 256;
pub const O_NOFOLLOW: ::c_int = 131072;
pub const O_NONBLOCK: ::c_int = 2048;
pub const O_SYNC: ::c_int = 1052672;
pub const PF_FILE: ::c_int = 1;
pub const PF_KCM: ::c_int = 41;
pub const PF_MAX: ::c_int = 43;
pub const PF_QIPCRTR: ::c_int = 42;
pub const RLIMIT_AS: ::c_int = 9;
pub const RLIMIT_MEMLOCK: ::c_int = 8;
pub const RLIMIT_NOFILE: ::c_int = 7;
pub const RLIMIT_NPROC: ::c_int = 6;
pub const RLIMIT_RSS: ::c_int = 5;
#[deprecated(since = "0.2.64", note = "Not stable across OS versions")]
pub const RLIM_NLIMITS: ::c_int = 16;
pub const SA_ONSTACK: ::c_int = 0x08000000;
pub const SA_SIGINFO: ::c_int = 0x00000004;
pub const SA_NOCLDWAIT: ::c_int = 0x00000002;
pub const SIGBUS: ::c_int = 7;
pub const SIGCHLD: ::c_int = 17;
pub const SIGCONT: ::c_int = 18;
pub const SIGIO: ::c_int = 29;
pub const SIGPOLL: ::c_int = 29;
pub const SIGPROF: ::c_int = 27;
pub const SIGPWR: ::c_int = 30;
pub const SIGSTKFLT: ::c_int = 16;
pub const SIGSTOP: ::c_int = 19;
pub const SIGSYS: ::c_int = 31;
pub const SIGTSTP: ::c_int = 20;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGURG: ::c_int = 23;
pub const SIGUSR1: ::c_int = 10;
pub const SIGUSR2: ::c_int = 12;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGWINCH: ::c_int = 28;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIG_SETMASK: ::c_int = 2; // FIXME check these
pub const SIG_BLOCK: ::c_int = 0x000000;
pub const SIG_UNBLOCK: ::c_int = 0x01;
pub const SO_ACCEPTCONN: ::c_int = 30;
pub const SO_ATTACH_BPF: ::c_int = 50;
pub const SO_ATTACH_FILTER: ::c_int = 26;
pub const SO_ATTACH_REUSEPORT_CBPF: ::c_int = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: ::c_int = 52;
pub const SO_BPF_EXTENSIONS: ::c_int = 48;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_BSDCOMPAT: ::c_int = 14;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_NONBLOCK: ::c_int = 2048;
pub const SOCK_SEQPACKET: ::c_int = 5;
pub const SOCK_STREAM: ::c_int = 1;
pub const SO_CNX_ADVICE: ::c_int = 53;
pub const SO_DETACH_BPF: ::c_int = 27;
pub const SO_DETACH_FILTER: ::c_int = 27;
pub const SO_DOMAIN: ::c_int = 39;
pub const SO_DONTROUTE: ::c_int = 5;
pub const SO_ERROR: ::c_int = 4;
pub const SO_GET_FILTER: ::c_int = 26;
pub const SO_INCOMING_CPU: ::c_int = 49;
pub const SO_KEEPALIVE: ::c_int = 9;
pub const SOL_CAIF: ::c_int = 278;
pub const SO_LINGER: ::c_int = 13;
pub const SOL_IUCV: ::c_int = 277;
pub const SOL_KCM: ::c_int = 281;
pub const SOL_NFC: ::c_int = 280;
pub const SO_LOCK_FILTER: ::c_int = 44;
pub const SOL_PNPIPE: ::c_int = 275;
pub const SOL_PPPOL2TP: ::c_int = 273;
pub const SOL_RDS: ::c_int = 276;
pub const SOL_RXRPC: ::c_int = 272;
pub const SOL_SOCKET: ::c_int = 1;
pub const SO_MAX_PACING_RATE: ::c_int = 47;
pub const SO_NO_CHECK: ::c_int = 11;
pub const SO_NOFCS: ::c_int = 43;
pub const SO_OOBINLINE: ::c_int = 10;
pub const SO_PASSCRED: ::c_int = 16;
pub const SO_PASSSEC: ::c_int = 34;
pub const SO_PEERCRED: ::c_int = 17;
pub const SO_PEERNAME: ::c_int = 28;
pub const SO_PEERSEC: ::c_int = 31;
pub const SO_PRIORITY: ::c_int = 12;
pub const SO_PROTOCOL: ::c_int = 38;
pub const SO_RCVBUF: ::c_int = 8;
pub const SO_RCVBUFFORCE: ::c_int = 33;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_REUSEADDR: ::c_int = 2;
pub const SO_REUSEPORT: ::c_int = 15;
pub const SO_SECURITY_AUTHENTICATION: ::c_int = 22;
pub const SO_SECURITY_ENCRYPTION_NETWORK: ::c_int = 24;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: ::c_int = 23;
pub const SO_SELECT_ERR_QUEUE: ::c_int = 45;
pub const SO_SNDBUF: ::c_int = 7;
pub const SO_SNDBUFFORCE: ::c_int = 32;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_SNDTIMEO: ::c_int = 21;
pub const SO_TYPE: ::c_int = 3;
pub const SO_WIFI_STATUS: ::c_int = 41;
pub const SYS3264_fadvise64: ::c_int = 223;
pub const SYS3264_fcntl: ::c_int = 25;
pub const SYS3264_fstatat: ::c_int = 79;
pub const SYS3264_fstat: ::c_int = 80;
pub const SYS3264_fstatfs: ::c_int = 44;
pub const SYS3264_ftruncate: ::c_int = 46;
pub const SYS3264_lseek: ::c_int = 62;
pub const SYS3264_lstat: ::c_int = 1039;
pub const SYS3264_mmap: ::c_int = 222;
pub const SYS3264_sendfile: ::c_int = 71;
pub const SYS3264_stat: ::c_int = 1038;
pub const SYS3264_statfs: ::c_int = 43;
pub const SYS3264_truncate: ::c_int = 45;
pub const TCFLSH: ::c_int = 21515;
pub const TCGETA: ::c_int = 21509;
pub const TCGETS: ::c_int = 21505;
pub const TCGETX: ::c_int = 21554;
pub const TCSBRK: ::c_int = 21513;
pub const TCSBRKP: ::c_int = 21541;
pub const TCSETA: ::c_int = 21510;
pub const TCSETAF: ::c_int = 21512;
pub const TCSETAW: ::c_int = 21511;
pub const TCSETS: ::c_int = 21506;
pub const TCSETSF: ::c_int = 21508;
pub const TCSETSW: ::c_int = 21507;
pub const TCSETX: ::c_int = 21555;
pub const TCSETXF: ::c_int = 21556;
pub const TCSETXW: ::c_int = 21557;
pub const TCXONC: ::c_int = 21514;
pub const TIOCCONS: ::c_int = 21533;
pub const TIOCEXCL: ::c_int = 21516;
pub const TIOCGETD: ::c_int = 21540;
pub const TIOCGICOUNT: ::c_int = 21597;
pub const TIOCGLCKTRMIOS: ::c_int = 21590;
pub const TIOCGPGRP: ::c_int = 21519;
pub const TIOCGRS485: ::c_int = 21550;
pub const TIOCGSERIAL: ::c_int = 21534;
pub const TIOCGSID: ::c_int = 21545;
pub const TIOCGSOFTCAR: ::c_int = 21529;
pub const TIOCGWINSZ: ::c_int = 21523;
pub const TIOCLINUX: ::c_int = 21532;
pub const TIOCMBIC: ::c_int = 21527;
pub const TIOCMBIS: ::c_int = 21526;
pub const TIOCM_CAR: ::c_int = 64;
pub const TIOCM_CD: ::c_int = 64;
pub const TIOCM_CTS: ::c_int = 32;
pub const TIOCM_DSR: ::c_int = 256;
pub const TIOCM_DTR: ::c_int = 2;
pub const TIOCMGET: ::c_int = 21525;
pub const TIOCMIWAIT: ::c_int = 21596;
pub const TIOCM_LE: ::c_int = 1;
pub const TIOCM_LOOP: ::c_int = 32768;
pub const TIOCM_OUT1: ::c_int = 8192;
pub const TIOCM_OUT2: ::c_int = 16384;
pub const TIOCM_RI: ::c_int = 128;
pub const TIOCM_RNG: ::c_int = 128;
pub const TIOCM_RTS: ::c_int = 4;
pub const TIOCMSET: ::c_int = 21528;
pub const TIOCM_SR: ::c_int = 16;
pub const TIOCM_ST: ::c_int = 8;
pub const TIOCNOTTY: ::c_int = 21538;
pub const TIOCNXCL: ::c_int = 21517;
pub const TIOCOUTQ: ::c_int = 21521;
pub const TIOCPKT: ::c_int = 21536;
pub const TIOCPKT_DATA: ::c_int = 0;
pub const TIOCPKT_DOSTOP: ::c_int = 32;
pub const TIOCPKT_FLUSHREAD: ::c_int = 1;
pub const TIOCPKT_FLUSHWRITE: ::c_int = 2;
pub const TIOCPKT_IOCTL: ::c_int = 64;
pub const TIOCPKT_NOSTOP: ::c_int = 16;
pub const TIOCPKT_START: ::c_int = 8;
pub const TIOCPKT_STOP: ::c_int = 4;
pub const TIOCSCTTY: ::c_int = 21518;
pub const TIOCSERCONFIG: ::c_int = 21587;
pub const TIOCSERGETLSR: ::c_int = 21593;
pub const TIOCSERGETMULTI: ::c_int = 21594;
pub const TIOCSERGSTRUCT: ::c_int = 21592;
pub const TIOCSERGWILD: ::c_int = 21588;
pub const TIOCSERSETMULTI: ::c_int = 21595;
pub const TIOCSERSWILD: ::c_int = 21589;
pub const TIOCSER_TEMT: ::c_int = 1;
pub const TIOCSETD: ::c_int = 21539;
pub const TIOCSLCKTRMIOS: ::c_int = 21591;
pub const TIOCSPGRP: ::c_int = 21520;
pub const TIOCSRS485: ::c_int = 21551;
pub const TIOCSSERIAL: ::c_int = 21535;
pub const TIOCSSOFTCAR: ::c_int = 21530;
pub const TIOCSTI: ::c_int = 21522;
pub const TIOCSWINSZ: ::c_int = 21524;
pub const TIOCVHANGUP: ::c_int = 21559;
pub const TOSTOP: ::c_int = 256;
pub const VEOF: ::c_int = 4;
pub const VEOL2: ::c_int = 16;
pub const VEOL: ::c_int = 11;
pub const VMIN: ::c_int = 6;
