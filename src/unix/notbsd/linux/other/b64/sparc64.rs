//! SPARC64-specific definitions for 64-bit linux-like values

pub type c_char = i8;
pub type wchar_t = i32;
pub type nlink_t = u32;
pub type blksize_t = i64;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        __pad0: u64,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad1: u64,
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
        __pad0: u64,
        pub st_ino: ::ino64_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad2: ::c_int,
        pub st_size: ::off64_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt64_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __reserved: [::c_long; 2],
    }

    pub struct pthread_attr_t {
        __size: [u64; 7]
    }

    pub struct ipc_perm {
        pub __key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::mode_t,
        __pad0: u16,
        pub __seq: ::c_ushort,
        __unused1: ::c_ulonglong,
        __unused2: ::c_ulonglong,
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_atime: ::time_t,
        pub shm_dtime: ::time_t,
        pub shm_ctime: ::time_t,
        pub shm_segsz: ::size_t,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        __reserved1: ::c_ulong,
        __reserved2: ::c_ulong
    }
}

pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;

pub const O_DIRECTORY: ::c_int = 0o200000;
pub const O_NOFOLLOW: ::c_int = 0o400000;
pub const O_DIRECT: ::c_int = 0x100000;

pub const MAP_LOCKED: ::c_int = 0x0100;
pub const MAP_NORESERVE: ::c_int = 0x00040;

pub const EDEADLOCK: ::c_int = 108;

pub const SO_PEERCRED: ::c_int = 0x40;
pub const SO_RCVLOWAT: ::c_int = 0x800;
pub const SO_SNDLOWAT: ::c_int = 0x1000;
pub const SO_RCVTIMEO: ::c_int = 0x2000;
pub const SO_SNDTIMEO: ::c_int = 0x4000;

pub const FIOCLEX: ::c_ulong = 0x20006601;
pub const FIONBIO: ::c_ulong = 0x8004667e;

pub const SYS_gettid: ::c_long = 143;
pub const SYS_perf_event_open: ::c_long = 327;

pub const MCL_CURRENT: ::c_int = 0x2000;
pub const MCL_FUTURE: ::c_int = 0x4000;

pub const SIGSTKSZ: ::size_t = 16384;
pub const CBAUD: ::tcflag_t = 0x0000100f;
pub const TAB1: ::c_int = 0x800;
pub const TAB2: ::c_int = 0x1000;
pub const TAB3: ::c_int = 0x1800;
pub const CR1: ::c_int  = 0x200;
pub const CR2: ::c_int  = 0x400;
pub const CR3: ::c_int  = 0x600;
pub const FF1: ::c_int  = 0x8000;
pub const BS1: ::c_int  = 0x2000;
pub const VT1: ::c_int  = 0x4000;
pub const VWERASE: usize = 0xe;
pub const VREPRINT: usize = 0xc;
pub const VSUSP: usize = 0xa;
pub const VSTART: usize = 0x8;
pub const VSTOP: usize = 0x9;
pub const VDISCARD: usize = 0xd;
pub const VTIME: usize = 0x5;
pub const IXON: ::tcflag_t = 0x400;
pub const IXOFF: ::tcflag_t = 0x1000;
pub const ONLCR: ::tcflag_t = 0x4;
pub const CSIZE: ::tcflag_t = 0x30;
pub const CS6: ::tcflag_t = 0x10;
pub const CS7: ::tcflag_t = 0x20;
pub const CS8: ::tcflag_t = 0x30;
pub const CSTOPB: ::tcflag_t = 0x40;
pub const CREAD: ::tcflag_t = 0x80;
pub const PARENB: ::tcflag_t = 0x100;
pub const PARODD: ::tcflag_t = 0x200;
pub const HUPCL: ::tcflag_t = 0x400;
pub const CLOCAL: ::tcflag_t = 0x800;
pub const ECHOKE: ::tcflag_t = 0x800;
pub const ECHOE: ::tcflag_t = 0x10;
pub const ECHOK: ::tcflag_t = 0x20;
pub const ECHONL: ::tcflag_t = 0x40;
pub const ECHOPRT: ::tcflag_t = 0x400;
pub const ECHOCTL: ::tcflag_t = 0x200;
pub const ISIG: ::tcflag_t = 0x1;
pub const ICANON: ::tcflag_t = 0x2;
pub const PENDIN: ::tcflag_t = 0x4000;
pub const NOFLSH: ::tcflag_t = 0x80;

pub const VEOL: usize = 5;
pub const VEOL2: usize = 6;
pub const VMIN: usize = 4;
pub const IEXTEN: ::tcflag_t = 0x8000;
pub const TOSTOP: ::tcflag_t = 0x100;
pub const FLUSHO: ::tcflag_t = 0x2000;
pub const EXTPROC: ::tcflag_t = 0x10000;
pub const TCGETS: ::c_ulong = 0x40245408;
pub const TCSETS: ::c_ulong = 0x80245409;
pub const TCSETSW: ::c_ulong = 0x8024540a;
pub const TCSETSF: ::c_ulong = 0x8024540b;
pub const TCGETA: ::c_ulong = 0x40125401;
pub const TCSETA: ::c_ulong = 0x80125402;
pub const TCSETAW: ::c_ulong = 0x80125403;
pub const TCSETAF: ::c_ulong = 0x80125404;
pub const TCSBRK: ::c_ulong = 0x20005405;
pub const TCXONC: ::c_ulong = 0x20005406;
pub const TCFLSH: ::c_ulong = 0x20005407;
pub const TIOCINQ: ::c_ulong = 0x4004667f;
pub const TIOCGPGRP: ::c_ulong = 0x40047483;
pub const TIOCSPGRP: ::c_ulong = 0x80047482;
pub const TIOCOUTQ: ::c_ulong = 0x40047473;
pub const TIOCGWINSZ: ::c_ulong = 0x40087468;
pub const TIOCSWINSZ: ::c_ulong = 0x80087467;
pub const FIONREAD: ::c_ulong = 0x4004667f;
