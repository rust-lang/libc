//! Hacking together the definitions for VxWorks Bindings
use core::mem::size_of;
use core::ptr::null_mut;

#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum DIR {}
impl ::Copy for DIR {}
impl ::Clone for DIR {
    fn clone(&self) -> DIR {
        *self
    }
}

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub type pid_t = i32;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sighandler_t = ::size_t;
pub type cc_t = ::c_uchar;

pub type blkcnt_t = ::c_long;
pub type blksize_t = ::c_long;
pub type ino_t = ::c_ulong;
pub type ino32_t = u32;
pub type off_t = ::c_longlong;

pub type rlim_t = ::c_ulonglong;
pub type suseconds_t = ::c_long;
pub type time_t = ::c_long;
pub type wchar_t = ::c_int;
pub type errno_t = ::c_int;

pub type useconds_t = ::c_ulong;

pub type socklen_t = ::c_int;

pub type pthread_t = ::c_ulong;

pub type clockid_t = ::c_int;

//defined for the structs
pub type dev_t = ::c_ulong;
pub type mode_t = ::c_int;
pub type nlink_t = ::c_ulong;
pub type uid_t = ::c_ushort;
pub type gid_t = ::c_ushort;
pub type sigset_t = ::c_ulonglong;
pub type key_t = ::c_long;
pub type shmatt_t = ::c_ulong;

pub type mqd_t = ::c_int;

pub type nfds_t = ::c_uint;
pub type nl_item = ::c_int;
pub type stat64 = ::stat;

pub type pthread_key_t = ::c_ulong;

// From b_off_t.h
pub type off64_t = ::c_longlong;
pub type off_t64 = ::c_longlong;

// From b_BOOL.h
pub type BOOL = ::c_int; // excuse me what

//Straight from vxWind.h ..
pub type _Vx_OBJ_HANDLE = ::c_int;
pub type _Vx_TASK_ID = ::_Vx_OBJ_HANDLE;
pub type _Vx_MSG_Q_ID = ::_Vx_OBJ_HANDLE;
pub type _Vx_SEM_ID_KERNEL = ::_Vx_OBJ_HANDLE;
pub type _Vx_RTP_ID = ::_Vx_OBJ_HANDLE;
pub type _Vx_SD_ID = ::_Vx_OBJ_HANDLE;
pub type _Vx_CONDVAR_ID = ::_Vx_OBJ_HANDLE;
pub type _Vx_SEM_ID = *mut ::_Vx_semaphore;
pub type OBJ_HANDLE = ::_Vx_OBJ_HANDLE;
pub type TASK_ID = ::OBJ_HANDLE;
pub type MSG_Q_ID = ::OBJ_HANDLE;
pub type SEM_ID_KERNEL = ::OBJ_HANDLE;
pub type RTP_ID = ::OBJ_HANDLE;
pub type SD_ID = ::OBJ_HANDLE;
pub type CONDVAR_ID = ::OBJ_HANDLE;

// From vxTypes.h
pub type _Vx_usr_arg_t = ::ssize_t; // c_int for LP32
pub type _Vx_exit_code_t = ::ssize_t; // c_int for LP32
pub type _Vx_ticks_t = ::c_uint;
pub type _Vx_ticks64_t = ::c_ulonglong;

// From vxTypesBase.h
pub type va_list = *mut ::c_char;

pub type sa_family_t = ::c_uchar;

// structs that only exist in userspace
s! {
    // b_struct_vx_eventsResourceCb.h
    pub struct _Vx_EVENTS_RSRC {
        pub registered : ::c_uint,
        pub taskId     : ::c_int,
        pub options    : ::c_uchar,
        pub pad        : [::c_uchar; 3],
    }

    // b_struct_vx_semaphore.h
    pub struct _Vx_semaphore {
        pub magic  : ::c_uint,
        pub semType: ::c_uint,
        pub options: ::c_uint,
        pub recurse: ::c_uint,
        pub owned_k: ::c_uint, // owned_k is volatile
        pub semId_k: ::_Vx_SEM_ID_KERNEL,
        pub state  : ::c_uint, //state is union of _Vx_UINT and _Vx_UINT
        pub events : ::_Vx_EVENTS_RSRC,
    }

    // b_pthread_condattr_t.h
    pub struct pthread_condattr_t {
        pub condAttrStatus: ::c_int,
        pub condAttrPshared: ::c_int,
        pub _CondAttrClockId: ::clockid_t,
    }

    // b_pthread_cond_t.h
    pub struct pthread_cond_t{
        pub condSemId: ::_Vx_SEM_ID,
        pub condValid: ::c_int,
        pub condInitted: ::c_int,
        pub condRefCount: ::c_int,
        pub condMutex: *mut ::pthread_mutex_t,
        pub condAttr: ::pthread_condattr_t,
        pub condSemName: [::c_char; PTHREAD_SHARED_SEM_NAME_MAX]
    }

    // b_pthread_rwlockattr_t.h
    pub struct pthread_rwlockattr_t {
        pub rwlockAttrStatus: ::c_int,
        pub rwlockAttrMaxReaders: ::c_uint,
    }

    // b_pthread_rwlock_t.h
    pub struct pthread_rwlock_t {
        pub rwlockSemId: :: _Vx_SEM_ID,
        pub rwlockReadersRefCount: ::c_int,
        pub rwlockValid: ::c_int,
        pub rwlockInitted: ::c_int,
        pub rwlockAttr: ::pthread_rwlockattr_t,
        pub rwlockName: [::c_char; PTHREAD_SHARED_SEM_NAME_MAX]
    }

    // b_struct_timeval.h
    pub struct timeval {
        pub tv_sec: ::time_t,
        pub tv_usec: ::suseconds_t,
    }

    // socket.h
    pub struct sockaddr {
        pub sa_len    : ::c_uchar,
        pub sa_family : sa_family_t,
        pub sa_data   : [::c_char; 14],
    }

    // socket.h
    pub struct sockaddr_storage {
        pub ss_len     : ::c_uchar,
        pub ss_family  : ::sa_family_t,
        pub __ss_pad1  : [::c_char; _SS_PAD1SIZE],
        pub __ss_align : i32,
        // pub __ss_pad2  : [::c_char; _SS_PAD2SIZE],
        pub __ss_pad2  : [::c_char; 32],
        pub __ss_pad3  : [::c_char; 32],
        pub __ss_pad4  : [::c_char; 32],
        pub __ss_pad5  : [::c_char; 32],
    }
    pub struct iovec {
        pub iov_base: *mut ::c_void,
        pub iov_len: ::size_t,
    }

    // poll.h
    pub struct pollfd {
        pub fd      : ::c_int,
        pub events  : ::c_short,
        pub revents : ::c_short,
    }

    // dirent.h
    pub struct dirent {
        pub d_ino  : ::ino_t,
        // pub d_name : [::c_char; (_PARM_NAME_MAX + 1)],
        pub d_name : [::c_char; 32],
        pub d_name1 : [::c_char; 32],
        pub d_name2 : [::c_char; 32],
        pub d_name3 : [::c_char; 32],
        pub d_name4 : [::c_char; 32],
        pub d_name5 : [::c_char; 32],
        pub d_name6 : [::c_char; 32],
        pub d_name7 : [::c_char; 32],
    }

    pub struct dirent64 {
        pub d_ino    : ::ino_t,
        pub d_off    : ::off64_t,
        pub d_reclen : u16,
        pub d_type   : u8,
        // pub d_name   : [::c_char; 256],
        pub d_name   : [::c_char; 32],
        pub d_name1   : [::c_char; 32],
        pub d_name2   : [::c_char; 32],
        pub d_name3   : [::c_char; 32],
        pub d_name4   : [::c_char; 32],
        pub d_name5   : [::c_char; 32],
        pub d_name6   : [::c_char; 32],
        pub d_name7   : [::c_char; 32],
    } // Doesn't seem like it exists anymore

    // resource.h
    pub struct rlimit { /* Is this really needed? Questionable ... */
                           pub rlim_cur : ::size_t,
                           pub rlim_max : ::size_t,
    }

    // stat.h
    pub struct stat {
                         pub st_dev       : ::dev_t,
                         pub st_ino       : ::ino_t,
                         pub st_mode      : ::mode_t,
                         pub st_nlink     : ::nlink_t,
                         pub st_uid       : ::uid_t,
                         pub st_gid       : ::gid_t,
                         pub st_rdev      : ::dev_t,
                         pub st_size      : ::off64_t,
                         pub st_atime     : ::time_t,
                         pub st_mtime     : ::time_t,
                         pub st_ctime     : ::time_t,
                         pub st_blksize   : ::blksize_t,
                         pub st_blocks    : ::blkcnt_t,
                         pub st_attrib    : ::c_uchar,
                         pub st_reserved1 : ::c_int,
                         pub st_reserved2 : ::c_int,
                         pub st_reserved3 : ::c_int,
                         pub st_reserved4 : ::c_int,
    }

    //b_struct__Timespec.h
    pub struct _Timespec {
        pub tv_sec  : ::time_t,
        pub tv_nsec : ::c_long,
    }

    // b_struct__Sched_param.h
    pub struct _Sched_param {
        pub sched_priority: ::c_int, /* scheduling priority */
        pub sched_ss_low_priority: ::c_int,    /* low scheduling priority */
        pub sched_ss_repl_period: ::_Timespec, /* replenishment period */
        pub sched_ss_init_budget: ::_Timespec, /* initial budget */
        pub sched_ss_max_repl: ::c_int,        /* max pending replenishment */

    }

    // b_pthread_attr_t.h
    pub struct pthread_attr_t {
        pub threadAttrStatus          : ::c_int,
        pub threadAttrStacksize       : ::size_t,
        pub threadAttrStackaddr       : *mut ::c_void,
        pub threadAttrGuardsize       : ::size_t,
        pub threadAttrDetachstate     : ::c_int,
        pub threadAttrContentionscope : ::c_int,
        pub threadAttrInheritsched    : ::c_int,
        pub threadAttrSchedpolicy     : ::c_int,
        pub threadAttrName            : *mut ::c_char,
        pub threadAttrOptions         : ::c_int,
        pub threadAttrSchedparam      : ::_Sched_param,
    }

    // signal.h
    pub struct sigaction { // pulled from kernel side,
        pub sa_u     : ::size_t,
        // This is a union of two function pointers.
        // Under the assumption that function pointers are the same
        // size as other pointers, we can replace the union with size_t
        pub sa_mask  : ::sigset_t,
        pub sa_flags : ::c_int,
    }

    // b_stack_t.h
    pub struct stack_t {
        pub ss_sp    : *mut ::c_void,
        pub ss_size  : ::size_t,
        pub ss_flags : ::c_int,
    }

    // signal.h
    pub struct siginfo_t {
        pub si_signo : ::c_int,
        pub si_code  : ::c_int,
        // This field is a union of int and void * in vxworks
        // The size has been set to the larger of the two
        pub si_value : ::size_t,
    }

    pub struct ipc_perm {
                             pub __key : key_t,
                             pub uid   : uid_t,
                             pub gid   : gid_t,
                             pub cuid  : uid_t,
                             pub cgid  : gid_t,
                             pub mode  : ::c_ushort,
                             pub __seq : ::c_ushort,
    }

    pub struct shmid_ds {
        pub shm_perm  : ipc_perm,
        pub shm_segsz : ::c_int,
    }

    // pthread.h (krnl)
    // b_pthread_mutexattr_t.h (usr)
    pub struct pthread_mutexattr_t {
        mutexAttrStatus      : ::c_int,
        mutexAttrPshared     : ::c_int,
        mutexAttrProtocol    : ::c_int,
        mutexAttrPrioceiling : ::c_int,
        mutexAttrType        : ::c_int,
    }

    // pthread.h (krnl)
    // b_pthread_mutex_t.h (usr)
    pub struct pthread_mutex_t  {
        pub mutexSemId: ::_Vx_SEM_ID, /*_Vx_SEM_ID ..*/
        pub mutexValid: ::c_int,
        pub mutexInitted: ::c_int,
        pub mutexCondRefCount: ::c_int,
        pub mutexSavPriority: ::c_int,
        pub mutexAttr: ::pthread_mutexattr_t,
        pub mutexSemName: [::c_char; PTHREAD_SHARED_SEM_NAME_MAX],
    }

    // b_struct_timespec.h
    pub struct timespec {
        pub tv_sec: ::time_t,
        pub tv_nsec: ::c_long,
    }

    // in.h
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }

    // in.h
    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    // in6.h
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    // in6.h
    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: in6_addr,
        pub ipv6mr_interface: ::c_uint,
    }

    // netdb.h
    pub struct addrinfo {
        pub ai_flags    : ::c_int,
        pub ai_family   : ::c_int,
        pub ai_socktype : ::c_int,
        pub ai_protocol : ::c_int,
        pub ai_addrlen  : ::size_t,
        pub ai_canonname: *mut ::c_char,
        pub ai_addr     : *mut ::sockaddr,
        pub ai_next     : *mut ::addrinfo,
    }

    // in.h
    pub struct sockaddr_in {
        pub sin_len   : u8,
        pub sin_family: u8,
        pub sin_port  : u16,
        pub sin_addr  : ::in_addr,
        pub sin_zero  : [::c_char; 8],
    }

    // in6.h
    // There is a different implementation in ipv6.h in
    // krnl directory, but this seems to only happen
    // when the VSB is built for ipv6 only.
    pub struct sockaddr_in6 {
        pub sin6_len     : u8,
        pub sin6_family  : u8,
        pub sin6_port    : u16,
        pub sin6_flowinfo: u32,
        pub sin6_addr    : ::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        //pub sun_path: [::c_char; 108]
        pub sun_path: [::c_char; 32],
        pub sun_path1: [::c_char; 32],
        pub sun_path2: [::c_char; 32],
        pub sun_path3: [::c_char; 12],
    }

    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }

    // rtpLibCommon.h
    pub struct RTP_DESC {
        pub status    : ::c_int,
        pub options   : u32,
        pub entrAddr  : *mut ::c_void,
        pub initTaskId: ::TASK_ID,
        pub parentId  : ::RTP_ID,
        //pub pathName  : [::c_char; (VX_RTP_NAME_LENGTH + 1)],
        pub pathName  : [::c_char; 32],
        pub pathName1  : [::c_char; 32],
        pub pathName2  : [::c_char; 32],
        pub pathName3  : [::c_char; 32],
        pub pathName4  : [::c_char; 32],
        pub pathName5  : [::c_char; 32],
        pub pathName6  : [::c_char; 32],
        pub pathName7  : [::c_char; 32],
        pub taskCnt   : u32,
        pub textStart : *mut ::c_void,
        pub textEnd   : *mut ::c_void,
    }

    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }
}

pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;

pub const EXIT_SUCCESS: ::c_int = 0;
pub const EXIT_FAILURE: ::c_int = 1;

pub const EAI_SERVICE: ::c_int = 9;
pub const EAI_SOCKTYPE: ::c_int = 10;
pub const EAI_SYSTEM: ::c_int = 11;

pub const RTLD_DEFAULT: *mut ::c_void = 0i64 as *mut ::c_void;

//Clock Lib Stuff
pub const CLOCK_REALTIME: ::c_int = 0x0;
pub const CLOCK_MONOTONIC: ::c_int = 0x1;
pub const CLOCK_PROCESS_CPUTIME_ID: ::c_int = 0x2;
pub const CLOCK_THREAD_CPUTIME_ID: ::c_int = 0x3;
pub const TIMER_ABSTIME: ::c_int = 0x1;
pub const TIME_RELTIME: ::c_int = 0xFFFFFFFE;

// PTHREAD STUFF
pub const PTHREAD_INITIALIZED_OBJ: ::c_int = 0xF70990EF;
pub const PTHREAD_DESTROYED_OBJ: ::c_int = -1;
pub const PTHREAD_VALID_OBJ: ::c_int = 0xEC542A37;
pub const PTHREAD_INVALID_OBJ: ::c_int = -1;
pub const PTHREAD_UNUSED_YET_OBJ: ::c_int = -1;

pub const PTHREAD_PRIO_NONE: ::c_int = 0;
pub const PTHREAD_PRIO_INHERIT: ::c_int = 1;
pub const PTHREAD_PRIO_PROTECT: ::c_int = 2;

pub const PTHREAD_MUTEX_NORMAL: ::c_int = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 1;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_STACK_MIN: usize = 4096;
pub const PTHREAD_SHARED_SEM_NAME_MAX: usize = 30;

pub const EFAULT: ::c_int = 14;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const ENODEV: ::c_int = 19;
pub const EINVAL: ::c_int = 22;
pub const EPIPE: ::c_int = 32;
pub const ERANGE: ::c_int = 34;

// ERRNO STUFF
pub const EPERM: ::c_int = 1; /* Not owner */
pub const ENOENT: ::c_int = 2; /* No such file or directory */
pub const ESRCH: ::c_int = 3; /* No such process */
pub const EINTR: ::c_int = 4; /* Interrupted system call */
pub const EIOA: ::c_int = 5; /* I/O error */
pub const ENXIO: ::c_int = 6; /* No such device or address */
pub const E2BIG: ::c_int = 7; /* Arg list too long */
pub const ENOEXEC: ::c_int = 8; /* Exec format error */
pub const EBADF: ::c_int = 9; /* Bad file number */
pub const CHILD: ::c_int = 10; /* No children */
pub const EAGAIN: ::c_int = 11; /* No more processes */
pub const ENOMEM: ::c_int = 12; /* Not enough core */
pub const EACCES: ::c_int = 13; /* Permission denied */
pub const EDEADLK: ::c_int = 33;
pub const EINPROGRESS: ::c_int = 68;
pub const EALREADY: ::c_int = 69;
pub const EWOULDBLOCK: ::c_int = 70;
pub const ENOSYS: ::c_int = 71;
pub const EDESTADDRREQ: ::c_int = 40;
pub const EPROTOTYPE: ::c_int = 41;
pub const ENOPROTOOPT: ::c_int = 42;
pub const EPROTONOSUPPORT: ::c_int = 43;
pub const ESOCKTNOSUPPORT: ::c_int = 44;
pub const EOPNOTSUPP: ::c_int = 45;
pub const EPFNOSUPPORT: ::c_int = 46;
pub const EAFNOSUPPORT: ::c_int = 47;
pub const EADDRINUSE: ::c_int = 48;
pub const EADDRNOTAVAIL: ::c_int = 49;
pub const ENOTSOCK: ::c_int = 50;
pub const ENETUNREACH: ::c_int = 51;
pub const ENETRESET: ::c_int = 52;
pub const ECONNABORTED: ::c_int = 53;
pub const ECONNRESET: ::c_int = 54;
pub const ENOBUFS: ::c_int = 55;
pub const EISCONN: ::c_int = 56;
pub const ENOTCONN: ::c_int = 57;
pub const ESHUTDOWN: ::c_int = 58;
pub const ETOOMANYREFS: ::c_int = 59;
pub const ETIMEDOUT: ::c_int = 60;
pub const ECONNREFUSED: ::c_int = 61;

// NFS errnos: Refer to pkgs_v2/storage/fs/nfs/h/nfs/nfsCommon.h
const M_nfsStat: ::c_int = 48 << 16;
enum nfsstat {
    NFS_OK = 0,
    NFSERR_PERM = 1,
    NFSERR_NOENT = 2,
    NFSERR_IO = 5,
    NFSERR_NXIO = 6,
    NFSERR_ACCESS = 13,
    NFSERR_EXIST = 17,
    NFSERR_XDEV = 18,
    NFSERR_NODEV = 19,
    NFSERR_NOTDIR = 20,
    NFSERR_ISDIR = 21,
    NFSERR_INVAL = 22,
    NFSERR_FBIG = 27,
    NFSERR_NOSPC = 28,
    NFSERR_ROFS = 30,
    NFSERR_MLINK = 31,
    NFSERR_NAMETOOLONG = 63,
    NFSERR_NOTEMPTY = 66,
    NFSERR_DQUOT = 69,
    NFSERR_STALE = 70,
    NFSERR_REMOTE = 71,
    NFSERR_WFLUSH = 99,
    NFSERR_BADHANDLE = 10001,
    NFSERR_NOT_SYNC = 10002,
    NFSERR_BAD_COOKIE = 10003,
    NFSERR_NOTSUPP = 10004,
    NFSERR_TOOSMALL = 10005,
    NFSERR_SERVERFAULT = 10006,
    NFSERR_BADTYPE = 10007,
    NFSERR_JUKEBOX = 10008,
}

pub const S_nfsLib_NFS_OK: ::c_int = M_nfsStat | nfsstat::NFS_OK as ::c_int;
pub const S_nfsLib_NFSERR_PERM: ::c_int =
    M_nfsStat | nfsstat::NFSERR_PERM as ::c_int;
pub const S_nfsLib_NFSERR_NOENT: ::c_int =
    M_nfsStat | nfsstat::NFSERR_NOENT as ::c_int;
pub const S_nfsLib_NFSERR_IO: ::c_int =
    M_nfsStat | nfsstat::NFSERR_IO as ::c_int;
pub const S_nfsLib_NFSERR_NXIO: ::c_int =
    M_nfsStat | nfsstat::NFSERR_NXIO as ::c_int;
pub const S_nfsLib_NFSERR_ACCESS: ::c_int =
    M_nfsStat | nfsstat::NFSERR_ACCESS as ::c_int;
pub const S_nfsLib_NFSERR_EXIST: ::c_int =
    M_nfsStat | nfsstat::NFSERR_EXIST as ::c_int;
pub const S_nfsLib_NFSERR_XDEV: ::c_int =
    M_nfsStat | nfsstat::NFSERR_XDEV as ::c_int;
pub const S_nfsLib_NFSERR_NODEV: ::c_int =
    M_nfsStat | nfsstat::NFSERR_NODEV as ::c_int;
pub const S_nfsLib_NFSERR_NOTDIR: ::c_int =
    M_nfsStat | nfsstat::NFSERR_NOTDIR as ::c_int;
pub const S_nfsLib_NFSERR_ISDIR: ::c_int =
    M_nfsStat | nfsstat::NFSERR_ISDIR as ::c_int;
pub const S_nfsLib_NFSERR_INVAL: ::c_int =
    M_nfsStat | nfsstat::NFSERR_INVAL as ::c_int;
pub const S_nfsLib_NFSERR_FBIG: ::c_int =
    M_nfsStat | nfsstat::NFSERR_FBIG as ::c_int;
pub const S_nfsLib_NFSERR_NOSPC: ::c_int =
    M_nfsStat | nfsstat::NFSERR_NOSPC as ::c_int;
pub const S_nfsLib_NFSERR_ROFS: ::c_int =
    M_nfsStat | nfsstat::NFSERR_ROFS as ::c_int;
pub const S_nfsLib_NFSERR_MLINK: ::c_int =
    M_nfsStat | nfsstat::NFSERR_MLINK as ::c_int;
pub const S_nfsLib_NFSERR_NAMETOOLONG: ::c_int =
    M_nfsStat | nfsstat::NFSERR_NAMETOOLONG as ::c_int;
pub const S_nfsLib_NFSERR_NOTEMPTY: ::c_int =
    M_nfsStat | nfsstat::NFSERR_NOTEMPTY as ::c_int;
pub const S_nfsLib_NFSERR_DQUOT: ::c_int =
    M_nfsStat | nfsstat::NFSERR_DQUOT as ::c_int;
pub const S_nfsLib_NFSERR_STALE: ::c_int =
    M_nfsStat | nfsstat::NFSERR_STALE as ::c_int;
pub const S_nfsLib_NFSERR_WFLUSH: ::c_int =
    M_nfsStat | nfsstat::NFSERR_WFLUSH as ::c_int;
pub const S_nfsLib_NFSERR_REMOTE: ::c_int =
    M_nfsStat | nfsstat::NFSERR_REMOTE as ::c_int;
pub const S_nfsLib_NFSERR_BADHANDLE: ::c_int =
    M_nfsStat | nfsstat::NFSERR_BADHANDLE as ::c_int;
pub const S_nfsLib_NFSERR_NOT_SYNC: ::c_int =
    M_nfsStat | nfsstat::NFSERR_NOT_SYNC as ::c_int;
pub const S_nfsLib_NFSERR_BAD_COOKIE: ::c_int =
    M_nfsStat | nfsstat::NFSERR_BAD_COOKIE as ::c_int;
pub const S_nfsLib_NFSERR_NOTSUPP: ::c_int =
    M_nfsStat | nfsstat::NFSERR_NOTSUPP as ::c_int;
pub const S_nfsLib_NFSERR_TOOSMALL: ::c_int =
    M_nfsStat | nfsstat::NFSERR_TOOSMALL as ::c_int;
pub const S_nfsLib_NFSERR_SERVERFAULT: ::c_int =
    M_nfsStat | nfsstat::NFSERR_SERVERFAULT as ::c_int;
pub const S_nfsLib_NFSERR_BADTYPE: ::c_int =
    M_nfsStat | nfsstat::NFSERR_BADTYPE as ::c_int;
pub const S_nfsLib_NFSERR_JUKEBOX: ::c_int =
    M_nfsStat | nfsstat::NFSERR_JUKEBOX as ::c_int;

// IP Stuff? These are allll guesswork
pub const IPPROTO_IP: ::c_int = 0;
pub const IP_TTL: ::c_int = 4; // not sure if this is right
pub const IP_ADD_MEMBERSHIP: ::c_int = 11;
pub const IP_DROP_MEMBERSHIP: ::c_int = 12;
pub const IPV6_V6ONLY: ::c_int = 26;
pub const IP_MULTICAST_TTL: ::c_int = 33;
pub const IP_MULTICAST_LOOP: ::c_int = 34;
pub const IPV6_MULTICAST_LOOP: ::c_int = 19;
pub const IPPROTO_IPV6: ::c_int = 41; // or this one, for that matter

// STAT Stuff
pub const S_IFMT: ::c_int = 0xf000;
pub const S_IFIFO: ::c_int = 0x1000;
pub const S_IFCHR: ::c_int = 0x2000;
pub const S_IFDIR: ::c_int = 0x4000;
pub const S_IFBLK: ::c_int = 0x6000;
pub const S_IFREG: ::c_int = 0x8000;
pub const S_IFLNK: ::c_int = 0xa000;
pub const S_IFSHM: ::c_int = 0xb000;
pub const S_IFDEVMEM: ::c_int = 0xd000;
pub const S_IFSOCK: ::c_int = 0xc000;
pub const S_ISUID: ::c_int = 0x0800;
pub const S_ISGID: ::c_int = 0x0400;
pub const S_ISTXT: ::c_int = 0x0200;
pub const S_IRUSR: ::c_int = 0x0100;
pub const S_IWUSR: ::c_int = 0x0080;
pub const S_IXUSR: ::c_int = 0x0040;
pub const S_IRWXU: ::c_int = 0x01c0;
pub const S_IRGRP: ::c_int = 0x0020;
pub const S_IWGRP: ::c_int = 0x0010;
pub const S_IXGRP: ::c_int = 0x0008;
pub const S_IRWXG: ::c_int = 0x0038;
pub const S_IROTH: ::c_int = 0x0004;
pub const S_IWOTH: ::c_int = 0x0002;
pub const S_IXOTH: ::c_int = 0x0001;
pub const S_IRWXO: ::c_int = 0x0007;

pub const SOL_SOCKET: ::c_int = 0xffff;
pub const SO_BROADCAST: ::c_int = 0x001e;
pub const SO_SNDTIMEO: ::c_int = 0x1005;
pub const SO_RCVTIMEO: ::c_int = 0x1006;
pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_RAW: ::c_int = 3;
pub const SOCK_RDM: ::c_int = 4;
pub const SOCK_SEQPACKET: ::c_int = 5;
pub const SOCK_PACKET: ::c_int = 10;
pub const SO_DEBUG: ::c_int = 0x0001;
pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_RCVLOWAT: ::c_int = 0x0012;

pub const _SS_MAXSIZE: usize = 128;
pub const _SS_ALIGNSIZE: usize = size_of::<u32>();
pub const _SS_PAD1SIZE: usize =
    (_SS_ALIGNSIZE - size_of::<::c_uchar>() - size_of::<::sa_family_t>());
pub const _SS_PAD2SIZE: usize = (_SS_MAXSIZE
    - size_of::<::c_uchar>()
    - size_of::<::sa_family_t>()
    - _SS_PAD1SIZE
    - _SS_ALIGNSIZE);

pub const MSG_OOB: ::c_int = 0x0001;
pub const MSG_PEEK: ::c_int = 0x0002;
pub const MSG_DONTROUTE: ::c_int = 0x0004;
pub const MSG_EOR: ::c_int = 0x0008;
pub const MSG_TRUNC: ::c_int = 0x0010;
pub const MSG_CTRUNC: ::c_int = 0x0020;
pub const MSG_WAITALL: ::c_int = 0x0040;
pub const MSG_DONTWAIT: ::c_int = 0x0080;
pub const MSG_EOF: ::c_int = 0x0100;
pub const MSG_EXP: ::c_int = 0x0200;
pub const MSG_MBUF: ::c_int = 0x0400;
pub const MSG_NOTIFICATION: ::c_int = 0x0800;
pub const MSG_COMPAT: ::c_int = 0x8000;

pub const AF_UNSPEC: ::c_int = 0;
pub const AF_LOCAL: ::c_int = 1;
pub const AF_UNIX: ::c_int = AF_LOCAL;
pub const AF_INET: ::c_int = 2;
pub const AF_NETLINK: ::c_int = 16;
pub const AF_ROUTE: ::c_int = 17;
pub const AF_LINK: ::c_int = 18;
pub const AF_PACKET: ::c_int = 19;
pub const pseudo_AF_KEY: ::c_int = 27;
pub const AF_KEY: ::c_int = pseudo_AF_KEY;
pub const AF_INET6: ::c_int = 28;
pub const AF_SOCKDEV: ::c_int = 31;
pub const AF_TIPC: ::c_int = 33;
pub const AF_MIPC: ::c_int = 34;
pub const AF_MIPC_SAFE: ::c_int = 35;
pub const AF_MAX: ::c_int = 36;

pub const SHUT_RD: ::c_int = 0;
pub const SHUT_WR: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;

pub const IPPROTO_TCP: ::c_int = 6;
pub const TCP_NODELAY: ::c_int = 1;
pub const TCP_MAXSEG: ::c_int = 2;
pub const TCP_NOPUSH: ::c_int = 3;
pub const TCP_KEEPIDLE: ::c_int = 4;
pub const TCP_KEEPINTVL: ::c_int = 5;
pub const TCP_KEEPCNT: ::c_int = 6;
pub const SO_ERROR: ::c_int = 4;

// IO Lib Definitions:

pub const FIONREAD: ::c_int = 1;
pub const FIOFLUSH: ::c_int = 2;
pub const FIOOPTIONS: ::c_int = 3;
pub const FIOBAUDRATE: ::c_int = 4;
pub const FIODISKFORMAT: ::c_int = 5;
pub const FIODISKINIT: ::c_int = 6;
pub const FIOSEEK: ::c_int = 7;
pub const FIOWHERE: ::c_int = 8;
pub const FIODIRENTRY: ::c_int = 9;
pub const FIORENAME: ::c_int = 10;
pub const FIOREADYCHANGE: ::c_int = 11;
pub const FIOWRITE: ::c_int = 12;
pub const FIODISKCHANGE: ::c_int = 13;
pub const FIOCANCEL: ::c_int = 14;
pub const FIOSQUEEZE: ::c_int = 15;
pub const FIONBIO: ::c_int = -1878786032; // it goes on ...
pub const _POSIX_PATH_MAX: ::c_int = 256;

// Some poll stuff
pub const POLLIN: ::c_short = 0x0001;
pub const POLLPRI: ::c_short = 0x0002;
pub const POLLOUT: ::c_short = 0x0004;
pub const POLLRDNORM: ::c_short = 0x0040;
pub const POLLWRNORM: ::c_short = POLLOUT;
pub const POLLRDBAND: ::c_short = 0x0080;
pub const POLLWRBAND: ::c_short = 0x0100;
pub const POLLER: ::c_short = 0x0008;
pub const POLLHUP: ::c_short = 0x0010;
pub const POLLNVAL: ::c_short = 0x0020;

//Some Fcntlcom Stuff (look at fcntlcom.h to find definitions)
pub const FD_CLOEXEC: ::c_int = 1;
pub const F_DUPFD: ::c_int = 0;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;
pub const F_GETOWN: ::c_int = 5;
pub const F_SETOWN: ::c_int = 6;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const F_DUPFD_CLOEXEC: ::c_int = 14;

//Some Dirent.h stuff
pub const DT_UNKNOWN: ::c_uchar = 0x0;
pub const DT_FIFO: ::c_uchar = 0x1;
pub const DT_CHR: ::c_uchar = 0x2;
pub const DT_DIR: ::c_uchar = 0x4;
pub const DT_BLK: ::c_uchar = 0x6;
pub const DT_REG: ::c_uchar = 0x8;
pub const DT_LNK: ::c_uchar = 0xA;
pub const DT_SOCK: ::c_uchar = 0xC;
pub const DT_WHT: ::c_uchar = 0xE;

// Other Random Stuff
pub const VXSIM_EWOULDBLOCK: ::c_int = 70;
pub const IPV6_ADD_MEMBERSHIP: ::c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: ::c_int = 21;

pub const SIG_DFL: sighandler_t = 0 as sighandler_t;
pub const SIG_IGN: sighandler_t = 1 as sighandler_t;
pub const SIG_ERR: sighandler_t = !0 as sighandler_t;

pub const SIGHUP: ::c_int = 1;
pub const SIGINT: ::c_int = 2;
pub const SIGQUIT: ::c_int = 3;
pub const SIGILL: ::c_int = 4;
pub const SIGTRAP: ::c_int = 5;
pub const SIGABRT: ::c_int = 6;
pub const SIGEMT: ::c_int = 7;
pub const SIGFPE: ::c_int = 8;
pub const SIGKILL: ::c_int = 9;
pub const SIGBUS: ::c_int = 10;
pub const SIGSEGV: ::c_int = 11;
pub const SIGFMT: ::c_int = 12;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGTERM: ::c_int = 15;
pub const SIGCNCL: ::c_int = 16;
pub const SIGSTOP: ::c_int = 17;
pub const SIGTSTP: ::c_int = 18;
pub const SIGCONT: ::c_int = 19;
pub const SIGCHLD: ::c_int = 20;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;

pub const SIG_BLOCK: ::c_int = 1;
pub const SIG_UNBLOCK: ::c_int = 2;
pub const SIG_SETMASK: ::c_int = 3;

pub const SI_SYNC: ::c_int = 0;
pub const SI_USER: ::c_int = -1;
pub const SI_QUEUE: ::c_int = -2;
pub const SI_TIMER: ::c_int = -3;
pub const SI_ASYNCIO: ::c_int = -4;
pub const SI_MESGQ: ::c_int = -5;
pub const SI_CHILD: ::c_int = -6;
pub const SI_KILL: ::c_int = SI_USER;

// vxParams.h definitions
pub const _PARM_NAME_MAX: usize = 255;
pub const _PARM_PATH_MAX: usize = 1024;

// WAIT STUFF
pub const WNOHANG: ::c_int = 0x01;
pub const WUNTRACED: ::c_int = 0x02;

const PTHREAD_MUTEXATTR_INITIALIZER: pthread_mutexattr_t =
    pthread_mutexattr_t {
        mutexAttrStatus: PTHREAD_INITIALIZED_OBJ,
        mutexAttrProtocol: PTHREAD_PRIO_NONE,
        mutexAttrPrioceiling: 0,
        mutexAttrType: PTHREAD_MUTEX_DEFAULT,
        mutexAttrPshared: 1,
    };
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    mutexSemId: null_mut(),
    mutexValid: PTHREAD_VALID_OBJ,
    mutexInitted: PTHREAD_UNUSED_YET_OBJ,
    mutexCondRefCount: 0,
    mutexSavPriority: 0,
    mutexAttr: PTHREAD_MUTEXATTR_INITIALIZER,
    mutexSemName: [0; PTHREAD_SHARED_SEM_NAME_MAX],
};

const PTHREAD_CONDATTR_INITIALIZER: pthread_condattr_t = pthread_condattr_t {
    condAttrStatus: 0,
    condAttrPshared: 0,
    _CondAttrClockId: CLOCK_REALTIME,
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    condSemId: null_mut(),
    condValid: PTHREAD_VALID_OBJ,
    condInitted: PTHREAD_UNUSED_YET_OBJ,
    condRefCount: 0,
    condMutex: null_mut(),
    condAttr: PTHREAD_CONDATTR_INITIALIZER,
    condSemName: [0; PTHREAD_SHARED_SEM_NAME_MAX],
};

const PTHREAD_RWLOCKATTR_INITIALIZER: pthread_rwlockattr_t =
    pthread_rwlockattr_t {
        rwlockAttrStatus: PTHREAD_INITIALIZED_OBJ,
        rwlockAttrMaxReaders: 0,
    };
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    rwlockSemId: null_mut(),
    rwlockReadersRefCount: 0,
    rwlockValid: PTHREAD_VALID_OBJ,
    rwlockInitted: PTHREAD_UNUSED_YET_OBJ,
    rwlockAttr: PTHREAD_RWLOCKATTR_INITIALIZER,
    rwlockName: [0; PTHREAD_SHARED_SEM_NAME_MAX],
};

pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;

// rtpLibCommon.h
pub const VX_RTP_NAME_LENGTH: usize = 255;

//Some unsupported stuff
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = -1; // Via unistd.h
pub const _SC_PAGESIZE: ::c_int = 64;
pub const O_ACCMODE: ::c_int = 3;
pub const O_CLOEXEC: ::c_int = 0x100000; // fcntlcom
pub const O_EXCL: ::c_int = 0x0800;
pub const O_CREAT: ::c_int = 0x0200;
pub const O_TRUNC: ::c_int = 0x0400;
pub const O_APPEND: ::c_int = 0x0008;
pub const O_RDWR: ::c_int = 2;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDONLY: ::c_int = 0;
pub const O_NONBLOCK: ::c_int = 0x4;

#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum FILE {}
impl ::Copy for FILE {}
impl ::Clone for FILE {
    fn clone(&self) -> FILE {
        *self
    }
}
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum fpos_t {} // TODO: fill this out with a struct
impl ::Copy for fpos_t {}
impl ::Clone for fpos_t {
    fn clone(&self) -> fpos_t {
        *self
    }
}

extern {
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
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "fopen$UNIX2003"
    )]
    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "freopen$UNIX2003"
    )]
    pub fn freopen(
        filename: *const c_char,
        mode: *const c_char,
        file: *mut FILE,
    ) -> *mut FILE;
    pub fn fflush(file: *mut FILE) -> c_int;
    pub fn fclose(file: *mut FILE) -> c_int;
    pub fn remove(filename: *const c_char) -> c_int;
    pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn setvbuf(
        stream: *mut FILE,
        buffer: *mut c_char,
        mode: c_int,
        size: size_t,
    ) -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn getchar() -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE)
        -> *mut c_char;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "fputs$UNIX2003"
    )]
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fread(
        ptr: *mut c_void,
        size: size_t,
        nobj: size_t,
        stream: *mut FILE,
    ) -> size_t;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "fwrite$UNIX2003"
    )]
    pub fn fwrite(
        ptr: *const c_void,
        size: size_t,
        nobj: size_t,
        stream: *mut FILE,
    ) -> size_t;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn rewind(stream: *mut FILE);
    #[cfg_attr(target_os = "netbsd", link_name = "__fgetpos50")]
    pub fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int;
    #[cfg_attr(target_os = "netbsd", link_name = "__fsetpos50")]
    pub fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int;
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn perror(s: *const c_char);
    pub fn atoi(s: *const c_char) -> c_int;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "strtod$UNIX2003"
    )]
    pub fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double;
    pub fn strtol(
        s: *const c_char,
        endp: *mut *mut c_char,
        base: c_int,
    ) -> c_long;
    pub fn strtoul(
        s: *const c_char,
        endp: *mut *mut c_char,
        base: c_int,
    ) -> c_ulong;
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    //    pub fn _exit(status: c_int) -> !;
    pub fn atexit(cb: extern fn()) -> c_int;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "system$UNIX2003"
    )]
    pub fn system(s: *const c_char) -> c_int;
    pub fn getenv(s: *const c_char) -> *mut c_char;

    pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(
        dst: *mut c_char,
        src: *const c_char,
        n: size_t,
    ) -> *mut c_char;
    pub fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char;
    pub fn strncat(
        s: *mut c_char,
        ct: *const c_char,
        n: size_t,
    ) -> *mut c_char;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int;
    pub fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strdup(cs: *const c_char) -> *mut c_char;
    pub fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncasecmp(
        s1: *const c_char,
        s2: *const c_char,
        n: size_t,
    ) -> c_int;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "strerror$UNIX2003"
    )]
    pub fn strerror(n: c_int) -> *mut c_char;
    pub fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char;
    pub fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t;
    pub fn wcslen(buf: *const wchar_t) -> size_t;
    pub fn wcstombs(
        dest: *mut c_char,
        src: *const wchar_t,
        n: size_t,
    ) -> ::size_t;

    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(
        dest: *mut c_void,
        src: *const c_void,
        n: size_t,
    ) -> *mut c_void;
    pub fn memmove(
        dest: *mut c_void,
        src: *const c_void,
        n: size_t,
    ) -> *mut c_void;
    pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
}

extern {
    #[cfg_attr(target_os = "netbsd", link_name = "__getpwnam50")]
    pub fn getpwnam(name: *const ::c_char) -> *mut passwd;
    #[cfg_attr(target_os = "netbsd", link_name = "__getpwuid50")]
    pub fn getpwuid(uid: ::uid_t) -> *mut passwd;

    pub fn fprintf(
        stream: *mut ::FILE,
        format: *const ::c_char,
        ...
    ) -> ::c_int;
    pub fn printf(format: *const ::c_char, ...) -> ::c_int;
    pub fn snprintf(
        s: *mut ::c_char,
        n: ::size_t,
        format: *const ::c_char,
        ...
    ) -> ::c_int;
    pub fn sprintf(s: *mut ::c_char, format: *const ::c_char, ...) -> ::c_int;
    pub fn fscanf(
        stream: *mut ::FILE,
        format: *const ::c_char,
        ...
    ) -> ::c_int;
    pub fn scanf(format: *const ::c_char, ...) -> ::c_int;
    pub fn sscanf(s: *const ::c_char, format: *const ::c_char, ...)
        -> ::c_int;
    pub fn getchar_unlocked() -> ::c_int;
    pub fn putchar_unlocked(c: ::c_int) -> ::c_int;

    pub fn stat(path: *const c_char, buf: *mut stat) -> ::c_int;

    pub fn pclose(stream: *mut ::FILE) -> ::c_int;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "fdopen$UNIX2003"
    )]
    pub fn fdopen(fd: ::c_int, mode: *const c_char) -> *mut ::FILE;
    pub fn fileno(stream: *mut ::FILE) -> ::c_int;

    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "open$UNIX2003"
    )]
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "creat$UNIX2003"
    )]
    pub fn creat(path: *const c_char, mode: mode_t) -> ::c_int;

    pub fn fdopendir(fd: ::c_int) -> *mut ::DIR;

    pub fn rewinddir(dirp: *mut ::DIR);

    pub fn openat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        flags: ::c_int,
        ...
    ) -> ::c_int;
    pub fn fchmodat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        mode: ::mode_t,
        flags: ::c_int,
    ) -> ::c_int;
    pub fn fchown(fd: ::c_int, owner: ::uid_t, group: ::gid_t) -> ::c_int;
    pub fn fchownat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        owner: ::uid_t,
        group: ::gid_t,
        flags: ::c_int,
    ) -> ::c_int;
    #[cfg_attr(target_os = "macos", link_name = "fstatat$INODE64")]
    #[cfg_attr(target_os = "freebsd", link_name = "fstatat@FBSD_1.1")]
    pub fn fstatat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        buf: *mut stat,
        flags: ::c_int,
    ) -> ::c_int;
    pub fn linkat(
        olddirfd: ::c_int,
        oldpath: *const ::c_char,
        newdirfd: ::c_int,
        newpath: *const ::c_char,
        flags: ::c_int,
    ) -> ::c_int;
    pub fn mkdirat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        mode: ::mode_t,
    ) -> ::c_int;
    pub fn readlinkat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        buf: *mut ::c_char,
        bufsiz: ::size_t,
    ) -> ::ssize_t;
    pub fn renameat(
        olddirfd: ::c_int,
        oldpath: *const ::c_char,
        newdirfd: ::c_int,
        newpath: *const ::c_char,
    ) -> ::c_int;
    pub fn symlinkat(
        target: *const ::c_char,
        newdirfd: ::c_int,
        linkpath: *const ::c_char,
    ) -> ::c_int;

    pub fn access(path: *const c_char, amode: ::c_int) -> ::c_int;
    pub fn alarm(seconds: ::c_uint) -> ::c_uint;
    pub fn fchdir(dirfd: ::c_int) -> ::c_int;
    pub fn chown(path: *const c_char, uid: uid_t, gid: gid_t) -> ::c_int;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "lchown$UNIX2003"
    )]
    pub fn lchown(path: *const c_char, uid: uid_t, gid: gid_t) -> ::c_int;
    pub fn execl(path: *const c_char, arg0: *const c_char, ...) -> ::c_int;
    pub fn execle(
        path: *const ::c_char,
        arg0: *const ::c_char,
        ...
    ) -> ::c_int;
    pub fn execlp(
        file: *const ::c_char,
        arg0: *const ::c_char,
        ...
    ) -> ::c_int;
    pub fn execv(prog: *const c_char, argv: *const *const c_char) -> ::c_int;
    pub fn execve(
        prog: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> ::c_int;
    /*
    pub fn execvp(c: *const c_char,
    argv: *const *const c_char) -> ::c_int;
     */
    //    pub fn fork() -> pid_t;
    pub fn fpathconf(filedes: ::c_int, name: ::c_int) -> c_long;
    pub fn getegid() -> gid_t;
    pub fn geteuid() -> uid_t;
    pub fn getgroups(ngroups_max: ::c_int, groups: *mut gid_t) -> ::c_int;
    pub fn getlogin() -> *mut c_char;
    pub fn getopt(
        argc: ::c_int,
        argv: *const *mut c_char,
        optstr: *const c_char,
    ) -> ::c_int;
    pub fn pathconf(path: *const c_char, name: ::c_int) -> c_long;
    pub fn pause() -> ::c_int;
    pub fn seteuid(uid: uid_t) -> ::c_int;
    pub fn setegid(gid: gid_t) -> ::c_int;
    pub fn setpgid(pid: pid_t, pgid: pid_t) -> ::c_int;
    pub fn setsid() -> pid_t;
    pub fn sleep(secs: ::c_uint) -> ::c_uint;
    pub fn tcgetpgrp(fd: ::c_int) -> pid_t;
    pub fn tcsetpgrp(fd: ::c_int, pgrp: ::pid_t) -> ::c_int;
    pub fn ttyname(fd: ::c_int) -> *mut c_char;
    pub fn wait(status: *mut ::c_int) -> pid_t;
    /*
    pub fn pread(fd: ::c_int, buf: *mut ::c_void, count: ::size_t,
    offset: off_t) -> ::ssize_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
    link_name = "pwrite$UNIX2003")]
    pub fn pwrite(fd: ::c_int, buf: *const ::c_void, count: ::size_t,
    offset: off_t) -> ::ssize_t;
     */
    pub fn umask(mask: mode_t) -> mode_t;

    //    #[cfg_attr(target_os = "netbsd", link_name = "__utime50")]
    //    pub fn utime(file: *const c_char, buf: *const utimbuf) -> ::c_int;

    /*
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
    link_name = "kill$UNIX2003")]
    pub fn kill(pid: pid_t, sig: ::c_int) -> ::c_int;
     */
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "killpg$UNIX2003"
    )]
    pub fn killpg(pgrp: pid_t, sig: ::c_int) -> ::c_int;

    pub fn mlock(addr: *const ::c_void, len: ::size_t) -> ::c_int;
    pub fn munlock(addr: *const ::c_void, len: ::size_t) -> ::c_int;
    pub fn mlockall(flags: ::c_int) -> ::c_int;
    pub fn munlockall() -> ::c_int;

    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "mmap$UNIX2003"
    )]
    pub fn mmap(
        addr: *mut ::c_void,
        len: ::size_t,
        prot: ::c_int,
        flags: ::c_int,
        fd: ::c_int,
        offset: off_t,
    ) -> *mut ::c_void;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "munmap$UNIX2003"
    )]
    pub fn munmap(addr: *mut ::c_void, len: ::size_t) -> ::c_int;

    pub fn if_nametoindex(ifname: *const c_char) -> ::c_uint;
    pub fn if_indextoname(
        ifindex: ::c_uint,
        ifname: *mut ::c_char,
    ) -> *mut ::c_char;

    pub fn truncate(path: *const c_char, length: off_t) -> ::c_int;

    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "getrlimit$UNIX2003"
    )]
    pub fn getrlimit(resource: ::c_int, rlim: *mut rlimit) -> ::c_int;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "setrlimit$UNIX2003"
    )]
    pub fn setrlimit(resource: ::c_int, rlim: *const rlimit) -> ::c_int;
    //    #[cfg_attr(target_os = "netbsd", link_name = "__getrusage50")]
    //    pub fn getrusage(resource: ::c_int, usage: *mut rusage) -> ::c_int;

    /*
    #[cfg_attr(any(target_os = "macos", target_os = "ios"),
    link_name = "realpath$DARWIN_EXTSN")]
    pub fn realpath(pathname: *const ::c_char, resolved: *mut ::c_char)
                    -> *mut ::c_char;
     */
    pub fn flock(fd: ::c_int, operation: ::c_int) -> ::c_int;

    #[cfg_attr(target_os = "netbsd", link_name = "__gettimeofday50")]
    pub fn gettimeofday(tp: *mut ::timeval, tz: *mut ::c_void) -> ::c_int;
    pub fn pthread_exit(value: *mut ::c_void);
    pub fn pthread_attr_setdetachstate(
        attr: *mut ::pthread_attr_t,
        state: ::c_int,
    ) -> ::c_int;

    pub fn strerror_r(
        errnum: ::c_int,
        buf: *mut c_char,
        buflen: ::size_t,
    ) -> ::c_int;

    pub fn sigaction(
        signum: ::c_int,
        act: *const sigaction,
        oldact: *mut sigaction,
    ) -> ::c_int;

    #[cfg_attr(target_os = "netbsd", link_name = "__utimes50")]
    pub fn utimes(
        filename: *const ::c_char,
        times: *const ::timeval,
    ) -> ::c_int;
    pub fn dlopen(filename: *const ::c_char, flag: ::c_int) -> *mut ::c_void;
    pub fn dlerror() -> *mut ::c_char;
    pub fn dlsym(
        handle: *mut ::c_void,
        symbol: *const ::c_char,
    ) -> *mut ::c_void;
    pub fn dlclose(handle: *mut ::c_void) -> ::c_int;
    pub fn res_init() -> ::c_int;
    /*
    #[cfg_attr(target_os = "netbsd", link_name = "__gmtime_r50")]
    pub fn gmtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm;
    #[cfg_attr(target_os = "netbsd", link_name = "__localtime_r50")]
    pub fn localtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
    link_name = "mktime$UNIX2003")]
    #[cfg_attr(target_os = "netbsd", link_name = "__mktime50")]
    pub fn mktime(tm: *mut tm) -> time_t;
    #[cfg_attr(target_os = "netbsd", link_name = "__time50")]
    pub fn time(time: *mut time_t) -> time_t;
    #[cfg_attr(target_os = "netbsd", link_name = "__gmtime50")]
    pub fn gmtime(time_p: *const time_t) -> *mut tm;
    #[cfg_attr(target_os = "netbsd", link_name = "__locatime50")]
    pub fn localtime(time_p: *const time_t) -> *mut tm;
     */
    #[cfg_attr(target_os = "netbsd", link_name = "__difftime50")]
    pub fn difftime(time1: time_t, time0: time_t) -> ::c_double;

    #[cfg_attr(target_os = "netbsd", link_name = "__mknod50")]
    #[cfg_attr(target_os = "freebsd", link_name = "mknod@FBSD_1.0")]
    pub fn mknod(
        pathname: *const ::c_char,
        mode: ::mode_t,
        dev: ::dev_t,
    ) -> ::c_int;
    pub fn gethostname(name: *mut ::c_char, len: ::size_t) -> ::c_int;
    //    pub fn getservbyname(name: *const ::c_char,
    //                         proto: *const ::c_char) -> *mut servent;
    //    pub fn getprotobyname(name: *const ::c_char) -> *mut protoent;
    //    pub fn getprotobynumber(proto: ::c_int) -> *mut protoent;
    pub fn chroot(name: *const ::c_char) -> ::c_int;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "usleep$UNIX2003"
    )]
    pub fn usleep(secs: ::c_uint) -> ::c_int;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "putenv$UNIX2003"
    )]
    pub fn putenv(string: *mut c_char) -> ::c_int;
    #[cfg_attr(target_os = "netbsd", link_name = "__select50")]
    //    pub fn select(nfds: ::c_int,
    //                  readfs: *mut fd_set,
    //                  writefds: *mut fd_set,
    //                  errorfds: *mut fd_set,
    //                  timeout: *mut timeval) -> ::c_int;
    #[cfg_attr(target_os = "netbsd", link_name = "__setlocale50")]
    pub fn setlocale(
        category: ::c_int,
        locale: *const ::c_char,
    ) -> *mut ::c_char;
    //    pub fn localeconv() -> *mut lconv;

    pub fn sigprocmask(
        how: ::c_int,
        set: *const sigset_t,
        oldset: *mut sigset_t,
    ) -> ::c_int;
    #[cfg_attr(target_os = "netbsd", link_name = "__sigpending14")]
    pub fn sigpending(set: *mut sigset_t) -> ::c_int;

    pub fn getsid(pid: pid_t) -> pid_t;

    pub fn mkfifo(path: *const c_char, mode: mode_t) -> ::c_int;

    pub fn fseeko(
        stream: *mut ::FILE,
        offset: ::off_t,
        whence: ::c_int,
    ) -> ::c_int;
    pub fn ftello(stream: *mut ::FILE) -> ::off_t;
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "tcdrain$UNIX2003"
    )]
    pub fn tcdrain(fd: ::c_int) -> ::c_int;
    pub fn tcflow(fd: ::c_int, action: ::c_int) -> ::c_int;
    pub fn tcflush(fd: ::c_int, action: ::c_int) -> ::c_int;
    pub fn tcgetsid(fd: ::c_int) -> ::pid_t;
    pub fn tcsendbreak(fd: ::c_int, duration: ::c_int) -> ::c_int;
    pub fn mkstemp(template: *mut ::c_char) -> ::c_int;
    pub fn mkdtemp(template: *mut ::c_char) -> *mut ::c_char;

    pub fn tmpnam(ptr: *mut ::c_char) -> *mut ::c_char;

    pub fn openlog(ident: *const ::c_char, logopt: ::c_int, facility: ::c_int);
    pub fn closelog();
    pub fn setlogmask(maskpri: ::c_int) -> ::c_int;
    #[cfg_attr(target_os = "macos", link_name = "syslog$DARWIN_EXTSN")]
    pub fn syslog(priority: ::c_int, message: *const ::c_char, ...);
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "nice$UNIX2003"
    )]
    pub fn nice(incr: ::c_int) -> ::c_int;

    pub fn grantpt(fd: ::c_int) -> ::c_int;
    pub fn posix_openpt(flags: ::c_int) -> ::c_int;
    pub fn ptsname(fd: ::c_int) -> *mut ::c_char;
    pub fn unlockpt(fd: ::c_int) -> ::c_int;

    pub fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn getline(
        lineptr: *mut *mut c_char,
        n: *mut size_t,
        stream: *mut FILE,
    ) -> ssize_t;

    pub fn _rtld_dladdr(addr: *const ::c_void, info: *mut Dl_info) -> ::c_int;
}

extern {
    // this is gonna be a big one

    // stdlib.h
    // This function may not be defined for armv7
    pub fn memalign(block_size: ::size_t, size_arg: ::size_t)
        -> *mut ::c_void;

    // ioLib.h
    pub fn getcwd(buf: *mut ::c_char, size: ::size_t) -> *mut ::c_char;

    // ioLib.h
    pub fn chdir(attr: *const ::c_char) -> ::c_int;

    // pthread.h
    pub fn pthread_mutexattr_init(
        /* PTHREAD STUFF */
        attr: *mut pthread_mutexattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutexattr_destroy(
        attr: *mut pthread_mutexattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutexattr_settype(
        pAttr: *mut ::pthread_mutexattr_t,
        pType: ::c_int,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutex_init(
        mutex: *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> ::c_int;

    // pthread.h
    pub fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> ::c_int;

    // pthread.h
    pub fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> ::c_int;

    // pthread.h
    pub fn pthread_mutex_timedlock(
        attr: *mut pthread_mutex_t,
        spec: *const timespec,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> ::c_int;

    // pthread.h
    pub fn pthread_attr_setname(
        pAttr: *mut ::pthread_attr_t,
        name: *mut ::c_char,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_attr_setstacksize(
        attr: *mut ::pthread_attr_t,
        stacksize: ::size_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_attr_getstacksize(
        attr: *const ::pthread_attr_t,
        size: *mut ::size_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_attr_init(attr: *mut ::pthread_attr_t) -> ::c_int;

    // pthread.h
    pub fn pthread_create(
        pThread: *mut ::pthread_t,
        pAttr: *const ::pthread_attr_t,
        start_routine: extern fn(*mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_attr_destroy(thread: *mut ::pthread_attr_t) -> ::c_int;

    // pthread.h
    pub fn pthread_detach(thread: ::pthread_t) -> ::c_int;

    // int pthread_atfork (void (*)(void), void (*)(void), void (*)(void));
    pub fn pthread_atfork(
        prepare: ::Option<unsafe extern fn()>,
        parent: ::Option<unsafe extern fn()>,
        child: ::Option<unsafe extern fn()>,
    ) -> ::c_int;
    // stat.h
    pub fn fstat(fildes: ::c_int, buf: *mut stat) -> ::c_int;

    // stat.h
    pub fn lstat(path: *const ::c_char, buf: *mut stat) -> ::c_int;

    // unistd.h
    pub fn ftruncate(fd: ::c_int, length: off_t) -> ::c_int;

    // dirent.h
    pub fn readdir_r(
        pDir: *mut ::DIR,
        entry: *mut ::dirent,
        result: *mut *mut ::dirent,
    ) -> ::c_int;

    // dirent.h
    pub fn readdir(pDir: *mut ::DIR) -> *mut ::dirent;

    // fcntl.h or
    // ioLib.h
    pub fn open(
        // this might be hacked
        path: *const ::c_char,
        oflag: ::c_int,
        ...
    ) -> ::c_int;

    // poll.h
    pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: ::c_int) -> ::c_int;

    // pthread.h
    pub fn pthread_condattr_init(attr: *mut ::pthread_condattr_t) -> ::c_int;

    // pthread.h
    pub fn pthread_condattr_destroy(
        attr: *mut ::pthread_condattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_condattr_getclock(
        pAttr: *const ::pthread_condattr_t,
        pClockId: *mut ::clockid_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_condattr_setclock(
        pAttr: *mut ::pthread_condattr_t,
        clockId: ::clockid_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_cond_init(
        cond: *mut ::pthread_cond_t,
        attr: *const ::pthread_condattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> ::c_int;

    // pthread.h
    pub fn pthread_cond_signal(cond: *mut ::pthread_cond_t) -> ::c_int;

    // pthread.h
    pub fn pthread_cond_broadcast(cond: *mut ::pthread_cond_t) -> ::c_int;

    // pthread.h
    pub fn pthread_cond_wait(
        cond: *mut ::pthread_cond_t,
        mutex: *mut ::pthread_mutex_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlockattr_init(
        attr: *mut ::pthread_rwlockattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlockattr_destroy(
        attr: *mut ::pthread_rwlockattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlockattr_setmaxreaders(
        attr: *mut ::pthread_rwlockattr_t,
        attr2: ::c_uint,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlock_init(
        attr: *mut ::pthread_rwlock_t,
        host: *const ::pthread_rwlockattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlock_destroy(attr: *mut ::pthread_rwlock_t) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlock_rdlock(attr: *mut ::pthread_rwlock_t) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlock_tryrdlock(attr: *mut ::pthread_rwlock_t) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlock_timedrdlock(
        attr: *mut ::pthread_rwlock_t,
        host: *const ::timespec,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlock_wrlock(attr: *mut ::pthread_rwlock_t) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlock_trywrlock(attr: *mut ::pthread_rwlock_t) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlock_timedwrlock(
        attr: *mut ::pthread_rwlock_t,
        host: *const ::timespec,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_rwlock_unlock(attr: *mut ::pthread_rwlock_t) -> ::c_int;

    // pthread.h
    pub fn pthread_key_create(
        key: *mut ::pthread_key_t,
        dtor: ::Option<unsafe extern fn(*mut ::c_void)>,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_key_delete(key: ::pthread_key_t) -> ::c_int;

    // pthread.h
    pub fn pthread_setspecific(
        key: ::pthread_key_t,
        value: *const ::c_void,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_getspecific(key: ::pthread_key_t) -> *mut ::c_void;

    // pthread.h
    pub fn pthread_cond_timedwait(
        cond: *mut ::pthread_cond_t,
        mutex: *mut ::pthread_mutex_t,
        abstime: *const ::timespec,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_attr_getname(
        attr: *mut ::pthread_attr_t,
        name: *mut *mut ::c_char,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_join(
        thread: ::pthread_t,
        status: *mut *mut ::c_void,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_self() -> ::pthread_t;

    // clockLib.h
    pub fn clock_gettime(
        clock_id: ::clockid_t,
        tp: *mut ::timespec,
    ) -> ::c_int;

    // clockLib.h
    pub fn clock_settime(
        clock_id: ::clockid_t,
        tp: *const ::timespec,
    ) -> ::c_int;

    // clockLib.h
    pub fn clock_getres(
        clock_id: ::clockid_t,
        res: *mut ::timespec,
    ) -> ::c_int;

    // clockLib.h
    pub fn clock_nanosleep(
        clock_id: ::clockid_t,
        flags: ::c_int,
        rqtp: *const ::timespec,
        rmtp: *mut ::timespec,
    ) -> ::c_int;

    // timerLib.h
    pub fn nanosleep(
        rqtp: *const ::timespec,
        rmtp: *mut ::timespec,
    ) -> ::c_int;

    // socket.h
    pub fn accept(
        s: ::c_int,
        addr: *mut ::sockaddr,
        addrlen: *mut ::socklen_t,
    ) -> ::c_int;

    // socket.h
    pub fn bind(fd: ::c_int, addr: *const sockaddr, len: socklen_t)
        -> ::c_int;

    // socket.h
    pub fn connect(
        s: ::c_int,
        name: *const ::sockaddr,
        namelen: ::socklen_t,
    ) -> ::c_int;

    // socket.h
    pub fn getpeername(
        s: ::c_int,
        name: *mut ::sockaddr,
        namelen: *mut ::socklen_t,
    ) -> ::c_int;

    // socket.h
    pub fn getsockname(
        socket: ::c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> ::c_int;

    // socket.h
    pub fn getsockopt(
        sockfd: ::c_int,
        level: ::c_int,
        optname: ::c_int,
        optval: *mut ::c_void,
        optlen: *mut ::socklen_t,
    ) -> ::c_int;

    // socket.h
    pub fn listen(socket: ::c_int, backlog: ::c_int) -> ::c_int;

    // socket.h
    pub fn recv(
        s: ::c_int,
        buf: *mut ::c_void,
        bufLen: ::size_t,
        flags: ::c_int,
    ) -> ::ssize_t;

    // socket.h
    pub fn recvfrom(
        s: ::c_int,
        buf: *mut ::c_void,
        bufLen: ::size_t,
        flags: ::c_int,
        from: *mut ::sockaddr,
        pFromLen: *mut ::socklen_t,
    ) -> ::ssize_t;

    // socket.h
    pub fn send(
        socket: ::c_int,
        buf: *const ::c_void,
        len: ::size_t,
        flags: ::c_int,
    ) -> ::ssize_t;

    // socket.h
    pub fn sendto(
        socket: ::c_int,
        buf: *const ::c_void,
        len: ::size_t,
        flags: ::c_int,
        addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ::ssize_t;

    // socket.h
    pub fn setsockopt(
        socket: ::c_int,
        level: ::c_int,
        name: ::c_int,
        value: *const ::c_void,
        option_len: socklen_t,
    ) -> ::c_int;

    // socket.h
    pub fn shutdown(s: ::c_int, how: ::c_int) -> ::c_int;

    // socket.h
    pub fn socket(
        domain: ::c_int,
        _type: ::c_int,
        protocol: ::c_int,
    ) -> ::c_int;

    pub fn socketpair(
        // Doesn't exist
        domain: ::c_int,
        type_: ::c_int,
        protocol: ::c_int,
        socket_vector: *mut ::c_int,
    ) -> ::c_int;

    // icotl.h
    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;

    // fcntl.h
    pub fn fcntl(fd: ::c_int, cmd: ::c_int, ...) -> ::c_int;

    // ntp_rfc2553.h for kernel
    // netdb.h for user
    pub fn gai_strerror(errcode: ::c_int) -> *mut ::c_char;

    // ioLib.h or
    // unistd.h
    pub fn close(fd: ::c_int) -> ::c_int;

    // ioLib.h or
    // unistd.h
    pub fn read(
        // Since this is from FD< big errors might happen
        fd: ::c_int,
        buf: *mut ::c_void,
        count: ::size_t,
    ) -> ::ssize_t;

    // ioLib.h or
    // unistd.h
    pub fn write(
        fd: ::c_int,
        buf: *const ::c_void,
        count: ::size_t,
    ) -> ::ssize_t;

    // ioLib.h or
    // unistd.h
    pub fn isatty(fd: ::c_int) -> ::c_int;

    // ioLib.h or
    // unistd.h
    pub fn dup(src: ::c_int) -> ::c_int;

    // ioLib.h or
    // unistd.h
    pub fn dup2(src: ::c_int, dst: ::c_int) -> ::c_int;

    // ioLib.h or
    // unistd.h
    pub fn pipe(fds: *mut ::c_int) -> ::c_int;

    // ioLib.h or
    // unistd.h
    pub fn unlink(pathname: *const ::c_char) -> ::c_int;

    // unistd.h and
    // ioLib.h
    pub fn lseek(fd: ::c_int, offset: off_t, whence: ::c_int) -> off_t;

    // netdb.h
    pub fn getaddrinfo(
        node: *const ::c_char,
        service: *const ::c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> ::c_int;

    // netdb.h
    pub fn freeaddrinfo(res: *mut addrinfo);

    // signal.h
    pub fn signal(
        // Probably wrong ...
        signum: ::c_int,
        handler: sighandler_t,
    ) -> sighandler_t;

    // unistd.h
    pub fn getpid() -> ::c_int; //should be pid_t, but is being dodged

    // unistd.h
    pub fn getppid() -> ::c_int;

    // wait.h
    pub fn waitpid(
        pid: ::c_int, //should be pid_t, but is being dodged
        status: *mut ::c_int,
        optons: ::c_int,
    ) -> ::c_int; //should be pid_t, but is being dodged

    // unistd.h
    pub fn sysconf(attr: ::c_int) -> ::c_long;

    // unistd.h
    // For user space, return value is static inline int
    // For kernel space, exactly how it should be
    pub fn getpagesize() -> ::c_int;

    // stdlib.h
    pub fn setenv(
        // setenv.c
        envVarName: *const ::c_char,
        envVarValue: *const ::c_char,
        overwrite: ::c_int,
    ) -> ::c_int;

    // stdlib.h
    pub fn unsetenv(
        // setenv.c
        envVarName: *const ::c_char,
    ) -> ::c_int;

    // unistd.h
    pub fn link(src: *const ::c_char, dst: *const ::c_char) -> ::c_int;

    // unistd.h
    pub fn readlink(
        path: *const ::c_char,
        buf: *mut ::c_char,
        bufsize: ::size_t,
    ) -> ::ssize_t;

    // unistd.h
    pub fn symlink(path1: *const ::c_char, path2: *const ::c_char) -> ::c_int;

    // dirent.h
    pub fn opendir(name: *const ::c_char) -> *mut ::DIR;

    // unistd.h
    pub fn rmdir(path: *const ::c_char) -> ::c_int;

    // stat.h
    pub fn mkdir(dirName: *const ::c_char, mode: ::mode_t) -> ::c_int;

    // stat.h
    pub fn chmod(path: *const ::c_char, mode: ::mode_t) -> ::c_int;

    // stat.h
    pub fn fchmod(attr1: ::c_int, attr2: ::mode_t) -> ::c_int;

    // unistd.h
    pub fn fsync(fd: ::c_int) -> ::c_int;

    // dirent.h
    pub fn closedir(ptr: *mut ::DIR) -> ::c_int;

    pub fn pwrite64(
        fd: ::c_int, // if you want to use fd, you gotta fix these
        buf: *const ::c_void,
        count: ::size_t,
        offset: off64_t,
    ) -> ::ssize_t;

    pub fn pread64(
        fd: ::c_int,
        buf: *const ::c_void,
        count: ::size_t,
        offset: off64_t,
    ) -> ::ssize_t;

    // sched.h
    pub fn sched_yield() -> ::c_int;

    // errnoLib.h
    pub fn errnoSet(err: ::c_int) -> ::c_int;

    // errnoLib.h
    pub fn errnoGet() -> ::c_int;

    pub fn fork(// Does not exist at all
    ) -> ::c_int;

    // unistd.h
    pub fn _exit(status: ::c_int) -> !;

    // unistd.h
    pub fn setgid(gid: ::gid_t) -> ::c_int;

    // unistd.h
    pub fn getgid() -> ::gid_t;

    // unistd.h
    pub fn setuid(uid: ::uid_t) -> ::c_int;

    // unistd.h
    pub fn getuid() -> ::uid_t;

    pub fn setgroups(
        // Does not exist at all
        ngroups: ::c_int,
        grouplist: *const ::gid_t,
    ) -> ::c_int;

    // signal.h
    pub fn sigemptyset(__set: *mut sigset_t) -> ::c_int;

    // pthread.h for kernel
    // signal.h for user
    pub fn pthread_sigmask(
        __how: ::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::c_int;

    pub fn execvp(
        // Does not exist at all
        c: *const ::c_char,
        argv: *const *const ::c_char,
    ) -> ::c_int;

    // signal.h for user
    pub fn kill(
        __pid: ::c_int, //should be pid_t, but is being dodged
        __signo: ::c_int,
    ) -> ::c_int;

    // signal.h for user
    pub fn sigqueue(
        __pid: ::c_int, //should be pid_t, but is being dodged
        __signo: ::c_int,
        __value: ::size_t, // Actual type is const union sigval value,
                           // which is a union of int and void *
    ) -> ::c_int;

    // signal.h for user
    pub fn _sigqueue(
        rtpId: ::RTP_ID,
        signo: ::c_int,
        pValue: *mut ::size_t, // Actual type is const union * sigval value,
        // which is a union of int and void *
        sigCode: ::c_int,
    ) -> ::c_int;

    // signal.h
    // It seems like for kernel space, this function doesn't actually exist,
    // it just macros to kill
    pub fn taskKill(taskId: ::TASK_ID, signo: ::c_int) -> ::c_int;

    // signal.h
    pub fn raise(__signo: ::c_int) -> ::c_int;
    // taskLibCommon.h
    pub fn taskIdSelf() -> ::TASK_ID;

    // rtpLibCommon.h
    pub fn rtpInfoGet(rtpId: ::RTP_ID, rtpStruct: *mut ::RTP_DESC) -> ::c_int;

    // ioLib.h
    pub fn _realpath(
        fileName: *const ::c_char,
        resolvedName: *mut ::c_char,
    ) -> *mut ::c_char;

    // pathLib.h
    pub fn _pathIsAbsolute(
        filepath: *const ::c_char,
        pNameTail: *const *const ::c_char,
    ) -> bool;

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
}

pub fn dladdr(addr: *const ::c_void, info: *mut Dl_info) -> ::c_int {
    unsafe { _rtld_dladdr(addr, info) }
}

//Dummy functions, these don't really exist in VxWorks.

// wait.h macros
pub fn WIFEXITED(status: ::c_int) -> bool {
    (status & 0xFF00) == 0
}
pub fn WIFSIGNALED(status: ::c_int) -> bool {
    (status & 0xFF00) != 0
}
pub fn WIFSTOPPED(status: ::c_int) -> bool {
    (status & 0xFF0000) != 0
}
pub fn WEXITSTATUS(status: ::c_int) -> ::c_int {
    status & 0xFF
}
pub fn WTERMSIG(status: ::c_int) -> ::c_int {
    (status >> 8) & 0xFF
}
pub fn WSTOPSIG(status: ::c_int) -> ::c_int {
    (status >> 16) & 0xFF
}

pub fn pread(
    _fd: ::c_int,
    _buf: *mut ::c_void,
    _count: ::size_t,
    _offset: off64_t,
) -> ::ssize_t {
    -1
}

pub fn pwrite(
    _fd: ::c_int,
    _buf: *const ::c_void,
    _count: ::size_t,
    _offset: off64_t,
) -> ::ssize_t {
    -1
}
pub fn posix_memalign(
    memptr: *mut *mut ::c_void,
    align: ::size_t,
    size: ::size_t,
) -> ::c_int {
    // check to see if align is a power of 2 and if align is a multiple
    //  of sizeof(void *)
    if (align & align - 1 != 0) || (align % size_of::<::size_t>() != 0) {
        return ::EINVAL;
    }

    unsafe {
        // posix_memalign should not set errno
        let e = ::errnoGet();

        let temp = memalign(align, size);
        ::errnoSet(e as ::c_int);

        if temp.is_null() {
            ::ENOMEM
        } else {
            *memptr = temp;
            0
        }
    }
}

// From sysconf.c -> doesn't seem to be supported?
pub fn getpwuid_r(
    _uid: ::uid_t,
    _pwd: *mut passwd,
    _buf: *mut ::c_char,
    _buflen: ::size_t,
    _result: *mut *mut passwd,
) -> ::c_int {
    0
}

// VxWorks requires that resolvedName be allocated in userspace
pub fn realpath(
    fileName: *const ::c_char,
    resolvedName: *mut ::c_char,
) -> *mut ::c_char {
    unsafe {
        if resolvedName == null_mut::<::c_char>() {
            let emptyResolvedName =
                super::malloc(::_POSIX_PATH_MAX as _) as *mut ::c_char;
            let r = _realpath(fileName, emptyResolvedName);

            if r == null_mut::<::c_char>() {
                super::free(emptyResolvedName as *mut _);
            }
            r
        } else {
            _realpath(fileName, resolvedName)
        }
    }
}

cfg_if! {
    if #[cfg(libc_core_cvoid)] {
        pub use ::ffi::c_void;
    } else {
        // Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help
        // enable more optimization opportunities around it recognizing things
        // like malloc/free.
        #[repr(u8)]
        #[allow(missing_copy_implementations)]
        #[allow(missing_debug_implementations)]
        pub enum c_void {
            // Two dummy variants so the #[repr] attribute can be used.
            #[doc(hidden)]
            __variant1,
            #[doc(hidden)]
            __variant2,
        }
    }
}

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(any(target_arch = "arm"))] {
        mod arm;
        pub use self::arm::*;
    } else if #[cfg(any(target_arch = "armv7"))] {
        mod armv7;
        pub use self::armv7::*;
    }  else if #[cfg(any(target_arch = "x86"))] {
        mod x86;
        pub use self::x86::*;
    } else if #[cfg(any(target_arch = "x86_64"))] {
        mod x86_64;
        pub use self::x86_64::*;
    } else if #[cfg(any(target_arch = "powerpc"))] {
        mod powerpc;
        pub use self::powerpc::*;
    } else if #[cfg(any(target_arch = "powerpc64"))] {
        mod powerpc64;
        pub use self::powerpc64::*;
    } else {
        // Unknown target_arch
    }
}
