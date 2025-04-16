use crate::prelude::*;

pub type regoff_t = c_int;
pub type __rlimit_resource_t = c_int;
pub type __priority_which_t = c_uint;

cfg_if! {
    if #[cfg(doc)] {
        pub(crate) type Ioctl = c_ulong;
    } else {
        #[doc(hidden)]
        pub type Ioctl = c_ulong;
    }
}

s! {
    pub struct statvfs {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_files: crate::fsfilcnt_t,
        pub f_ffree: crate::fsfilcnt_t,
        pub f_favail: crate::fsfilcnt_t,
        #[cfg(target_endian = "little")]
        pub f_fsid: c_ulong,
        #[cfg(target_pointer_width = "32")]
        __f_unused: c_int,
        #[cfg(target_endian = "big")]
        pub f_fsid: c_ulong,
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
        __f_spare: [c_int; 6],
    }

    pub struct regex_t {
        __buffer: *mut c_void,
        __allocated: size_t,
        __used: size_t,
        __syntax: c_ulong,
        __fastmap: *mut c_char,
        __translate: *mut c_char,
        __re_nsub: size_t,
        __bitfield: u8,
    }

    pub struct rtentry {
        pub rt_pad1: c_ulong,
        pub rt_dst: crate::sockaddr,
        pub rt_gateway: crate::sockaddr,
        pub rt_genmask: crate::sockaddr,
        pub rt_flags: c_ushort,
        pub rt_pad2: c_short,
        pub rt_pad3: c_ulong,
        pub rt_tos: c_uchar,
        pub rt_class: c_uchar,
        #[cfg(target_pointer_width = "64")]
        pub rt_pad4: [c_short; 3usize],
        #[cfg(not(target_pointer_width = "64"))]
        pub rt_pad4: c_short,
        pub rt_metric: c_short,
        pub rt_dev: *mut c_char,
        pub rt_mtu: c_ulong,
        pub rt_window: c_ulong,
        pub rt_irtt: c_ushort,
    }

    pub struct __exit_status {
        pub e_termination: c_short,
        pub e_exit: c_short,
    }

    pub struct sigaction {
        pub sa_handler: crate::sighandler_t,
        pub sa_flags: c_ulong,
        pub sa_restorer: Option<extern "C" fn()>,
        pub sa_mask: crate::sigset_t,
    }
}

impl siginfo_t {
    pub unsafe fn si_addr(&self) -> *mut c_void {
        #[repr(C)]
        struct siginfo_sigfault {
            _si_signo: c_int,
            _si_errno: c_int,
            _si_code: c_int,
            si_addr: *mut c_void,
        }
        (*(self as *const siginfo_t as *const siginfo_sigfault)).si_addr
    }

    pub unsafe fn si_value(&self) -> crate::sigval {
        #[repr(C)]
        struct siginfo_si_value {
            _si_signo: c_int,
            _si_errno: c_int,
            _si_code: c_int,
            _si_timerid: c_int,
            _si_overrun: c_int,
            si_value: crate::sigval,
        }
        (*(self as *const siginfo_t as *const siginfo_si_value)).si_value
    }
}

// Internal, for casts to access union fields
#[repr(C)]
struct sifields_sigchld {
    si_pid: crate::pid_t,
    si_uid: crate::uid_t,
    si_status: c_int,
    si_utime: c_long,
    si_stime: c_long,
}
impl Copy for sifields_sigchld {}
impl Clone for sifields_sigchld {
    fn clone(&self) -> sifields_sigchld {
        *self
    }
}

// Internal, for casts to access union fields
#[repr(C)]
union sifields {
    _align_pointer: *mut c_void,
    sigchld: sifields_sigchld,
}

// Internal, for casts to access union fields. Note that some variants
// of sifields start with a pointer, which makes the alignment of
// sifields vary on 32-bit and 64-bit architectures.
#[repr(C)]
struct siginfo_f {
    _siginfo_base: [c_int; 3],
    sifields: sifields,
}

impl siginfo_t {
    unsafe fn sifields(&self) -> &sifields {
        &(*(self as *const siginfo_t as *const siginfo_f)).sifields
    }

    pub unsafe fn si_pid(&self) -> crate::pid_t {
        self.sifields().sigchld.si_pid
    }

    pub unsafe fn si_uid(&self) -> crate::uid_t {
        self.sifields().sigchld.si_uid
    }

    pub unsafe fn si_status(&self) -> c_int {
        self.sifields().sigchld.si_status
    }

    pub unsafe fn si_utime(&self) -> c_long {
        self.sifields().sigchld.si_utime
    }

    pub unsafe fn si_stime(&self) -> c_long {
        self.sifields().sigchld.si_stime
    }
}

pub const MCL_CURRENT: c_int = 0x0001;
pub const MCL_FUTURE: c_int = 0x0002;
pub const MCL_ONFAULT: c_int = 0x0004;

pub const SIGEV_THREAD_ID: c_int = 4;

pub const AF_VSOCK: c_int = 40;

pub const POSIX_FADV_DONTNEED: c_int = 4;
pub const POSIX_FADV_NOREUSE: c_int = 5;

// These are different than GNU!
pub const LC_CTYPE: c_int = 0;
pub const LC_NUMERIC: c_int = 1;
pub const LC_TIME: c_int = 3;
pub const LC_COLLATE: c_int = 4;
pub const LC_MONETARY: c_int = 2;
pub const LC_MESSAGES: c_int = 5;
pub const LC_ALL: c_int = 6;
// end different section

// MS_ flags for mount(2)
pub const MS_RMT_MASK: c_ulong =
    crate::MS_RDONLY | crate::MS_SYNCHRONOUS | crate::MS_MANDLOCK | crate::MS_I_VERSION;

pub const IPV6_JOIN_GROUP: c_int = 20;
pub const IPV6_LEAVE_GROUP: c_int = 21;

pub const CODESET: crate::nl_item = 10;
pub const CRNCYSTR: crate::nl_item = 0x215;
pub const RADIXCHAR: crate::nl_item = 0x100;
pub const THOUSEP: crate::nl_item = 0x101;
pub const YESEXPR: crate::nl_item = 0x500;
pub const NOEXPR: crate::nl_item = 0x501;
pub const YESSTR: crate::nl_item = 0x502;
pub const NOSTR: crate::nl_item = 0x503;

pub const TMP_MAX: c_int = 238328;

pub const FILENAME_MAX: c_uint = 4095;

pub const PRIO_PROCESS: c_int = 0;
pub const PRIO_PGRP: c_int = 1;
pub const PRIO_USER: c_int = 2;

pub const ST_RELATIME: c_ulong = 4096;

pub const AF_NFC: c_int = PF_NFC;
pub const BUFSIZ: c_int = 256;
pub const FOPEN_MAX: c_int = 16;
pub const F_GETOWN: c_int = 9;
pub const F_RDLCK: c_int = 0;
pub const F_SETOWN: c_int = 8;
pub const F_UNLCK: c_int = 2;
pub const F_WRLCK: c_int = 1;
pub const MAP_HUGE_SHIFT: c_int = 26;
pub const MAP_HUGE_MASK: c_int = 0x3f;
pub const NI_MAXHOST: crate::socklen_t = 1025;
pub const PF_NFC: c_int = 39;
pub const PF_VSOCK: c_int = 40;
pub const RTLD_NOLOAD: c_int = 0x00004;
pub const RUSAGE_THREAD: c_int = 1;
pub const TCP_COOKIE_TRANSACTIONS: c_int = 15;

pub const PTHREAD_RWLOCK_INITIALIZER: crate::pthread_rwlock_t = crate::pthread_rwlock_t {
    size: {
        let mut arr = [0; __SIZEOF_PTHREAD_RWLOCK_T];
        arr[__SIZEOF_PTHREAD_RWLOCK_T - 8] = 1;
        arr
    },
};

extern "C" {
    pub fn gettimeofday(tp: *mut crate::timeval, tz: *mut crate::timezone) -> c_int;

    pub fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;

    pub fn pthread_rwlockattr_getkind_np(
        attr: *const crate::pthread_rwlockattr_t,
        val: *mut c_int,
    ) -> c_int;
    pub fn pthread_rwlockattr_setkind_np(
        attr: *mut crate::pthread_rwlockattr_t,
        val: c_int,
    ) -> c_int;

    pub fn openpty(
        amaster: *mut c_int,
        aslave: *mut c_int,
        name: *mut c_char,
        termp: *mut crate::termios,
        winp: *mut crate::winsize,
    ) -> c_int;

    pub fn getnameinfo(
        sa: *const crate::sockaddr,
        salen: crate::socklen_t,
        host: *mut c_char,
        hostlen: crate::socklen_t,
        serv: *mut c_char,
        servlen: crate::socklen_t,
        flags: c_int,
    ) -> c_int;

    pub fn getrlimit64(resource: crate::__rlimit_resource_t, rlim: *mut crate::rlimit64) -> c_int;
    pub fn setrlimit64(resource: crate::__rlimit_resource_t, rlim: *const crate::rlimit64)
        -> c_int;
    pub fn getrlimit(resource: crate::__rlimit_resource_t, rlim: *mut crate::rlimit) -> c_int;
    pub fn setrlimit(resource: crate::__rlimit_resource_t, rlim: *const crate::rlimit) -> c_int;
    pub fn getauxval(type_: c_ulong) -> c_ulong;
}

mod l4re;
pub use self::l4re::*;

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    } else if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else {
        pub use unsupported_target;
    }
}
