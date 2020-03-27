//! s390x

use pthread_mutex_t;

pub type blksize_t = i64;
pub type c_char = u8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type nlink_t = u64;
pub type suseconds_t = i64;
pub type wchar_t = i32;
pub type greg_t = u64;
pub type __u64 = u64;

s! {
    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        __glibc_reserved0: ::c_int,
        pub sa_flags: ::c_int,
        pub sa_restorer: ::Option<extern fn()>,
        pub sa_mask: ::sigset_t,
    }

    pub struct statfs {
        pub f_type: ::c_uint,
        pub f_bsize: ::c_uint,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::c_uint,
        pub f_frsize: ::c_uint,
        pub f_flags: ::c_uint,
        f_spare: [::c_uint; 4],
    }

    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_pid: ::pid_t,
    }

    pub struct flock64 {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off64_t,
        pub l_len: ::off64_t,
        pub l_pid: ::pid_t,
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        _pad: ::c_int,
        _pad2: [::c_long; 14],
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: ::size_t
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_nlink: ::nlink_t,
        pub st_mode: ::mode_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        st_pad0: ::c_int,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        __glibc_reserved: [::c_long; 3],
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino64_t,
        pub st_nlink: ::nlink_t,
        pub st_mode: ::mode_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        st_pad0: ::c_int,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt64_t,
        __glibc_reserved: [::c_long; 3],
    }

    pub struct pthread_attr_t {
        __size: [::c_ulong; 7]
    }

    pub struct ipc_perm {
        pub __key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::mode_t,
        pub __seq: ::c_ushort,
        __pad1: ::c_ushort,
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

    pub struct __psw_t {
        pub mask: u64,
        pub addr: u64,
    }

    pub struct fpregset_t {
        pub fpc: u32,
        __pad: u32,
        pub fprs: [fpreg_t; 16],
    }

    pub struct mcontext_t {
        pub psw: __psw_t,
        pub gregs: [u64; 16],
        pub aregs: [u32; 16],
        pub fpregs: fpregset_t,
    }

    pub struct ucontext_t {
        pub uc_flags: ::c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_mcontext: mcontext_t,
        pub uc_sigmask: ::sigset_t,
    }

    pub struct statfs64 {
        pub f_type: ::c_uint,
        pub f_bsize: ::c_uint,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_bavail: u64,
        pub f_files: u64,
        pub f_ffree: u64,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::c_uint,
        pub f_frsize: ::c_uint,
        pub f_flags: ::c_uint,
        pub f_spare: [::c_uint; 4],
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
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }
}

s_no_extra_traits! {
    // FIXME: This is actually a union.
    pub struct fpreg_t {
        pub d: ::c_double,
        // f: ::c_float,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for fpreg_t {
            fn eq(&self, other: &fpreg_t) -> bool {
                self.d == other.d
            }
        }

        impl Eq for fpreg_t {}

        impl ::fmt::Debug for fpreg_t {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("fpreg_t")
                    .field("d", &self.d)
                    .finish()
            }
        }

        impl ::hash::Hash for fpreg_t {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                let d: u64 = unsafe { ::mem::transmute(self.d) };
                d.hash(state);
            }
        }
    }
}

pub const POSIX_FADV_DONTNEED: ::c_int = 6;
pub const POSIX_FADV_NOREUSE: ::c_int = 7;

pub const VEOF: usize = 4;
pub const RTLD_DEEPBIND: ::c_int = 0x8;
pub const RTLD_GLOBAL: ::c_int = 0x100;
pub const RTLD_NOLOAD: ::c_int = 0x4;
pub const SFD_CLOEXEC: ::c_int = 0x080000;

pub const NCCS: usize = 32;

pub const O_TRUNC: ::c_int = 512;
pub const O_NOATIME: ::c_int = 0o1000000;
pub const O_CLOEXEC: ::c_int = 0x80000;
pub const O_PATH: ::c_int = 0o10000000;
pub const O_TMPFILE: ::c_int = 0o20000000 | O_DIRECTORY;

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
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56;

align_const! {
    pub const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
        pthread_mutex_t {
            size: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };
    pub const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
        pthread_mutex_t {
            size: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };
    pub const PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
        pthread_mutex_t {
            size: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };
}

pub const EUCLEAN: ::c_int = 117;
pub const ENOTNAM: ::c_int = 118;
pub const ENAVAIL: ::c_int = 119;
pub const EISNAM: ::c_int = 120;
pub const EREMOTEIO: ::c_int = 121;
pub const EADDRINUSE: ::c_int = 98;
pub const EADDRNOTAVAIL: ::c_int = 99;
pub const ECONNABORTED: ::c_int = 103;
pub const ECONNREFUSED: ::c_int = 111;
pub const ECONNRESET: ::c_int = 104;
pub const EDEADLK: ::c_int = 35;
pub const ENOSYS: ::c_int = 38;
pub const ENOTCONN: ::c_int = 107;
pub const ETIMEDOUT: ::c_int = 110;
pub const FIOCLEX: ::c_ulong = 0x5451;
pub const FIONCLEX: ::c_ulong = 0x5450;
pub const FIONBIO: ::c_ulong = 0x5421;
pub const O_APPEND: ::c_int = 1024;
pub const O_CREAT: ::c_int = 64;
pub const O_EXCL: ::c_int = 128;
pub const O_NONBLOCK: ::c_int = 2048;
pub const SA_NOCLDWAIT: ::c_int = 2;
pub const SA_ONSTACK: ::c_int = 0x08000000;
pub const SA_SIGINFO: ::c_int = 4;
pub const SIGBUS: ::c_int = 7;
pub const SIGSTKSZ: ::size_t = 0x2000;
pub const MINSIGSTKSZ: ::size_t = 2048;
pub const SIG_SETMASK: ::c_int = 2;
pub const SOL_SOCKET: ::c_int = 1;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_ERROR: ::c_int = 4;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_REUSEADDR: ::c_int = 2;
pub const SO_SNDTIMEO: ::c_int = 21;
pub const SO_BINDTODEVICE: ::c_int = 25;
pub const SO_TIMESTAMP: ::c_int = 29;
pub const SO_MARK: ::c_int = 36;
pub const SO_PROTOCOL: ::c_int = 38;
pub const SO_DOMAIN: ::c_int = 39;
pub const SO_RXQ_OVFL: ::c_int = 40;
pub const SO_PEEK_OFF: ::c_int = 42;
pub const SO_BUSY_POLL: ::c_int = 46;

pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;

pub const RLIMIT_RSS: ::__rlimit_resource_t = 5;
pub const RLIMIT_AS: ::__rlimit_resource_t = 9;
pub const RLIMIT_MEMLOCK: ::__rlimit_resource_t = 8;
pub const RLIMIT_NOFILE: ::__rlimit_resource_t = 7;
pub const RLIMIT_NPROC: ::__rlimit_resource_t = 6;

pub const O_NOCTTY: ::c_int = 256;
pub const O_SYNC: ::c_int = 1052672;
pub const O_RSYNC: ::c_int = 1052672;
pub const O_DSYNC: ::c_int = 4096;
pub const O_FSYNC: ::c_int = 0x101000;
pub const O_DIRECT: ::c_int = 0x4000;
pub const O_DIRECTORY: ::c_int = 0x10000;
pub const O_NOFOLLOW: ::c_int = 0x20000;

pub const MADV_SOFT_OFFLINE: ::c_int = 101;
pub const MAP_GROWSDOWN: ::c_int = 0x0100;
pub const MAP_LOCKED: ::c_int = 0x02000;
pub const MAP_NORESERVE: ::c_int = 0x04000;
pub const MAP_ANON: ::c_int = 0x0020;
pub const MAP_ANONYMOUS: ::c_int = 0x0020;
pub const MAP_DENYWRITE: ::c_int = 0x0800;
pub const MAP_EXECUTABLE: ::c_int = 0x01000;
pub const MAP_POPULATE: ::c_int = 0x08000;
pub const MAP_NONBLOCK: ::c_int = 0x010000;
pub const MAP_STACK: ::c_int = 0x020000;
pub const MAP_HUGETLB: ::c_int = 0x040000;

pub const EDEADLOCK: ::c_int = 35;
pub const ENAMETOOLONG: ::c_int = 36;
pub const ENOLCK: ::c_int = 37;
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
pub const ENETDOWN: ::c_int = 100;
pub const ENETUNREACH: ::c_int = 101;
pub const ENETRESET: ::c_int = 102;
pub const ENOBUFS: ::c_int = 105;
pub const EISCONN: ::c_int = 106;
pub const ESHUTDOWN: ::c_int = 108;
pub const ETOOMANYREFS: ::c_int = 109;
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

pub const SO_TYPE: ::c_int = 3;
pub const SO_DONTROUTE: ::c_int = 5;
pub const SO_SNDBUF: ::c_int = 7;
pub const SO_RCVBUF: ::c_int = 8;
pub const SO_KEEPALIVE: ::c_int = 9;
pub const SO_OOBINLINE: ::c_int = 10;
pub const SO_PRIORITY: ::c_int = 12;
pub const SO_LINGER: ::c_int = 13;
pub const SO_BSDCOMPAT: ::c_int = 14;
pub const SO_REUSEPORT: ::c_int = 15;
pub const SO_PASSCRED: ::c_int = 16;
pub const SO_PEERCRED: ::c_int = 17;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_ACCEPTCONN: ::c_int = 30;
pub const SO_SNDBUFFORCE: ::c_int = 32;
pub const SO_RCVBUFFORCE: ::c_int = 33;

pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF: ::c_int = 27;
pub const SIGWINCH: ::c_int = 28;
pub const SIGCHLD: ::c_int = 17;
pub const SIGUSR1: ::c_int = 10;
pub const SIGUSR2: ::c_int = 12;
pub const SIGCONT: ::c_int = 18;
pub const SIGSTOP: ::c_int = 19;
pub const SIGTSTP: ::c_int = 20;
pub const SIGURG: ::c_int = 23;
pub const SIGIO: ::c_int = 29;
pub const SIGSYS: ::c_int = 31;
pub const SIGSTKFLT: ::c_int = 16;
#[deprecated(since = "0.2.55", note = "Use SIGSYS instead")]
pub const SIGUNUSED: ::c_int = 31;
pub const SIGPOLL: ::c_int = 29;
pub const SIGPWR: ::c_int = 30;
pub const SIG_BLOCK: ::c_int = 0x000000;
pub const SIG_UNBLOCK: ::c_int = 0x01;

pub const O_ASYNC: ::c_int = 0x2000;
pub const O_NDELAY: ::c_int = 0x800;

pub const VEOL: usize = 11;
pub const VEOL2: usize = 16;
pub const VMIN: usize = 6;
pub const IEXTEN: ::tcflag_t = 0x00008000;
pub const TOSTOP: ::tcflag_t = 0x00000100;
pub const FLUSHO: ::tcflag_t = 0x00001000;

pub const EXTPROC: ::tcflag_t = 0x00010000;

pub const PTRACE_DETACH: ::c_uint = 17;

pub const MCL_CURRENT: ::c_int = 0x0001;
pub const MCL_FUTURE: ::c_int = 0x0002;

pub const EFD_NONBLOCK: ::c_int = 0x800;

pub const F_RDLCK: ::c_int = 0;
pub const F_WRLCK: ::c_int = 1;
pub const F_UNLCK: ::c_int = 2;
pub const F_GETLK: ::c_int = 5;
pub const F_GETOWN: ::c_int = 9;
pub const F_SETOWN: ::c_int = 8;
pub const F_SETLK: ::c_int = 6;
pub const F_SETLKW: ::c_int = 7;
pub const F_OFD_GETLK: ::c_int = 36;
pub const F_OFD_SETLK: ::c_int = 37;
pub const F_OFD_SETLKW: ::c_int = 38;

pub const SFD_NONBLOCK: ::c_int = 0x0800;

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
pub const TIOCGSOFTCAR: ::c_ulong = 0x5419;
pub const TIOCSSOFTCAR: ::c_ulong = 0x541A;
pub const TIOCINQ: ::c_ulong = 0x541B;
pub const TIOCEXCL: ::c_ulong = 0x540C;
pub const TIOCNXCL: ::c_ulong = 0x540D;
pub const TIOCSCTTY: ::c_ulong = 0x540E;
pub const TIOCGPGRP: ::c_ulong = 0x540F;
pub const TIOCSPGRP: ::c_ulong = 0x5410;
pub const TIOCOUTQ: ::c_ulong = 0x5411;
pub const TIOCSTI: ::c_ulong = 0x5412;
pub const TIOCGWINSZ: ::c_ulong = 0x5413;
pub const TIOCSWINSZ: ::c_ulong = 0x5414;
pub const TIOCMGET: ::c_ulong = 0x5415;
pub const TIOCMBIS: ::c_ulong = 0x5416;
pub const TIOCMBIC: ::c_ulong = 0x5417;
pub const TIOCMSET: ::c_ulong = 0x5418;
pub const FIONREAD: ::c_ulong = 0x541B;
pub const TIOCCONS: ::c_ulong = 0x541D;

pub const TCSANOW: ::c_int = 0;
pub const TCSADRAIN: ::c_int = 1;
pub const TCSAFLUSH: ::c_int = 2;

pub const TIOCLINUX: ::c_ulong = 0x541C;
pub const TIOCGSERIAL: ::c_ulong = 0x541E;
pub const TIOCM_ST: ::c_int = 0x008;
pub const TIOCM_SR: ::c_int = 0x010;
pub const TIOCM_CTS: ::c_int = 0x020;
pub const TIOCM_CAR: ::c_int = 0x040;
pub const TIOCM_RNG: ::c_int = 0x080;
pub const TIOCM_DSR: ::c_int = 0x100;

pub const VTIME: usize = 5;
pub const VSWTC: usize = 7;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VSUSP: usize = 10;
pub const VREPRINT: usize = 12;
pub const VDISCARD: usize = 13;
pub const VWERASE: usize = 14;
pub const OLCUC: ::tcflag_t = 0o000002;
pub const ONLCR: ::tcflag_t = 0o000004;
pub const NLDLY: ::tcflag_t = 0o000400;
pub const CRDLY: ::tcflag_t = 0o003000;
pub const CR1: ::tcflag_t = 0x00000200;
pub const CR2: ::tcflag_t = 0x00000400;
pub const CR3: ::tcflag_t = 0x00000600;
pub const TABDLY: ::tcflag_t = 0o014000;
pub const TAB1: ::tcflag_t = 0x00000800;
pub const TAB2: ::tcflag_t = 0x00001000;
pub const TAB3: ::tcflag_t = 0x00001800;
pub const BSDLY: ::tcflag_t = 0o020000;
pub const BS1: ::tcflag_t = 0x00002000;
pub const FFDLY: ::tcflag_t = 0o100000;
pub const FF1: ::tcflag_t = 0x00008000;
pub const VTDLY: ::tcflag_t = 0o040000;
pub const VT1: ::tcflag_t = 0x00004000;
pub const XTABS: ::tcflag_t = 0o014000;

pub const CBAUD: ::speed_t = 0o010017;
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
pub const CSIZE: ::tcflag_t = 0o000060;
pub const CS6: ::tcflag_t = 0o000020;
pub const CS7: ::tcflag_t = 0o000040;
pub const CS8: ::tcflag_t = 0o000060;
pub const CSTOPB: ::tcflag_t = 0o000100;
pub const CREAD: ::tcflag_t = 0o000200;
pub const PARENB: ::tcflag_t = 0o000400;
pub const PARODD: ::tcflag_t = 0o001000;
pub const HUPCL: ::tcflag_t = 0o002000;
pub const CLOCAL: ::tcflag_t = 0o004000;
pub const CBAUDEX: ::tcflag_t = 0o010000;
pub const BOTHER: ::speed_t = 0o010000;
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
pub const CIBAUD: ::tcflag_t = 0o02003600000;

pub const ISIG: ::tcflag_t = 0o000001;
pub const ICANON: ::tcflag_t = 0o000002;
pub const XCASE: ::tcflag_t = 0o000004;
pub const ECHOE: ::tcflag_t = 0o000020;
pub const ECHOK: ::tcflag_t = 0o000040;
pub const ECHONL: ::tcflag_t = 0o000100;
pub const NOFLSH: ::tcflag_t = 0o000200;
pub const ECHOCTL: ::tcflag_t = 0o001000;
pub const ECHOPRT: ::tcflag_t = 0o002000;
pub const ECHOKE: ::tcflag_t = 0o004000;
pub const PENDIN: ::tcflag_t = 0o040000;

pub const POLLWRNORM: ::c_short = 0x100;
pub const POLLWRBAND: ::c_short = 0x200;

pub const IXON: ::tcflag_t = 0o002000;
pub const IXOFF: ::tcflag_t = 0o010000;

#[link(name = "util")]
extern "C" {

    pub fn sysctl(
        name: *mut ::c_int,
        namelen: ::c_int,
        oldp: *mut ::c_void,
        oldlenp: *mut ::size_t,
        newp: *mut ::c_void,
        newlen: ::size_t,
    ) -> ::c_int;
    pub fn getcontext(ucp: *mut ::ucontext_t) -> ::c_int;
    pub fn setcontext(ucp: *const ::ucontext_t) -> ::c_int;
    pub fn makecontext(
        ucp: *mut ::ucontext_t,
        func: extern "C" fn(),
        argc: ::c_int,
        ...
    );
    pub fn swapcontext(
        uocp: *mut ::ucontext_t,
        ucp: *const ::ucontext_t,
    ) -> ::c_int;
}
