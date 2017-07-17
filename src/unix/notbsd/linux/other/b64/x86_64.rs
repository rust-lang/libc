//! x86_64-specific definitions for 64-bit linux-like values

pub type c_char = i8;
pub type wchar_t = i32;
pub type nlink_t = u64;
pub type blksize_t = i64;
pub type greg_t = i64;
pub type suseconds_t = i64;
pub type __u64 = ::c_ulonglong;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_nlink: ::nlink_t,
        pub st_mode: ::mode_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        __pad0: ::c_int,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_long; 3],
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino64_t,
        pub st_nlink: ::nlink_t,
        pub st_mode: ::mode_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        __pad0: ::c_int,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt64_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __reserved: [::c_long; 3],
    }

    pub struct pthread_attr_t {
        __size: [u64; 7]
    }

    pub struct _libc_fpxreg {
        pub significand: [u16; 4],
        pub exponent: u16,
        __private: [u16; 3],
    }

    pub struct _libc_xmmreg {
        pub element: [u32; 4],
    }

    pub struct _libc_fpstate {
        pub cwd: u16,
        pub swd: u16,
        pub ftw: u16,
        pub fop: u16,
        pub rip: u64,
        pub rdp: u64,
        pub mxcsr: u32,
        pub mxcr_mask: u32,
        pub _st: [_libc_fpxreg; 8],
        pub _xmm: [_libc_xmmreg; 16],
        __private: [u64; 12],
    }

    pub struct user_fpregs_struct {
        pub cwd: ::c_ushort,
        pub swd: ::c_ushort,
        pub ftw: ::c_ushort,
        pub fop: ::c_ushort,
        pub rip: ::c_ulonglong,
        pub rdp: ::c_ulonglong,
        pub mxcsr: ::c_uint,
        pub mxcr_mask: ::c_uint,
        pub st_space: [::c_uint; 32],
        pub xmm_space: [::c_uint; 64],
        padding: [::c_uint; 24],
    }

    pub struct user_regs_struct {
        pub r15: ::c_ulonglong,
        pub r14: ::c_ulonglong,
        pub r13: ::c_ulonglong,
        pub r12: ::c_ulonglong,
        pub rbp: ::c_ulonglong,
        pub rbx: ::c_ulonglong,
        pub r11: ::c_ulonglong,
        pub r10: ::c_ulonglong,
        pub r9: ::c_ulonglong,
        pub r8: ::c_ulonglong,
        pub rax: ::c_ulonglong,
        pub rcx: ::c_ulonglong,
        pub rdx: ::c_ulonglong,
        pub rsi: ::c_ulonglong,
        pub rdi: ::c_ulonglong,
        pub orig_rax: ::c_ulonglong,
        pub rip: ::c_ulonglong,
        pub cs: ::c_ulonglong,
        pub eflags: ::c_ulonglong,
        pub rsp: ::c_ulonglong,
        pub ss: ::c_ulonglong,
        pub fs_base: ::c_ulonglong,
        pub gs_base: ::c_ulonglong,
        pub ds: ::c_ulonglong,
        pub es: ::c_ulonglong,
        pub fs: ::c_ulonglong,
        pub gs: ::c_ulonglong,
    }

    pub struct user {
        pub regs: user_regs_struct,
        pub u_fpvalid: ::c_int,
        pub i387: user_fpregs_struct,
        pub u_tsize: ::c_ulonglong,
        pub u_dsize: ::c_ulonglong,
        pub u_ssize: ::c_ulonglong,
        pub start_code: ::c_ulonglong,
        pub start_stack: ::c_ulonglong,
        pub signal: ::c_longlong,
        __reserved: ::c_int,
        pub u_ar0: *mut user_regs_struct,
        pub u_fpstate: *mut user_fpregs_struct,
        pub magic: ::c_ulonglong,
        pub u_comm: [::c_char; 32],
        pub u_debugreg: [::c_ulonglong; 8],
    }

    pub struct mcontext_t {
        pub gregs: [greg_t; 23],
        pub fpregs: *mut _libc_fpstate,
        __private: [u64; 8],
    }

    pub struct ucontext_t {
        pub uc_flags: ::c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_mcontext: mcontext_t,
        pub uc_sigmask: ::sigset_t,
        __private: [u8; 512],
    }

    pub struct ipc_perm {
        pub __key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::c_ushort,
        __pad1: ::c_ushort,
        pub __seq: ::c_ushort,
        __pad2: ::c_ushort,
        __unused1: ::c_ulong,
        __unused2: ::c_ulong
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_segsz: ::size_t,
        pub shm_atime: ::time_t,
        pub shm_dtime: ::time_t,
        pub shm_ctime: ::time_t,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        __unused4: ::c_ulong,
        __unused5: ::c_ulong
    }
}

pub const TIOCGSOFTCAR: ::c_ulong = 0x5419;
pub const TIOCSSOFTCAR: ::c_ulong = 0x541A;

pub const RLIMIT_NOFILE: ::c_int = 7;
pub const RLIMIT_NPROC: ::c_int = 6;

pub const O_APPEND: ::c_int = 1024;
pub const O_CREAT: ::c_int = 64;
pub const O_EXCL: ::c_int = 128;
pub const O_NOCTTY: ::c_int = 256;
pub const O_NONBLOCK: ::c_int = 2048;
pub const O_SYNC: ::c_int = 1052672;
pub const O_RSYNC: ::c_int = 1052672;
pub const O_DSYNC: ::c_int = 4096;
pub const O_FSYNC: ::c_int = 0x101000;

pub const MAP_GROWSDOWN: ::c_int = 0x0100;

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
pub const EMULTIHOP: ::c_int = 72;
pub const EOVERFLOW: ::c_int = 75;
pub const ENOTUNIQ: ::c_int = 76;
pub const EBADFD: ::c_int = 77;
pub const EBADMSG: ::c_int = 74;
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
pub const EHWPOISON: ::c_int = 133;
pub const ERFKILL: ::c_int = 132;

pub const SOL_SOCKET: ::c_int = 1;

pub const SO_REUSEADDR: ::c_int = 2;
pub const SO_TYPE: ::c_int = 3;
pub const SO_ERROR: ::c_int = 4;
pub const SO_DONTROUTE: ::c_int = 5;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_SNDBUF: ::c_int = 7;
pub const SO_RCVBUF: ::c_int = 8;
pub const SO_SNDBUFFORCE: ::c_int = 32;
pub const SO_RCVBUFFORCE: ::c_int = 33;
pub const SO_KEEPALIVE: ::c_int = 9;
pub const SO_OOBINLINE: ::c_int = 10;
pub const SO_NO_CHECK: ::c_int = 11;
pub const SO_PRIORITY: ::c_int = 12;
pub const SO_LINGER: ::c_int = 13;
pub const SO_BSDCOMPAT: ::c_int = 14;
pub const SO_REUSEPORT: ::c_int = 15;
pub const SO_PASSCRED: ::c_int = 16;
pub const SO_PEERCRED: ::c_int = 17;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_SNDTIMEO: ::c_int = 21;
pub const SO_SECURITY_AUTHENTICATION: ::c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: ::c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: ::c_int = 24;
pub const SO_BINDTODEVICE: ::c_int = 25;
pub const SO_ATTACH_FILTER: ::c_int = 26;
pub const SO_DETACH_FILTER: ::c_int = 27;
pub const SO_GET_FILTER: ::c_int = SO_ATTACH_FILTER;
pub const SO_PEERNAME: ::c_int = 28;
pub const SO_TIMESTAMP: ::c_int = 29;
pub const SO_ACCEPTCONN: ::c_int = 30;
pub const SO_PEERSEC: ::c_int = 31;
pub const SO_PASSSEC: ::c_int = 34;
pub const SO_TIMESTAMPNS: ::c_int = 35;
pub const SCM_TIMESTAMPNS: ::c_int = SO_TIMESTAMPNS;
pub const SO_MARK: ::c_int = 36;
pub const SO_TIMESTAMPING: ::c_int = 37;
pub const SCM_TIMESTAMPING: ::c_int = SO_TIMESTAMPING;
pub const SO_PROTOCOL: ::c_int = 38;
pub const SO_DOMAIN: ::c_int = 39;
pub const SO_RXQ_OVFL: ::c_int = 40;
pub const SO_WIFI_STATUS: ::c_int = 41;
pub const SCM_WIFI_STATUS: ::c_int = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: ::c_int = 42;
pub const SO_NOFCS: ::c_int = 43;
pub const SO_LOCK_FILTER: ::c_int = 44;
pub const SO_SELECT_ERR_QUEUE: ::c_int = 45;
pub const SO_BUSY_POLL: ::c_int = 46;
pub const SO_MAX_PACING_RATE: ::c_int = 47;
pub const SO_BPF_EXTENSIONS: ::c_int = 48;
pub const SO_INCOMING_CPU: ::c_int = 49;
pub const SO_ATTACH_BPF: ::c_int = 50;
pub const SO_DETACH_BPF: ::c_int = SO_DETACH_FILTER;

pub const SA_ONSTACK: ::c_int = 0x08000000;
pub const SA_SIGINFO: ::c_int = 0x00000004;
pub const SA_NOCLDWAIT: ::c_int = 0x00000002;

pub const SIGCHLD: ::c_int = 17;
pub const SIGBUS: ::c_int = 7;
pub const SIGUSR1: ::c_int = 10;
pub const SIGUSR2: ::c_int = 12;
pub const SIGCONT: ::c_int = 18;
pub const SIGSTOP: ::c_int = 19;
pub const SIGTSTP: ::c_int = 20;
pub const SIGURG: ::c_int = 23;
pub const SIGIO: ::c_int = 29;
pub const SIGSYS: ::c_int = 31;
pub const SIGSTKFLT: ::c_int = 16;
pub const SIGUNUSED: ::c_int = 31;
pub const SIGPOLL: ::c_int = 29;
pub const SIGPWR: ::c_int = 30;
pub const SIG_SETMASK: ::c_int = 2;
pub const SIG_BLOCK: ::c_int = 0x000000;
pub const SIG_UNBLOCK: ::c_int = 0x01;

pub const POLLWRNORM: ::c_short = 0x100;
pub const POLLWRBAND: ::c_short = 0x200;

pub const O_ASYNC: ::c_int = 0x2000;
pub const O_NDELAY: ::c_int = 0x800;

pub const PTRACE_DETACH: ::c_uint = 17;

pub const EFD_NONBLOCK: ::c_int = 0x800;

pub const F_GETLK: ::c_int = 5;
pub const F_GETOWN: ::c_int = 9;
pub const F_SETOWN: ::c_int = 8;
pub const F_SETLK: ::c_int = 6;
pub const F_SETLKW: ::c_int = 7;

pub const SFD_NONBLOCK: ::c_int = 0x0800;

pub const TIOCEXCL: ::c_ulong = 0x540C;
pub const TIOCNXCL: ::c_ulong = 0x540D;
pub const TIOCSCTTY: ::c_ulong = 0x540E;
pub const TIOCSTI: ::c_ulong = 0x5412;
pub const TIOCMGET: ::c_ulong = 0x5415;
pub const TIOCMBIS: ::c_ulong = 0x5416;
pub const TIOCMBIC: ::c_ulong = 0x5417;
pub const TIOCMSET: ::c_ulong = 0x5418;
pub const TIOCCONS: ::c_ulong = 0x541D;

pub const SFD_CLOEXEC: ::c_int = 0x080000;

pub const NCCS: usize = 32;

pub const O_TRUNC: ::c_int = 512;

pub const O_CLOEXEC: ::c_int = 0x80000;

pub const EBFONT: ::c_int = 59;
pub const ENOSTR: ::c_int = 60;
pub const ENODATA: ::c_int = 61;
pub const ETIME: ::c_int = 62;
pub const ENOSR: ::c_int = 63;
pub const ENONET: ::c_int = 64;
pub const ENOPKG: ::c_int = 65;
pub const EREMOTE: ::c_int = 66;
pub const ENOLINK: ::c_int = 67;
pub const EADV: ::c_int = 68;
pub const ESRMNT: ::c_int = 69;
pub const ECOMM: ::c_int = 70;
pub const EPROTO: ::c_int = 71;
pub const EDOTDOT: ::c_int = 73;

pub const SA_NODEFER: ::c_int = 0x40000000;
pub const SA_RESETHAND: ::c_int = 0x80000000;
pub const SA_RESTART: ::c_int = 0x10000000;
pub const SA_NOCLDSTOP: ::c_int = 0x00000001;

pub const EPOLL_CLOEXEC: ::c_int = 0x80000;

pub const EFD_CLOEXEC: ::c_int = 0x80000;

pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;

pub const O_DIRECT: ::c_int = 0x4000;
pub const O_DIRECTORY: ::c_int = 0x10000;
pub const O_NOFOLLOW: ::c_int = 0x20000;

pub const MAP_LOCKED: ::c_int = 0x02000;
pub const MAP_NORESERVE: ::c_int = 0x04000;
pub const MAP_32BIT: ::c_int = 0x0040;

pub const EDEADLOCK: ::c_int = 35;

pub const FIOCLEX: ::c_ulong = 0x5451;
pub const FIONBIO: ::c_ulong = 0x5421;

pub const PTRACE_GETFPREGS: ::c_uint = 14;
pub const PTRACE_SETFPREGS: ::c_uint = 15;
pub const PTRACE_GETFPXREGS: ::c_uint = 18;
pub const PTRACE_SETFPXREGS: ::c_uint = 19;
pub const PTRACE_GETREGS: ::c_uint = 12;
pub const PTRACE_SETREGS: ::c_uint = 13;
pub const PTRACE_O_EXITKILL: ::c_uint = 1048576;
pub const PTRACE_O_TRACECLONE: ::c_uint = 8;
pub const PTRACE_O_TRACEEXEC: ::c_uint = 16;
pub const PTRACE_O_TRACEEXIT: ::c_uint = 64;
pub const PTRACE_O_TRACEFORK: ::c_uint = 2;
pub const PTRACE_O_TRACESYSGOOD: ::c_uint = 1;
pub const PTRACE_O_TRACEVFORK: ::c_uint = 4;
pub const PTRACE_O_TRACEVFORKDONE: ::c_uint = 32;
pub const PTRACE_O_TRACESECCOMP: ::c_uint = 128;
pub const PTRACE_O_SUSPEND_SECCOMP: ::c_uint = 2097152;
pub const PTRACE_PEEKSIGINFO_SHARED: ::c_uint = 1;

pub const SYS_gettid: ::c_long = 186;
pub const SYS_perf_event_open: ::c_long = 298;

pub const MCL_CURRENT: ::c_int = 0x0001;
pub const MCL_FUTURE: ::c_int = 0x0002;

pub const SIGSTKSZ: ::size_t = 8192;
pub const MINSIGSTKSZ: ::size_t = 2048;
pub const CBAUD: ::tcflag_t = 0o0010017;
pub const TAB1: ::c_int = 0x00000800;
pub const TAB2: ::c_int = 0x00001000;
pub const TAB3: ::c_int = 0x00001800;
pub const CR1: ::c_int  = 0x00000200;
pub const CR2: ::c_int  = 0x00000400;
pub const CR3: ::c_int  = 0x00000600;
pub const FF1: ::c_int  = 0x00008000;
pub const BS1: ::c_int  = 0x00002000;
pub const VT1: ::c_int  = 0x00004000;
pub const VWERASE: usize = 14;
pub const VREPRINT: usize = 12;
pub const VSUSP: usize = 10;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VDISCARD: usize = 13;
pub const VTIME: usize = 5;
pub const IXON: ::tcflag_t = 0x00000400;
pub const IXOFF: ::tcflag_t = 0x00001000;
pub const ONLCR: ::tcflag_t = 0x4;
pub const CSIZE: ::tcflag_t = 0x00000030;
pub const CS6: ::tcflag_t = 0x00000010;
pub const CS7: ::tcflag_t = 0x00000020;
pub const CS8: ::tcflag_t = 0x00000030;
pub const CSTOPB: ::tcflag_t = 0x00000040;
pub const CREAD: ::tcflag_t = 0x00000080;
pub const PARENB: ::tcflag_t = 0x00000100;
pub const PARODD: ::tcflag_t = 0x00000200;
pub const HUPCL: ::tcflag_t = 0x00000400;
pub const CLOCAL: ::tcflag_t = 0x00000800;
pub const ECHOKE: ::tcflag_t = 0x00000800;
pub const ECHOE: ::tcflag_t = 0x00000010;
pub const ECHOK: ::tcflag_t = 0x00000020;
pub const ECHONL: ::tcflag_t = 0x00000040;
pub const ECHOPRT: ::tcflag_t = 0x00000400;
pub const ECHOCTL: ::tcflag_t = 0x00000200;
pub const ISIG: ::tcflag_t = 0x00000001;
pub const ICANON: ::tcflag_t = 0x00000002;
pub const PENDIN: ::tcflag_t = 0x00004000;
pub const NOFLSH: ::tcflag_t = 0x00000080;
pub const CIBAUD: ::tcflag_t = 0o02003600000;
pub const CBAUDEX: ::tcflag_t = 0o010000;
pub const VSWTC: usize = 7;
pub const OLCUC:  ::tcflag_t = 0o000002;
pub const NLDLY:  ::tcflag_t = 0o000400;
pub const CRDLY:  ::tcflag_t = 0o003000;
pub const TABDLY: ::tcflag_t = 0o014000;
pub const BSDLY:  ::tcflag_t = 0o020000;
pub const FFDLY:  ::tcflag_t = 0o100000;
pub const VTDLY:  ::tcflag_t = 0o040000;
pub const XTABS:  ::tcflag_t = 0o014000;

pub const B0: ::speed_t = 0o000000;
pub const B50: ::speed_t = 0o000001;
pub const B75: ::speed_t = 0o000002;
pub const B110: ::speed_t = 0o000003;
pub const B134: ::speed_t = 0o000004;
pub const B150: ::speed_t = 0o000005;
pub const B200: ::speed_t = 0o000006;
pub const B300: ::speed_t = 0o000007;
pub const B600: ::speed_t = 0o000010;
pub const B1200: ::speed_t = 0o000011;
pub const B1800: ::speed_t = 0o000012;
pub const B2400: ::speed_t = 0o000013;
pub const B4800: ::speed_t = 0o000014;
pub const B9600: ::speed_t = 0o000015;
pub const B19200: ::speed_t = 0o000016;
pub const B38400: ::speed_t = 0o000017;
pub const EXTA: ::speed_t = B19200;
pub const EXTB: ::speed_t = B38400;
pub const B57600: ::speed_t = 0o010001;
pub const B115200: ::speed_t = 0o010002;
pub const B230400: ::speed_t = 0o010003;
pub const B460800: ::speed_t = 0o010004;
pub const B500000: ::speed_t = 0o010005;
pub const B576000: ::speed_t = 0o010006;
pub const B921600: ::speed_t = 0o010007;
pub const B1000000: ::speed_t = 0o010010;
pub const B1152000: ::speed_t = 0o010011;
pub const B1500000: ::speed_t = 0o010012;
pub const B2000000: ::speed_t = 0o010013;
pub const B2500000: ::speed_t = 0o010014;
pub const B3000000: ::speed_t = 0o010015;
pub const B3500000: ::speed_t = 0o010016;
pub const B4000000: ::speed_t = 0o010017;

pub const VEOL: usize = 11;
pub const VEOL2: usize = 16;
pub const VMIN: usize = 6;
pub const IEXTEN: ::tcflag_t = 0x00008000;
pub const TOSTOP: ::tcflag_t = 0x00000100;
pub const FLUSHO: ::tcflag_t = 0x00001000;
pub const EXTPROC: ::tcflag_t = 0x00010000;
pub const TCGETS: ::c_ulong = 0x5401;
pub const TCSETS: ::c_ulong = 0x5402;
pub const TCSETSW: ::c_ulong = 0x5403;
pub const TCSETSF: ::c_ulong = 0x5404;
pub const TCGETA: ::c_ulong = 0x5405;
pub const TCSETA: ::c_ulong = 0x5406;
pub const TCSETAW: ::c_ulong = 0x5407;
pub const TCSETAF: ::c_ulong = 0x5408;
pub const TCSBRK: ::c_ulong = 0x5409;
pub const TCXONC: ::c_ulong = 0x540A;
pub const TCFLSH: ::c_ulong = 0x540B;
pub const TIOCINQ: ::c_ulong = 0x541B;
pub const TIOCGPGRP: ::c_ulong = 0x540F;
pub const TIOCSPGRP: ::c_ulong = 0x5410;
pub const TIOCOUTQ: ::c_ulong = 0x5411;
pub const TIOCGWINSZ: ::c_ulong = 0x5413;
pub const TIOCSWINSZ: ::c_ulong = 0x5414;
pub const FIONREAD: ::c_ulong = 0x541B;

extern {
    pub fn getcontext(ucp: *mut ucontext_t) -> ::c_int;
    pub fn setcontext(ucp: *const ucontext_t) -> ::c_int;
    pub fn makecontext(ucp: *mut ucontext_t,
                       func:  extern fn (),
                       argc: ::c_int, ...);
    pub fn swapcontext(uocp: *mut ucontext_t,
                       ucp: *const ucontext_t) -> ::c_int;
    pub fn iopl(level: ::c_int) -> ::c_int;
    pub fn ioperm(from: ::c_ulong, num: ::c_ulong,
                  turn_on: ::c_int) -> ::c_int;
}

// Syscall table

pub const __NR_read: ::c_int = 0;
pub const __NR_write: ::c_int = 1;
pub const __NR_open: ::c_int = 2;
pub const __NR_close: ::c_int = 3;
pub const __NR_stat: ::c_int = 4;
pub const __NR_fstat: ::c_int = 5;
pub const __NR_lstat: ::c_int = 6;
pub const __NR_poll: ::c_int = 7;
pub const __NR_lseek: ::c_int = 8;
pub const __NR_mmap: ::c_int = 9;
pub const __NR_mprotect: ::c_int = 10;
pub const __NR_munmap: ::c_int = 11;
pub const __NR_brk: ::c_int = 12;
pub const __NR_rt_sigaction: ::c_int = 13;
pub const __NR_rt_sigprocmask: ::c_int = 14;
pub const __NR_rt_sigreturn: ::c_int = 15;
pub const __NR_ioctl: ::c_int = 16;
pub const __NR_pread64: ::c_int = 17;
pub const __NR_pwrite64: ::c_int = 18;
pub const __NR_readv: ::c_int = 19;
pub const __NR_writev: ::c_int = 20;
pub const __NR_access: ::c_int = 21;
pub const __NR_pipe: ::c_int = 22;
pub const __NR_select: ::c_int = 23;
pub const __NR_sched_yield: ::c_int = 24;
pub const __NR_mremap: ::c_int = 25;
pub const __NR_msync: ::c_int = 26;
pub const __NR_mincore: ::c_int = 27;
pub const __NR_madvise: ::c_int = 28;
pub const __NR_shmget: ::c_int = 29;
pub const __NR_shmat: ::c_int = 30;
pub const __NR_shmctl: ::c_int = 31;
pub const __NR_dup: ::c_int = 32;
pub const __NR_dup2: ::c_int = 33;
pub const __NR_pause: ::c_int = 34;
pub const __NR_nanosleep: ::c_int = 35;
pub const __NR_getitimer: ::c_int = 36;
pub const __NR_alarm: ::c_int = 37;
pub const __NR_setitimer: ::c_int = 38;
pub const __NR_getpid: ::c_int = 39;
pub const __NR_sendfile: ::c_int = 40;
pub const __NR_socket: ::c_int = 41;
pub const __NR_connect: ::c_int = 42;
pub const __NR_accept: ::c_int = 43;
pub const __NR_sendto: ::c_int = 44;
pub const __NR_recvfrom: ::c_int = 45;
pub const __NR_sendmsg: ::c_int = 46;
pub const __NR_recvmsg: ::c_int = 47;
pub const __NR_shutdown: ::c_int = 48;
pub const __NR_bind: ::c_int = 49;
pub const __NR_listen: ::c_int = 50;
pub const __NR_getsockname: ::c_int = 51;
pub const __NR_getpeername: ::c_int = 52;
pub const __NR_socketpair: ::c_int = 53;
pub const __NR_setsockopt: ::c_int = 54;
pub const __NR_getsockopt: ::c_int = 55;
pub const __NR_clone: ::c_int = 56;
pub const __NR_fork: ::c_int = 57;
pub const __NR_vfork: ::c_int = 58;
pub const __NR_execve: ::c_int = 59;
pub const __NR_exit: ::c_int = 60;
pub const __NR_wait4: ::c_int = 61;
pub const __NR_kill: ::c_int = 62;
pub const __NR_uname: ::c_int = 63;
pub const __NR_semget: ::c_int = 64;
pub const __NR_semop: ::c_int = 65;
pub const __NR_semctl: ::c_int = 66;
pub const __NR_shmdt: ::c_int = 67;
pub const __NR_msgget: ::c_int = 68;
pub const __NR_msgsnd: ::c_int = 69;
pub const __NR_msgrcv: ::c_int = 70;
pub const __NR_msgctl: ::c_int = 71;
pub const __NR_fcntl: ::c_int = 72;
pub const __NR_flock: ::c_int = 73;
pub const __NR_fsync: ::c_int = 74;
pub const __NR_fdatasync: ::c_int = 75;
pub const __NR_truncate: ::c_int = 76;
pub const __NR_ftruncate: ::c_int = 77;
pub const __NR_getdents: ::c_int = 78;
pub const __NR_getcwd: ::c_int = 79;
pub const __NR_chdir: ::c_int = 80;
pub const __NR_fchdir: ::c_int = 81;
pub const __NR_rename: ::c_int = 82;
pub const __NR_mkdir: ::c_int = 83;
pub const __NR_rmdir: ::c_int = 84;
pub const __NR_creat: ::c_int = 85;
pub const __NR_link: ::c_int = 86;
pub const __NR_unlink: ::c_int = 87;
pub const __NR_symlink: ::c_int = 88;
pub const __NR_readlink: ::c_int = 89;
pub const __NR_chmod: ::c_int = 90;
pub const __NR_fchmod: ::c_int = 91;
pub const __NR_chown: ::c_int = 92;
pub const __NR_fchown: ::c_int = 93;
pub const __NR_lchown: ::c_int = 94;
pub const __NR_umask: ::c_int = 95;
pub const __NR_gettimeofday: ::c_int = 96;
pub const __NR_getrlimit: ::c_int = 97;
pub const __NR_getrusage: ::c_int = 98;
pub const __NR_sysinfo: ::c_int = 99;
pub const __NR_times: ::c_int = 100;
pub const __NR_ptrace: ::c_int = 101;
pub const __NR_getuid: ::c_int = 102;
pub const __NR_syslog: ::c_int = 103;
pub const __NR_getgid: ::c_int = 104;
pub const __NR_setuid: ::c_int = 105;
pub const __NR_setgid: ::c_int = 106;
pub const __NR_geteuid: ::c_int = 107;
pub const __NR_getegid: ::c_int = 108;
pub const __NR_setpgid: ::c_int = 109;
pub const __NR_getppid: ::c_int = 110;
pub const __NR_getpgrp: ::c_int = 111;
pub const __NR_setsid: ::c_int = 112;
pub const __NR_setreuid: ::c_int = 113;
pub const __NR_setregid: ::c_int = 114;
pub const __NR_getgroups: ::c_int = 115;
pub const __NR_setgroups: ::c_int = 116;
pub const __NR_setresuid: ::c_int = 117;
pub const __NR_getresuid: ::c_int = 118;
pub const __NR_setresgid: ::c_int = 119;
pub const __NR_getresgid: ::c_int = 120;
pub const __NR_getpgid: ::c_int = 121;
pub const __NR_setfsuid: ::c_int = 122;
pub const __NR_setfsgid: ::c_int = 123;
pub const __NR_getsid: ::c_int = 124;
pub const __NR_capget: ::c_int = 125;
pub const __NR_capset: ::c_int = 126;
pub const __NR_rt_sigpending: ::c_int = 127;
pub const __NR_rt_sigtimedwait: ::c_int = 128;
pub const __NR_rt_sigqueueinfo: ::c_int = 129;
pub const __NR_rt_sigsuspend: ::c_int = 130;
pub const __NR_sigaltstack: ::c_int = 131;
pub const __NR_utime: ::c_int = 132;
pub const __NR_mknod: ::c_int = 133;
pub const __NR_uselib: ::c_int = 134;
pub const __NR_personality: ::c_int = 135;
pub const __NR_ustat: ::c_int = 136;
pub const __NR_statfs: ::c_int = 137;
pub const __NR_fstatfs: ::c_int = 138;
pub const __NR_sysfs: ::c_int = 139;
pub const __NR_getpriority: ::c_int = 140;
pub const __NR_setpriority: ::c_int = 141;
pub const __NR_sched_setparam: ::c_int = 142;
pub const __NR_sched_getparam: ::c_int = 143;
pub const __NR_sched_setscheduler: ::c_int = 144;
pub const __NR_sched_getscheduler: ::c_int = 145;
pub const __NR_sched_get_priority_max: ::c_int = 146;
pub const __NR_sched_get_priority_min: ::c_int = 147;
pub const __NR_sched_rr_get_interval: ::c_int = 148;
pub const __NR_mlock: ::c_int = 149;
pub const __NR_munlock: ::c_int = 150;
pub const __NR_mlockall: ::c_int = 151;
pub const __NR_munlockall: ::c_int = 152;
pub const __NR_vhangup: ::c_int = 153;
pub const __NR_modify_ldt: ::c_int = 154;
pub const __NR_pivot_root: ::c_int = 155;
pub const __NR__sysctl: ::c_int = 156;
pub const __NR_prctl: ::c_int = 157;
pub const __NR_arch_prctl: ::c_int = 158;
pub const __NR_adjtimex: ::c_int = 159;
pub const __NR_setrlimit: ::c_int = 160;
pub const __NR_chroot: ::c_int = 161;
pub const __NR_sync: ::c_int = 162;
pub const __NR_acct: ::c_int = 163;
pub const __NR_settimeofday: ::c_int = 164;
pub const __NR_mount: ::c_int = 165;
pub const __NR_umount2: ::c_int = 166;
pub const __NR_swapon: ::c_int = 167;
pub const __NR_swapoff: ::c_int = 168;
pub const __NR_reboot: ::c_int = 169;
pub const __NR_sethostname: ::c_int = 170;
pub const __NR_setdomainname: ::c_int = 171;
pub const __NR_iopl: ::c_int = 172;
pub const __NR_ioperm: ::c_int = 173;
pub const __NR_create_module: ::c_int = 174;
pub const __NR_init_module: ::c_int = 175;
pub const __NR_delete_module: ::c_int = 176;
pub const __NR_get_kernel_syms: ::c_int = 177;
pub const __NR_query_module: ::c_int = 178;
pub const __NR_quotactl: ::c_int = 179;
pub const __NR_nfsservctl: ::c_int = 180;
pub const __NR_getpmsg: ::c_int = 181;
pub const __NR_putpmsg: ::c_int = 182;
pub const __NR_afs_syscall: ::c_int = 183;
pub const __NR_tuxcall: ::c_int = 184;
pub const __NR_security: ::c_int = 185;
pub const __NR_gettid: ::c_int = 186;
pub const __NR_readahead: ::c_int = 187;
pub const __NR_setxattr: ::c_int = 188;
pub const __NR_lsetxattr: ::c_int = 189;
pub const __NR_fsetxattr: ::c_int = 190;
pub const __NR_getxattr: ::c_int = 191;
pub const __NR_lgetxattr: ::c_int = 192;
pub const __NR_fgetxattr: ::c_int = 193;
pub const __NR_listxattr: ::c_int = 194;
pub const __NR_llistxattr: ::c_int = 195;
pub const __NR_flistxattr: ::c_int = 196;
pub const __NR_removexattr: ::c_int = 197;
pub const __NR_lremovexattr: ::c_int = 198;
pub const __NR_fremovexattr: ::c_int = 199;
pub const __NR_tkill: ::c_int = 200;
pub const __NR_time: ::c_int = 201;
pub const __NR_futex: ::c_int = 202;
pub const __NR_sched_setaffinity: ::c_int = 203;
pub const __NR_sched_getaffinity: ::c_int = 204;
pub const __NR_set_thread_area: ::c_int = 205;
pub const __NR_io_setup: ::c_int = 206;
pub const __NR_io_destroy: ::c_int = 207;
pub const __NR_io_getevents: ::c_int = 208;
pub const __NR_io_submit: ::c_int = 209;
pub const __NR_io_cancel: ::c_int = 210;
pub const __NR_get_thread_area: ::c_int = 211;
pub const __NR_lookup_dcookie: ::c_int = 212;
pub const __NR_epoll_create: ::c_int = 213;
pub const __NR_epoll_ctl_old: ::c_int = 214;
pub const __NR_epoll_wait_old: ::c_int = 215;
pub const __NR_remap_file_pages: ::c_int = 216;
pub const __NR_getdents64: ::c_int = 217;
pub const __NR_set_tid_address: ::c_int = 218;
pub const __NR_restart_syscall: ::c_int = 219;
pub const __NR_semtimedop: ::c_int = 220;
pub const __NR_fadvise64: ::c_int = 221;
pub const __NR_timer_create: ::c_int = 222;
pub const __NR_timer_settime: ::c_int = 223;
pub const __NR_timer_gettime: ::c_int = 224;
pub const __NR_timer_getoverrun: ::c_int = 225;
pub const __NR_timer_delete: ::c_int = 226;
pub const __NR_clock_settime: ::c_int = 227;
pub const __NR_clock_gettime: ::c_int = 228;
pub const __NR_clock_getres: ::c_int = 229;
pub const __NR_clock_nanosleep: ::c_int = 230;
pub const __NR_exit_group: ::c_int = 231;
pub const __NR_epoll_wait: ::c_int = 232;
pub const __NR_epoll_ctl: ::c_int = 233;
pub const __NR_tgkill: ::c_int = 234;
pub const __NR_utimes: ::c_int = 235;
pub const __NR_vserver: ::c_int = 236;
pub const __NR_mbind: ::c_int = 237;
pub const __NR_set_mempolicy: ::c_int = 238;
pub const __NR_get_mempolicy: ::c_int = 239;
pub const __NR_mq_open: ::c_int = 240;
pub const __NR_mq_unlink: ::c_int = 241;
pub const __NR_mq_timedsend: ::c_int = 242;
pub const __NR_mq_timedreceive: ::c_int = 243;
pub const __NR_mq_notify: ::c_int = 244;
pub const __NR_mq_getsetattr: ::c_int = 245;
pub const __NR_kexec_load: ::c_int = 246;
pub const __NR_waitid: ::c_int = 247;
pub const __NR_add_key: ::c_int = 248;
pub const __NR_request_key: ::c_int = 249;
pub const __NR_keyctl: ::c_int = 250;
pub const __NR_ioprio_set: ::c_int = 251;
pub const __NR_ioprio_get: ::c_int = 252;
pub const __NR_inotify_init: ::c_int = 253;
pub const __NR_inotify_add_watch: ::c_int = 254;
pub const __NR_inotify_rm_watch: ::c_int = 255;
pub const __NR_migrate_pages: ::c_int = 256;
pub const __NR_openat: ::c_int = 257;
pub const __NR_mkdirat: ::c_int = 258;
pub const __NR_mknodat: ::c_int = 259;
pub const __NR_fchownat: ::c_int = 260;
pub const __NR_futimesat: ::c_int = 261;
pub const __NR_newfstatat: ::c_int = 262;
pub const __NR_unlinkat: ::c_int = 263;
pub const __NR_renameat: ::c_int = 264;
pub const __NR_linkat: ::c_int = 265;
pub const __NR_symlinkat: ::c_int = 266;
pub const __NR_readlinkat: ::c_int = 267;
pub const __NR_fchmodat: ::c_int = 268;
pub const __NR_faccessat: ::c_int = 269;
pub const __NR_pselect6: ::c_int = 270;
pub const __NR_ppoll: ::c_int = 271;
pub const __NR_unshare: ::c_int = 272;
pub const __NR_set_robust_list: ::c_int = 273;
pub const __NR_get_robust_list: ::c_int = 274;
pub const __NR_splice: ::c_int = 275;
pub const __NR_tee: ::c_int = 276;
pub const __NR_sync_file_range: ::c_int = 277;
pub const __NR_vmsplice: ::c_int = 278;
pub const __NR_move_pages: ::c_int = 279;
pub const __NR_utimensat: ::c_int = 280;
pub const __NR_epoll_pwait: ::c_int = 281;
pub const __NR_signalfd: ::c_int = 282;
pub const __NR_timerfd_create: ::c_int = 283;
pub const __NR_eventfd: ::c_int = 284;
pub const __NR_fallocate: ::c_int = 285;
pub const __NR_timerfd_settime: ::c_int = 286;
pub const __NR_timerfd_gettime: ::c_int = 287;
pub const __NR_accept4: ::c_int = 288;
pub const __NR_signalfd4: ::c_int = 289;
pub const __NR_eventfd2: ::c_int = 290;
pub const __NR_epoll_create1: ::c_int = 291;
pub const __NR_dup3: ::c_int = 292;
pub const __NR_pipe2: ::c_int = 293;
pub const __NR_inotify_init1: ::c_int = 294;
pub const __NR_preadv: ::c_int = 295;
pub const __NR_pwritev: ::c_int = 296;
pub const __NR_rt_tgsigqueueinfo: ::c_int = 297;
pub const __NR_perf_event_open: ::c_int = 298;
pub const __NR_recvmmsg: ::c_int = 299;
pub const __NR_fanotify_init: ::c_int = 300;
pub const __NR_fanotify_mark: ::c_int = 301;
pub const __NR_prlimit64: ::c_int = 302;
pub const __NR_name_to_handle_at: ::c_int = 303;
pub const __NR_open_by_handle_at: ::c_int = 304;
pub const __NR_clock_adjtime: ::c_int = 305;
pub const __NR_syncfs: ::c_int = 306;
pub const __NR_sendmmsg: ::c_int = 307;
pub const __NR_setns: ::c_int = 308;
pub const __NR_getcpu: ::c_int = 309;
pub const __NR_process_vm_readv: ::c_int = 310;
pub const __NR_process_vm_writev: ::c_int = 311;
pub const __NR_kcmp: ::c_int = 312;
pub const __NR_finit_module: ::c_int = 313;
pub const __NR_sched_setattr: ::c_int = 314;
pub const __NR_sched_getattr: ::c_int = 315;
pub const __NR_renameat2: ::c_int = 316;
pub const __NR_seccomp: ::c_int = 317;
pub const __NR_getrandom: ::c_int = 318;
pub const __NR_memfd_create: ::c_int = 319;
pub const __NR_kexec_file_load: ::c_int = 320;
pub const __NR_bpf: ::c_int = 321;
pub const __NR_execveat: ::c_int = 322;
pub const __NR_userfaultfd: ::c_int = 323;
pub const __NR_membarrier: ::c_int = 324;
pub const __NR_mlock2: ::c_int = 325;
pub const __NR_copy_file_range: ::c_int = 326;
pub const __NR_preadv2: ::c_int = 327;
pub const __NR_pwritev2: ::c_int = 328;
pub const __NR_pkey_mprotect: ::c_int = 329;
pub const __NR_pkey_alloc: ::c_int = 330;
pub const __NR_pkey_free: ::c_int = 331;
