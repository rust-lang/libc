use crate::prelude::*;

pub type wchar_t = c_int;
pub type nlink_t = c_ulong;
pub type blksize_t = c_long;

pub type fsblkcnt64_t = u64;
pub type fsfilcnt64_t = u64;

s! {
    pub struct cmsghdr {
        pub cmsg_len: crate::size_t,
        pub cmsg_level: c_int,
        pub cmsg_type: c_int,
    }

    pub struct msghdr {
        pub msg_name: *mut c_void,
        pub msg_namelen: crate::socklen_t,
        pub msg_iov: *mut crate::iovec,
        pub msg_iovlen: crate::size_t,
        pub msg_control: *mut c_void,
        pub msg_controllen: crate::size_t,
        pub msg_flags: c_int,
    }

    pub struct stat {
        pub st_dev: c_ulong,
        pub st_ino: crate::ino_t,
        pub st_nlink: nlink_t,
        pub st_mode: crate::mode_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        __pad0: c_int,
        pub st_rdev: c_ulong,
        pub st_size: crate::off_t,
        pub st_blksize: crate::blksize_t,
        pub st_blocks: crate::blkcnt64_t,
        pub st_atim: crate::timespec,
        pub st_mtim: crate::timespec,
        pub st_ctim: crate::timespec,
        __uclibc_unused: [c_long; 3],
    }

    pub struct stat64 {
        pub st_dev: c_ulong,
        pub st_ino: crate::ino_t,
        pub st_nlink: nlink_t,
        pub st_mode: crate::mode_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        __pad0: c_int,
        pub st_rdev: c_ulong,
        pub st_size: crate::off_t,
        pub st_blksize: crate::blksize_t,
        pub st_blocks: crate::blkcnt64_t,
        pub st_atim: crate::timespec,
        pub st_mtim: crate::timespec,
        pub st_ctim: crate::timespec,
        st_pad4: [c_long; 3],
    }

    pub struct sysinfo {
        pub uptime: c_long,
        pub loads: [c_ulong; 3],
        pub totalram: c_ulong,
        pub freeram: c_ulong,
        pub sharedram: c_ulong,
        pub bufferram: c_ulong,
        pub totalswap: c_ulong,
        pub freeswap: c_ulong,
        pub procs: c_ushort,
        pub pad: c_ushort,
        pub totalhigh: c_ulong,
        pub freehigh: c_ulong,
        pub mem_unit: c_uint,
        pub _f: [c_char; 0],
    }

    pub struct statfs {
        pub f_type: c_long,
        pub f_bsize: c_long,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_files: crate::fsfilcnt_t,
        pub f_ffree: crate::fsfilcnt_t,
        pub f_fsid: crate::fsid_t,
        pub f_namelen: c_long,
        pub f_frsize: c_long,
        pub f_flags: c_long,
        f_spare: [c_long; 4],
    }

    pub struct statfs64 {
        pub f_type: c_long,
        pub f_bsize: c_long,
        pub f_blocks: crate::fsblkcnt64_t,
        pub f_bfree: crate::fsblkcnt64_t,
        pub f_bavail: crate::fsblkcnt64_t,
        pub f_files: crate::fsfilcnt64_t,
        pub f_ffree: crate::fsfilcnt64_t,
        pub f_fsid: crate::fsid_t,
        pub f_namelen: c_long,
        pub f_frsize: c_long,
        pub f_flags: c_long,
        pub f_spare: [c_long; 4],
    }

    pub struct statvfs64 {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: crate::fsfilcnt64_t,
        pub f_bfree: crate::fsfilcnt64_t,
        pub f_bavail: crate::fsfilcnt64_t,
        pub f_files: crate::fsfilcnt64_t,
        pub f_ffree: crate::fsfilcnt64_t,
        pub f_favail: crate::fsfilcnt64_t,
        pub f_fsid: c_ulong,
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
        pub __f_spare: [c_int; 6],
    }

    pub struct sigset_t {
        __val: [c_ulong; 1],
    }

    #[repr(align(8))]
    pub struct siginfo_t {
        pub si_signo: c_int,
        pub si_errno: c_int,
        pub si_code: c_int,
        pub _pad: [c_int; 28],
    }

    pub struct stack_t {
        pub ss_sp: *mut c_void,
        pub ss_flags: c_int,
        pub ss_size: crate::size_t,
    }

    pub struct ipc_perm {
        pub __key: crate::key_t,
        pub uid: crate::uid_t,
        pub gid: crate::gid_t,
        pub cuid: crate::uid_t,
        pub cgid: crate::gid_t,
        pub mode: c_uint,
        pub __seq: c_ushort,
        pub __pad2: c_ushort,
        pub __uclibc_unused1: c_ulong,
        pub __uclibc_unused2: c_ulong,
    }

    pub struct shmid_ds {
        pub shm_perm: crate::ipc_perm,
        pub shm_segsz: crate::size_t,
        pub shm_atime: crate::time_t,
        pub shm_dtime: crate::time_t,
        pub shm_ctime: crate::time_t,
        pub shm_cpid: crate::pid_t,
        pub shm_lpid: crate::pid_t,
        pub shm_nattch: crate::shmatt_t,
        __unused4: c_ulong,
        __unused5: c_ulong,
    }

    pub struct __sched_param {
        __sched_priority: c_int,
    }

    #[repr(align(8))]
    pub struct sem_t {
        __size: [c_char; 32],
    }
}

// constants
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_COND_T: usize = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;

pub const __SIZEOF_PTHREAD_ATTR_T: usize = 36;
pub const __SIZEOF_PTHREAD_COND_COMPAT_T: usize = 12;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 32;

pub const O_DIRECTORY: c_int = 0o200000;
pub const O_NOFOLLOW: c_int = 0o400000;
pub const O_DIRECT: c_int = 0o40000;
pub const O_TMPFILE: c_int = 0o20200000;
pub const O_LARGEFILE: c_int = 0;

pub const CPU_SETSIZE: c_int = 0x400;
pub const RTLD_GLOBAL: c_int = 0x00100;
pub const TCSADRAIN: c_int = 0x1;
pub const TCSAFLUSH: c_int = 0x2;
pub const TCSANOW: c_int = 0;

pub const MINSIGSTKSZ: c_int = 2048;
pub const SIGSTKSZ: crate::size_t = 8192;

pub const FIOQSIZE: crate::Ioctl = 0x5460;
