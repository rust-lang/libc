pub type blkcnt_t = i64;
pub type blksize_t = i64;
pub type c_char = u8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;
pub type ino_t = u64;
pub type nlink_t = u64;
pub type off_t = i64;
pub type rlim_t = u64;
pub type suseconds_t = i64;
pub type time_t = i64;
pub type wchar_t = i32;

s! {
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
        __glibc_reserved: [::c_int; 5],
    }

    pub struct stat64 {
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
        __glibc_reserved: [::c_int; 5],
    }

    pub struct pthread_attr_t {
        __size: [::c_ulong; 7]
    }

    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_flags: ::c_ulong,
        _restorer: *mut ::c_void,
        pub sa_mask: sigset_t,
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: ::size_t,
    }

    pub struct sigset_t {
        __size: [::c_ulong; 16],
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        _pad: ::c_int,
        _pad2: [::c_long; 14],
    }

    pub struct ipc_perm {
        pub __key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::c_uint,
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

    pub struct statfs {
        pub f_type: ::c_uint,
        pub f_bsize: ::c_uint,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsblkcnt_t,
        pub f_ffree: ::fsblkcnt_t,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::c_uint,
        pub f_frsize: ::c_uint,
        pub f_flags: ::c_uint,
        f_spare: [::c_uint; 4],
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

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; ::NCCS],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }

    pub struct sysinfo {
        pub uptime: ::c_long,
        pub loads: [::c_ulong; 3],
        pub totalram: ::c_ulong,
        pub freeram: ::c_ulong,
        pub sharedram: ::c_ulong,
        pub bufferram: ::c_ulong,
        pub totalswap: ::c_ulong,
        pub freeswap: ::c_ulong,
        pub procs: ::c_ushort,
        pub pad: ::c_ushort,
        pub totalhigh: ::c_ulong,
        pub freehigh: ::c_ulong,
        pub mem_unit: ::c_uint,
        pub _f: [::c_char; 0],
    }

    // FIXME this is actually a union
    pub struct sem_t {
        __size: [::c_char; 32],
        __align: [::c_long; 0],
    }
}

pub const POSIX_FADV_DONTNEED: ::c_int = 6;
pub const POSIX_FADV_NOREUSE: ::c_int = 7;

pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56;

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
pub const FIONBIO: ::c_ulong = 0x5421;
pub const MAP_ANON: ::c_int = 0x20;
pub const O_ACCMODE: ::c_int = 3;
pub const O_APPEND: ::c_int = 1024;
pub const O_CREAT: ::c_int = 64;
pub const O_EXCL: ::c_int = 128;
pub const O_NONBLOCK: ::c_int = 2048;
pub const PTHREAD_STACK_MIN: ::size_t = 16384;
pub const RLIM_INFINITY: ::rlim_t = 0xffffffffffffffff;
pub const SA_ONSTACK: ::c_ulong = 0x08000000;
pub const SA_SIGINFO: ::c_ulong = 4;
pub const SIGBUS: ::c_int = 7;
pub const SIGSTKSZ: ::size_t = 0x2000;
pub const SIG_SETMASK: ::c_int = 2;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_STREAM: ::c_int = 1;
pub const SOL_SOCKET: ::c_int = 1;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_ERROR: ::c_int = 4;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_REUSEADDR: ::c_int = 2;
pub const SO_SNDTIMEO: ::c_int = 21;

#[link(name = "util")]
extern {
    pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
}
