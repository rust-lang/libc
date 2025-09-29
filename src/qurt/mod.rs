//! Interface to QuRT (Qualcomm Real-Time OS) C library

use core::ptr::null_mut;

use crate::prelude::*;

// Forward declarations for opaque types
#[derive(Debug)]
pub enum DIR {}
impl Copy for DIR {}
impl Clone for DIR {
    fn clone(&self) -> DIR {
        *self
    }
}

// Basic integer types
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type size_t = crate::uintptr_t;
pub type ssize_t = intptr_t;

// Process and system types
pub type pid_t = c_int;
pub type uid_t = c_uint;
pub type gid_t = c_uint;

// Time types
pub type time_t = c_longlong;
pub type suseconds_t = c_long;
pub type useconds_t = c_ulong;
pub type clockid_t = c_int;
pub type timer_t = *mut c_void;

// File system types
pub type dev_t = c_ulong;
pub type ino_t = c_ulong;
pub type mode_t = c_uint;
pub type nlink_t = c_uint;
pub type off_t = c_long;
pub type blkcnt_t = c_long;
pub type blksize_t = c_long;

// Thread types based on QuRT pthread implementation
pub type pthread_t = c_uint;
pub type pthread_key_t = c_int;
pub type pthread_once_t = c_int;
pub type pthread_mutex_t = c_uint;
pub type pthread_cond_t = c_uint;
pub type pthread_spinlock_t = c_uint;
pub type pthread_barrier_t = c_uint;

// Signal types
pub type sigset_t = c_ulong;

// Network types
pub type socklen_t = c_uint;
pub type in_addr_t = u32;

// Error type
pub type errno_t = c_int;

// Resource limit type
pub type rlim_t = c_ulong;

// Terminal types
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;

// File descriptor set type for select()
pub type fd_set = c_ulong;

// Standard C library types
pub type FILE = c_void;
pub type fpos_t = c_long;
pub type clock_t = c_long;

// POSIX semaphore types
pub type sem_t = c_uint;

// Message queue types
pub type mqd_t = c_int;

// Scheduling types
pub type sched_param = c_int;

// Additional file system types
pub type nfds_t = c_ulong;

// Architecture-specific modules
cfg_if! {
    if #[cfg(target_arch = "hexagon")] {
        mod hexagon;
        pub use self::hexagon::*;
    } else {
        // Add other architectures as needed
    }
}

// Constants from QuRT headers

// POSIX error codes (from errno.h and QuRT qurt_error.h)
pub const EOK: c_int = 0;
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const ENOTBLK: c_int = 15;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOTTY: c_int = 25;
pub const ETXTBSY: c_int = 26;
pub const EFBIG: c_int = 27;
pub const ENOSPC: c_int = 28;
pub const ESPIPE: c_int = 29;
pub const EROFS: c_int = 30;
pub const EMLINK: c_int = 31;
pub const EPIPE: c_int = 32;
pub const EDOM: c_int = 33;
pub const ERANGE: c_int = 34;
pub const EDEADLK: c_int = 35;
pub const ENAMETOOLONG: c_int = 36;
pub const ENOLCK: c_int = 37;
pub const ENOSYS: c_int = 38;
pub const ENOTEMPTY: c_int = 39;
pub const ELOOP: c_int = 40;
pub const EWOULDBLOCK: c_int = EAGAIN;
pub const ENOMSG: c_int = 42;
pub const EIDRM: c_int = 43;
pub const ECHRNG: c_int = 44;
pub const EL2NSYNC: c_int = 45;
pub const EL3HLT: c_int = 46;
pub const EL3RST: c_int = 47;
pub const ELNRNG: c_int = 48;
pub const EUNATCH: c_int = 49;
pub const ENOCSI: c_int = 50;
pub const EL2HLT: c_int = 51;
pub const EBADE: c_int = 52;
pub const EBADR: c_int = 53;
pub const EXFULL: c_int = 54;
pub const ENOANO: c_int = 55;
pub const EBADRQC: c_int = 56;
pub const EBADSLT: c_int = 57;
pub const EDEADLOCK: c_int = EDEADLK;
pub const EBFONT: c_int = 59;
pub const ENOSTR: c_int = 60;
pub const ENODATA: c_int = 61;
pub const ETIME: c_int = 62;
pub const ENOSR: c_int = 63;
pub const ENONET: c_int = 64;
pub const ENOPKG: c_int = 65;
pub const EREMOTE: c_int = 66;
pub const ENOLINK: c_int = 67;
pub const EADV: c_int = 68;
pub const ESRMNT: c_int = 69;
pub const ECOMM: c_int = 70;
pub const EPROTO: c_int = 71;
pub const EMULTIHOP: c_int = 72;
pub const EDOTDOT: c_int = 73;
pub const EBADMSG: c_int = 74;
pub const EOVERFLOW: c_int = 75;
pub const ENOTUNIQ: c_int = 76;
pub const EBADFD: c_int = 77;
pub const EREMCHG: c_int = 78;
pub const ELIBACC: c_int = 79;
pub const ELIBBAD: c_int = 80;
pub const ELIBSCN: c_int = 81;
pub const ELIBMAX: c_int = 82;
pub const ELIBEXEC: c_int = 83;
pub const EILSEQ: c_int = 84;
pub const ERESTART: c_int = 85;
pub const ESTRPIPE: c_int = 86;
pub const EUSERS: c_int = 87;
pub const ENOTSOCK: c_int = 88;
pub const EDESTADDRREQ: c_int = 89;
pub const EMSGSIZE: c_int = 90;
pub const EPROTOTYPE: c_int = 91;
pub const ENOPROTOOPT: c_int = 92;
pub const EPROTONOSUPPORT: c_int = 93;
pub const ESOCKTNOSUPPORT: c_int = 94;
pub const EOPNOTSUPP: c_int = 95;
pub const EPFNOSUPPORT: c_int = 96;
pub const EAFNOSUPPORT: c_int = 97;
pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const ENETDOWN: c_int = 100;
pub const ENETUNREACH: c_int = 101;
pub const ENETRESET: c_int = 102;
pub const ECONNABORTED: c_int = 103;
pub const ECONNRESET: c_int = 104;
pub const ENOBUFS: c_int = 105;
pub const EISCONN: c_int = 106;
pub const ENOTCONN: c_int = 107;
pub const ESHUTDOWN: c_int = 108;
pub const ETOOMANYREFS: c_int = 109;
pub const ETIMEDOUT: c_int = 110;
pub const ECONNREFUSED: c_int = 111;
pub const EHOSTDOWN: c_int = 112;
pub const EHOSTUNREACH: c_int = 113;
pub const EALREADY: c_int = 114;
pub const EINPROGRESS: c_int = 115;
pub const ESTALE: c_int = 116;
pub const EUCLEAN: c_int = 117;
pub const ENOTNAM: c_int = 118;
pub const ENAVAIL: c_int = 119;
pub const EISNAM: c_int = 120;
pub const EREMOTEIO: c_int = 121;
pub const EDQUOT: c_int = 122;
pub const ENOMEDIUM: c_int = 123;
pub const EMEDIUMTYPE: c_int = 124;
pub const ECANCELED: c_int = 125;
pub const ENOKEY: c_int = 126;
pub const EKEYEXPIRED: c_int = 127;
pub const EKEYREVOKED: c_int = 128;
pub const EKEYREJECTED: c_int = 129;
pub const EOWNERDEAD: c_int = 130;
pub const ENOTRECOVERABLE: c_int = 131;

// Signal constants (from signal.h)
pub const POSIX_MSG: c_int = 7;
pub const POSIX_NOTIF: c_int = 8;
pub const SIGKILL: c_int = 9;
pub const SIGRTMIN: c_int = 10;
pub const SIGRTMAX: c_int = 32;

// Signal event types
pub const SIGEV_NONE: c_int = 0;
pub const SIGEV_SIGNAL: c_int = 1;
pub const SIGEV_THREAD: c_int = 2;

// Signal action flags
pub const SA_SIGINFO: c_int = 1;

// Signal mask operations
pub const SIG_BLOCK: c_int = 1;
pub const SIG_UNBLOCK: c_int = 2;
pub const SIG_SETMASK: c_int = 3;

// Signal handler constants
pub const SIG_DFL: usize = 0;
pub const SIG_IGN: usize = 1;
pub const SIG_ERR: usize = !0;

// File control constants (from fcntl.h)
pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
pub const O_ACCMODE: c_int = 0x3;
pub const O_CREAT: c_int = 0x100;
pub const O_EXCL: c_int = 0x200;
pub const O_NOCTTY: c_int = 0x400;
pub const O_TRUNC: c_int = 0x1000;
pub const O_APPEND: c_int = 0x2000;
pub const O_NONBLOCK: c_int = 0x4000;
pub const O_SYNC: c_int = 0x1000;
pub const O_DSYNC: c_int = 0x1000;
pub const O_RSYNC: c_int = 0x1000;

// fcntl commands
pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const F_GETLK: c_int = 5;
pub const F_SETLK: c_int = 6;
pub const F_SETLKW: c_int = 7;

// File descriptor flags
pub const FD_CLOEXEC: c_int = 1;

// File locking types
pub const F_RDLCK: c_int = 0;
pub const F_WRLCK: c_int = 1;
pub const F_UNLCK: c_int = 2;

// AT_* constants for *at() functions
pub const AT_FDCWD: c_int = -100;
pub const AT_EACCESS: c_int = 0x200;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const AT_SYMLINK_FOLLOW: c_int = 0x400;
pub const AT_REMOVEDIR: c_int = 0x200;

// Pthread constants (from pthread_types.h)
pub const PTHREAD_MAX_THREADS: c_uint = 512;
pub const PTHREAD_NAME_LEN: c_int = 16;
pub const PTHREAD_MIN_STACKSIZE: c_int = 512;
pub const PTHREAD_MAX_STACKSIZE: c_int = 1048576;
pub const PTHREAD_DEFAULT_STACKSIZE: c_int = 16384;
pub const PTHREAD_STACK_MIN: c_uint = 8192;
pub const PTHREAD_MIN_PRIORITY: c_uint = 0;
pub const PTHREAD_MAX_PRIORITY: c_uint = 255;
pub const PTHREAD_DEFAULT_PRIORITY: c_int = 1;

// Pthread initialization constants
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = 0xFFFFFFFF;
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = 0xFFFFFFFF;

// Pthread attribute constants
pub const PTHREAD_CREATE_JOINABLE: c_int = 1;
pub const PTHREAD_CREATE_DETACHED: c_int = 0;
pub const PTHREAD_SCOPE_PROCESS: c_int = 1;
pub const PTHREAD_SCOPE_SYSTEM: c_int = 0;
pub const PTHREAD_INHERIT_SCHED: c_int = 1;
pub const PTHREAD_EXPLICIT_SCHED: c_int = 0;

// Mutex constants
pub const PTHREAD_PROCESS_PRIVATE: c_int = 0;
pub const PTHREAD_PROCESS_SHARED: c_int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: c_int = 0;
pub const PTHREAD_MUTEX_NORMAL: c_int = 1;
pub const PTHREAD_MUTEX_RECURSIVE: c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT: c_int = 3;

// Priority constants
pub const PTHREAD_PRIO_NONE: c_int = 0;
pub const PTHREAD_PRIO_INHERIT: c_int = 1;
pub const PTHREAD_PRIO_PROTECT: c_int = 2;

// Spinlock constants
pub const PTHREAD_SPINLOCK_UNLOCKED: c_int = 0;
pub const PTHREAD_SPINLOCK_LOCKED: c_int = 1;

pub const PTHREAD_ONCE_INIT: c_int = 0;

// Clock types (from time.h)
pub const TIME_CONV_SCLK_FREQ: c_int = 19200000;
pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub const CLOCK_REALTIME_COARSE: clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: clockid_t = 6;
pub const CLOCK_BOOTTIME: clockid_t = 7;

// Standard I/O constants (from stdio.h)
pub const BUFSIZ: c_uint = 512;
pub const EOF: c_int = -1;
pub const FOPEN_MAX: c_uint = 20;
pub const FILENAME_MAX: c_uint = 260;
pub const L_tmpnam: c_uint = 260;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const TMP_MAX: c_uint = 25;

// File access constants (from unistd.h)
pub const F_OK: c_int = 0;
pub const X_OK: c_int = 1;
pub const W_OK: c_int = 2;
pub const R_OK: c_int = 4;

// Process constants
pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;

// Standard limits (from limits.h)
pub const CHAR_BIT: c_uint = 8;
pub const CHAR_MAX: c_char = 127;
pub const CHAR_MIN: c_char = -128;
pub const INT_MAX: c_int = 2147483647;
pub const INT_MIN: c_int = -2147483648;
pub const LONG_MAX: c_long = 9223372036854775807;
pub const LONG_MIN: c_long = -9223372036854775808;
pub const SCHAR_MAX: c_schar = 127;
pub const SCHAR_MIN: c_schar = -128;
pub const SHRT_MAX: c_short = 32767;
pub const SHRT_MIN: c_short = -32768;
pub const UCHAR_MAX: c_uchar = 255;
pub const UINT_MAX: c_uint = 4294967295;
pub const ULONG_MAX: c_ulong = 18446744073709551615;
pub const USHRT_MAX: c_ushort = 65535;

// POSIX limits (from limits.h)
pub const PATH_MAX: c_int = 260;
pub const NAME_MAX: c_int = 255;
pub const IOV_MAX: c_int = 16;

// Standard file descriptors
pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

// Memory mapping constants (from sys/mman.h)
pub const PROT_NONE: c_int = 0x00;
pub const PROT_READ: c_int = 0x01;
pub const PROT_WRITE: c_int = 0x02;
pub const PROT_EXEC: c_int = 0x04;

pub const MAP_SHARED: c_int = 0x0001;
pub const MAP_PRIVATE: c_int = 0x0002;
pub const MAP_FIXED: c_int = 0x0010;
pub const MAP_ANON: c_int = 0x1000;
pub const MAP_ANONYMOUS: c_int = MAP_ANON;
pub const MAP_FILE: c_int = 0x0000;
pub const MAP_RENAME: c_int = 0x0020;
pub const MAP_NORESERVE: c_int = 0x0040;
pub const MAP_INHERIT: c_int = 0x0080;
pub const MAP_HASSEMAPHORE: c_int = 0x0200;
pub const MAP_TRYFIXED: c_int = 0x0400;
pub const MAP_WIRED: c_int = 0x0800;

pub const MAP_FAILED: *mut c_void = !0 as *mut c_void;

pub const MS_ASYNC: c_int = 0x01;
pub const MS_INVALIDATE: c_int = 0x02;
pub const MS_SYNC: c_int = 0x04;

pub const MCL_CURRENT: c_int = 0x01;
pub const MCL_FUTURE: c_int = 0x02;

// Dynamic linking constants (from dlfcn.h)
pub const RTLD_LAZY: c_int = 1;
pub const RTLD_NOW: c_int = 2;
pub const RTLD_GLOBAL: c_int = 0x100;
pub const RTLD_LOCAL: c_int = 0x200;

// Semaphore constants
pub const SEM_FAILED: *mut sem_t = 0 as *mut sem_t;

// Structures based on QuRT headers

s! {
    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }

    pub struct timeval {
        pub tv_sec: time_t,
        pub tv_usec: suseconds_t,
    }

    pub struct sigval {
        pub sival_int: c_int,
        pub sival_ptr: *mut c_void,
    }

    pub struct sigevent {
        pub sigev_notify: c_int,
        pub sigev_signo: c_int,
        pub sigev_value: sigval,
        pub sigev_notify_function: Option<extern "C" fn(sigval)>,
        pub sigev_notify_attributes: *mut pthread_attr_t,
    }

    pub struct siginfo_t {
        pub si_signo: c_int,
        pub si_code: c_int,
        pub si_value: sigval,
    }

    pub struct sigaction {
        pub sa_handler: Option<extern "C" fn(c_int)>,
        pub sa_mask: sigset_t,
        pub sa_flags: c_int,
        pub sa_sigaction: Option<extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    }

    pub struct pthread_attr_t {
        pub stackaddr: *mut c_void,
        pub internal_stack: c_int,
        pub stacksize: size_t,
        pub priority: c_int,
        pub timetest_id: c_ushort,
        pub autostack: c_ushort,
        pub bus_priority: c_ushort,
        pub reserved: c_ushort,
        pub cpumask: c_uint,
        pub name: [c_char; 16],
        pub ext_context: c_int,
        pub detachstate: c_int,
    }

    pub struct pthread_mutexattr_t {
        pub is_initialized: c_int,
        pub type_: c_int,
        pub pshared: c_int,
        pub protocol: c_int,
    }

    pub struct pthread_condattr_t {
        pub is_initialized: c_int,
        pub pshared: c_int,
        pub clock_id: clockid_t,
    }

    pub struct pthread_barrierattr_t {
        pub is_initialized: c_int,
        pub pshared: c_int,
    }

    pub struct itimerspec {
        pub it_interval: timespec,
        pub it_value: timespec,
    }

    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_cc: [c_uchar; 32],
        pub c_ispeed: speed_t,
        pub c_ospeed: speed_t,
    }

    pub struct dirent {
        pub d_ino: ino_t,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
    }

    pub struct sched_param {
        pub sched_priority: c_int,
    }

    pub struct iovec {
        pub iov_base: *mut c_void,
        pub iov_len: size_t,
    }

    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }

    pub struct rusage {
        pub ru_utime: timeval,
        pub ru_stime: timeval,
        pub ru_maxrss: c_long,
        pub ru_ixrss: c_long,
        pub ru_idrss: c_long,
        pub ru_isrss: c_long,
        pub ru_minflt: c_long,
        pub ru_majflt: c_long,
        pub ru_nswap: c_long,
        pub ru_inblock: c_long,
        pub ru_oublock: c_long,
        pub ru_msgsnd: c_long,
        pub ru_msgrcv: c_long,
        pub ru_nsignals: c_long,
        pub ru_nvcsw: c_long,
        pub ru_nivcsw: c_long,
    }

    pub struct flock {
        pub l_type: c_short,
        pub l_whence: c_short,
        pub l_start: off_t,
        pub l_len: off_t,
        pub l_pid: pid_t,
    }
}

// Function declarations for QuRT POSIX API
extern "C" {
    // Signal functions
    pub fn sigwait(set: *const sigset_t, sig: *mut c_int) -> c_int;
    pub fn _sigaction(sig: c_int, act: *const sigaction, oact: *mut sigaction) -> c_int;
    pub fn sigsuspend(sigmask: *const sigset_t) -> c_int;
    pub fn sigaddset(set: *mut sigset_t, signo: c_int) -> c_int;
    pub fn sigdelset(set: *mut sigset_t, signo: c_int) -> c_int;
    pub fn sigemptyset(set: *mut sigset_t) -> c_int;
    pub fn sigfillset(set: *mut sigset_t) -> c_int;
    pub fn sigismember(set: *const sigset_t, signo: c_int) -> c_int;
    pub fn sigtimedwait(
        set: *const sigset_t,
        info: *mut siginfo_t,
        timeout: *const timespec,
    ) -> c_int;

    // Thread functions
    pub fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    pub fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    pub fn pthread_detach(thread: pthread_t) -> c_int;
    pub fn pthread_self() -> pthread_t;
    pub fn pthread_equal(t1: pthread_t, t2: pthread_t) -> c_int;
    pub fn pthread_exit(retval: *mut c_void) -> !;
    pub fn pthread_once(once_control: *mut pthread_once_t, init_routine: extern "C" fn()) -> c_int;
    pub fn pthread_cancel(thread: pthread_t) -> c_int;
    pub fn pthread_kill(thread: pthread_t, sig: c_int) -> c_int;

    // Thread attribute functions
    pub fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_getdetachstate(
        attr: *const pthread_attr_t,
        detachstate: *mut c_int,
    ) -> c_int;
    pub fn pthread_attr_setdetachstate(attr: *mut pthread_attr_t, detachstate: c_int) -> c_int;
    pub fn pthread_attr_getstack(
        attr: *const pthread_attr_t,
        stackaddr: *mut *mut c_void,
        stacksize: *mut size_t,
    ) -> c_int;
    pub fn pthread_attr_setstack(
        attr: *mut pthread_attr_t,
        stackaddr: *mut c_void,
        stacksize: size_t,
    ) -> c_int;
    pub fn pthread_attr_getstacksize(attr: *const pthread_attr_t, stacksize: *mut size_t) -> c_int;
    pub fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, stacksize: size_t) -> c_int;

    // Mutex functions
    pub fn pthread_mutex_init(
        mutex: *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> c_int;
    pub fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;

    // Mutex attribute functions
    pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_gettype(attr: *const pthread_mutexattr_t, type_: *mut c_int) -> c_int;
    pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, type_: c_int) -> c_int;

    // Condition variable functions
    pub fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) -> c_int;
    pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_cond_timedwait(
        cond: *mut pthread_cond_t,
        mutex: *mut pthread_mutex_t,
        abstime: *const timespec,
    ) -> c_int;
    pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;

    // Condition variable attribute functions
    pub fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> c_int;
    pub fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> c_int;

    // Time functions
    pub fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    pub fn clock_settime(clk_id: clockid_t, tp: *const timespec) -> c_int;
    pub fn clock_getres(clk_id: clockid_t, res: *mut timespec) -> c_int;
    pub fn clock_getcpuclockid(pid: pid_t, clock_id: *mut clockid_t) -> c_int;
    pub fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int;

    // Timer functions
    pub fn timer_create(clockid: clockid_t, evp: *mut sigevent, timerid: *mut timer_t) -> c_int;
    pub fn timer_delete(timerid: timer_t) -> c_int;
    pub fn timer_gettime(timerid: timer_t, value: *mut itimerspec) -> c_int;
    pub fn timer_settime(
        timerid: timer_t,
        flags: c_int,
        value: *const itimerspec,
        ovalue: *mut itimerspec,
    ) -> c_int;

    // POSIX semaphore functions
    pub fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
    pub fn sem_destroy(sem: *mut sem_t) -> c_int;
    pub fn sem_open(name: *const c_char, oflag: c_int, ...) -> *mut sem_t;
    pub fn sem_close(sem: *mut sem_t) -> c_int;
    pub fn sem_unlink(name: *const c_char) -> c_int;
    pub fn sem_wait(sem: *mut sem_t) -> c_int;
    pub fn sem_trywait(sem: *mut sem_t) -> c_int;
    pub fn sem_post(sem: *mut sem_t) -> c_int;
    pub fn sem_getvalue(sem: *mut sem_t, sval: *mut c_int) -> c_int;

    // Standard I/O functions (stdio.h)
    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn freopen(filename: *const c_char, mode: *const c_char, stream: *mut FILE) -> *mut FILE;
    pub fn fclose(stream: *mut FILE) -> c_int;
    pub fn fflush(stream: *mut FILE) -> c_int;
    pub fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    pub fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn rewind(stream: *mut FILE);
    pub fn fgetpos(stream: *mut FILE, pos: *mut fpos_t) -> c_int;
    pub fn fsetpos(stream: *mut FILE, pos: *const fpos_t) -> c_int;
    pub fn clearerr(stream: *mut FILE);
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn perror(s: *const c_char);
    pub fn remove(filename: *const c_char) -> c_int;
    pub fn rename(old: *const c_char, new: *const c_char) -> c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn tmpnam(s: *mut c_char) -> *mut c_char;
    pub fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int, size: size_t) -> c_int;
    pub fn setbuf(stream: *mut FILE, buffer: *mut c_char);
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    pub fn vprintf(format: *const c_char, ap: crate::va_list) -> c_int;
    pub fn vfprintf(stream: *mut FILE, format: *const c_char, ap: crate::va_list) -> c_int;
    pub fn vsprintf(s: *mut c_char, format: *const c_char, ap: crate::va_list) -> c_int;
    pub fn vsnprintf(s: *mut c_char, n: size_t, format: *const c_char, ap: crate::va_list)
        -> c_int;
    pub fn scanf(format: *const c_char, ...) -> c_int;
    pub fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    pub fn getchar() -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn gets(s: *mut c_char) -> *mut c_char;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;

    // Memory management functions (stdlib.h)
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    pub fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;

    // Process control functions (stdlib.h)
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    pub fn _Exit(status: c_int) -> !;
    pub fn atexit(func: extern "C" fn()) -> c_int;
    pub fn system(command: *const c_char) -> c_int;
    pub fn getenv(name: *const c_char) -> *mut c_char;

    // String conversion functions (stdlib.h)
    pub fn atoi(nptr: *const c_char) -> c_int;
    pub fn atol(nptr: *const c_char) -> c_long;
    pub fn atoll(nptr: *const c_char) -> c_longlong;
    pub fn atof(nptr: *const c_char) -> c_double;
    pub fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    pub fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_longlong;
    pub fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    pub fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    pub fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
    pub fn strtof(nptr: *const c_char, endptr: *mut *mut c_char) -> c_float;

    // String functions (string.h)
    pub fn strlen(s: *const c_char) -> size_t;
    pub fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    pub fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncat(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    pub fn strcoll(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strxfrm(dest: *mut c_char, src: *const c_char, n: size_t) -> size_t;
    pub fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    pub fn strspn(s: *const c_char, accept: *const c_char) -> size_t;
    pub fn strcspn(s: *const c_char, reject: *const c_char) -> size_t;
    pub fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    pub fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    pub fn strtok(s: *mut c_char, delim: *const c_char) -> *mut c_char;
    pub fn strerror(errnum: c_int) -> *mut c_char;
    pub fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    // POSIX system functions (unistd.h)
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn close(fd: c_int) -> c_int;
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    pub fn access(pathname: *const c_char, mode: c_int) -> c_int;
    pub fn unlink(pathname: *const c_char) -> c_int;
    pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    pub fn chdir(path: *const c_char) -> c_int;
    pub fn isatty(fd: c_int) -> c_int;
    pub fn getpid() -> pid_t;
    pub fn getppid() -> pid_t;
    pub fn getuid() -> uid_t;
    pub fn getgid() -> gid_t;
    pub fn geteuid() -> uid_t;
    pub fn getegid() -> gid_t;
    pub fn setuid(uid: uid_t) -> c_int;
    pub fn setgid(gid: gid_t) -> c_int;
    pub fn seteuid(euid: uid_t) -> c_int;
    pub fn setegid(egid: gid_t) -> c_int;
    pub fn fork() -> pid_t;
    pub fn execve(
        filename: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_int;
    pub fn sleep(seconds: c_uint) -> c_uint;
    pub fn usleep(usec: useconds_t) -> c_int;
    pub fn alarm(seconds: c_uint) -> c_uint;
    pub fn pause() -> c_int;
    pub fn sysconf(name: c_int) -> c_long;
    pub fn getpagesize() -> c_int;
    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;

    // File control functions (fcntl.h)
    pub fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    pub fn creat(pathname: *const c_char, mode: mode_t) -> c_int;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;

    // Directory functions (dirent.h)
    pub fn opendir(name: *const c_char) -> *mut DIR;
    pub fn closedir(dirp: *mut DIR) -> c_int;
    pub fn readdir(dirp: *mut DIR) -> *mut dirent;
    pub fn rewinddir(dirp: *mut DIR);
    pub fn telldir(dirp: *mut DIR) -> c_long;
    pub fn seekdir(dirp: *mut DIR, loc: c_long);

    // Additional time functions (time.h)
    pub fn time(tloc: *mut time_t) -> time_t;
    pub fn difftime(time1: time_t, time0: time_t) -> c_double;
    pub fn mktime(tm: *mut tm) -> time_t;
    pub fn asctime(tm: *const tm) -> *mut c_char;
    pub fn ctime(timep: *const time_t) -> *mut c_char;
    pub fn gmtime(timep: *const time_t) -> *mut tm;
    pub fn localtime(timep: *const time_t) -> *mut tm;
    pub fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const tm) -> size_t;

    // Math functions (math.h)
    pub fn acos(x: c_double) -> c_double;
    pub fn acosf(x: c_float) -> c_float;
    pub fn asin(x: c_double) -> c_double;
    pub fn asinf(x: c_float) -> c_float;
    pub fn atan(x: c_double) -> c_double;
    pub fn atanf(x: c_float) -> c_float;
    pub fn atan2(y: c_double, x: c_double) -> c_double;
    pub fn atan2f(y: c_float, x: c_float) -> c_float;
    pub fn cos(x: c_double) -> c_double;
    pub fn cosf(x: c_float) -> c_float;
    pub fn sin(x: c_double) -> c_double;
    pub fn sinf(x: c_float) -> c_float;
    pub fn tan(x: c_double) -> c_double;
    pub fn tanf(x: c_float) -> c_float;
    pub fn cosh(x: c_double) -> c_double;
    pub fn coshf(x: c_float) -> c_float;
    pub fn sinh(x: c_double) -> c_double;
    pub fn sinhf(x: c_float) -> c_float;
    pub fn tanh(x: c_double) -> c_double;
    pub fn tanhf(x: c_float) -> c_float;
    pub fn exp(x: c_double) -> c_double;
    pub fn expf(x: c_float) -> c_float;
    pub fn frexp(value: c_double, exp: *mut c_int) -> c_double;
    pub fn frexpf(value: c_float, exp: *mut c_int) -> c_float;
    pub fn ldexp(x: c_double, exp: c_int) -> c_double;
    pub fn ldexpf(x: c_float, exp: c_int) -> c_float;
    pub fn log(x: c_double) -> c_double;
    pub fn logf(x: c_float) -> c_float;
    pub fn log10(x: c_double) -> c_double;
    pub fn log10f(x: c_float) -> c_float;
    pub fn modf(value: c_double, iptr: *mut c_double) -> c_double;
    pub fn modff(value: c_float, iptr: *mut c_float) -> c_float;
    pub fn pow(x: c_double, y: c_double) -> c_double;
    pub fn powf(x: c_float, y: c_float) -> c_float;
    pub fn sqrt(x: c_double) -> c_double;
    pub fn sqrtf(x: c_float) -> c_float;
    pub fn ceil(x: c_double) -> c_double;
    pub fn ceilf(x: c_float) -> c_float;
    pub fn fabs(x: c_double) -> c_double;
    pub fn fabsf(x: c_float) -> c_float;
    pub fn floor(x: c_double) -> c_double;
    pub fn floorf(x: c_float) -> c_float;
    pub fn fmod(x: c_double, y: c_double) -> c_double;
    pub fn fmodf(x: c_float, y: c_float) -> c_float;

    // Memory mapping functions (sys/mman.h)
    pub fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    pub fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    pub fn mlock(addr: *const c_void, len: size_t) -> c_int;
    pub fn munlock(addr: *const c_void, len: size_t) -> c_int;
    pub fn mlockall(flags: c_int) -> c_int;
    pub fn munlockall() -> c_int;
    pub fn msync(addr: *mut c_void, len: size_t, flags: c_int) -> c_int;

    // Dynamic linking functions (dlfcn.h)
    pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> c_int;
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub fn dlerror() -> *mut c_char;

    // Character classification functions (ctype.h)
    pub fn isalnum(c: c_int) -> c_int;
    pub fn isalpha(c: c_int) -> c_int;
    pub fn iscntrl(c: c_int) -> c_int;
    pub fn isdigit(c: c_int) -> c_int;
    pub fn isgraph(c: c_int) -> c_int;
    pub fn islower(c: c_int) -> c_int;
    pub fn isprint(c: c_int) -> c_int;
    pub fn ispunct(c: c_int) -> c_int;
    pub fn isspace(c: c_int) -> c_int;
    pub fn isupper(c: c_int) -> c_int;
    pub fn isxdigit(c: c_int) -> c_int;
    pub fn tolower(c: c_int) -> c_int;
    pub fn toupper(c: c_int) -> c_int;
}

// Re-export common prelude items
pub use crate::*;
