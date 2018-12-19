//! Hacking together the definitions for VxWorks Bindings
use dox::mem; /*TODO: Figure out what this even does */
use core::ptr::null;
use core::ptr::null_mut;
use core::mem::size_of;

//  Notes: 
//  We might need to define c_int and c_uint, let's figure that out on the fly 
//  We should really make sense of the scope resolution, because we currently are looking pretty dumb ...
//  Glob_T is not needed (?)
//  This is still massively incomplete, probably not enough to compile std
//
//  Currently, we assume _WRS_KERNEL is not defined in the C-libraries.
//  (they are only defined for kernel-side builds only, and we are building for rtps currently)
//  In case someone tries to compile for kernel, I've added kernel specific structures and functions
//  using cfg. To enable them, compile with the feature _WRS_KERNEL on.
//  (This will also make sure that user-side definitions don't compile as well, at least for
//  most of them anyway)
//
//  The type defs can be different between different compilers and architecture.
//  An easy way to check the exact types of a faulting type / structure is to 
//  use gdb on the compiled executable, and use ptype {faulting type} for a
//  faulting type and ptype struct {struct name} for a faulting structure.
//  If it shows "No symbol {type / struct name}", that just means that it 
//  doesn't get called anywhere in the executable, hence it hasn't been
//  included during the linking process.
//
//  !IMPORTANT:
//  Another tip in finding the type definitions is to use the search
//	functionality in workspace (flashlight icon in the toolbar).
//	You can search for all USER definitions by setting the 'Look in:'
//	parameter to your VSB's usr folder, and the 'File types:' parameter
//	to *.h.
//
//	There are some types that were omitted to be defined, but rather
//	it was replaced with the type it is defined as in C.
//	For example, STATUS is all set to c_int, pid_t is all set to c_int.
//	(I'm not too sure why this was done, but it seems to have been the way
//	it was done by the previous co-op, so kept it for now.)


pub enum DIR{}

/*
 * vxWorks doesn't define types for these POSIX types:
 *
 * fsblkcnt_t
 * fsfilcnt_t
 * id_t
 * pthread_barrier_t
 * pthread_barrierattr_t
 * pthread_rwlock_t
 * pthread_rwlockattr_t
 * pthread_spinlock_t
 * trace_attr_t
 * trace_event_id_t
 * trace_event_set_t
 * trace_id_t
 */


// Primitive types. If you are unsure, create an RTP and just run
// sizeof on them.
pub type c_char = i8;

pub type blkcnt_t  = ::c_long; // will probably have to update all of this for each target past version one .. why are we still hacking this?
pub type blksize_t = ::c_long; // *both blkcnt_t and blksize_t were set to i32 before
pub type ino_t     = ::c_ulong;  // vxworks default ino_t is 64 bits, which is weird because it supports POSIX
pub type ino32_t   = u32;
// ssize_t is i32 for LP32 and i64 for LP64, which is accounted for already
// same goes for size_t

// #[cfg(feature = "__RTP__")]
pub type off_t = ::c_longlong;
//#[cfg(not(feature = "__RTP__"))]
//pub type off_t = ::c_long;

pub type rlim_t       = ::c_ulonglong; //might not be right
pub type suseconds_t  = ::c_long;
pub type time_t       = ::c_long;
pub type wchar_t      = ::c_int;
pub type errno_t      = ::c_int;
pub type sighandler_t = ::size_t;
pub type in_port_t    = u16;

pub type useconds_t = ::c_ulong;

#[cfg(feature = "SOCKLEN_T_UNSIGNED")]
pub type socklen_t = ::c_uint;
#[cfg(not(feature = "SOCKLEN_T_UNSIGNED"))]
pub type socklen_t = ::c_int;

pub type pthread_t = ::c_ulong;

#[cfg(feature = "_WRS_KERNEL")]
pub type clockid_t = ::size_t; //set to u64 with LP64 and u32 with LP32

#[cfg(not(feature = "_WRS_KERNEL"))]
pub type clockid_t = ::c_int;


pub const STDIN_FILENO  : ::c_int = 0;
pub const STDOUT_FILENO : ::c_int = 1;
pub const STDERR_FILENO : ::c_int = 2;

pub const EXIT_SUCCESS : ::c_int = 0;
pub const EXIT_FAILURE : ::c_int = 1;

// EAI - does this even exist in VxWorks?
pub const EAI_SERVICE : ::c_int = 9;
pub const EAI_SOCKTYPE: ::c_int = 10;
pub const EAI_SYSTEM  : ::c_int = 11;

//Clock Lib Stuff
pub const CLOCK_REALTIME           : ::c_int = 0x0;
pub const CLOCK_MONOTONIC          : ::c_int = 0x1;
pub const CLOCK_PROCESS_CPUTIME_ID : ::c_int = 0x2;
pub const CLOCK_THREAD_CPUTIME_ID  : ::c_int = 0x3;
pub const TIMER_ABSTIME            : ::c_int = 0x1;
pub const TIME_RELTIME             : ::c_int = 0xFFFFFFFE;

// PTHREAD STUFF
pub const PTHREAD_INITIALIZED_OBJ : ::c_int = 0xF70990EF; // This is an overflow, is that a bad thing? 
pub const PTHREAD_DESTROYED_OBJ   : ::c_int = -1;
pub const PTHREAD_VALID_OBJ       : ::c_int = 0xEC542A37; // This is an overflow, is that a bad thing? 
pub const PTHREAD_INVALID_OBJ     : ::c_int = -1;
pub const PTHREAD_UNUSED_YET_OBJ  : ::c_int = -1;

pub const PTHREAD_PRIO_NONE    : ::c_int = 0;
pub const PTHREAD_PRIO_INHERIT : ::c_int = 1;
pub const PTHREAD_PRIO_PROTECT : ::c_int = 2;

pub const PTHREAD_MUTEX_NORMAL     : ::c_int = 0;
pub const PTHREAD_MUTEX_ERRORCHECK : ::c_int = 1;
pub const PTHREAD_MUTEX_RECURSIVE  : ::c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT    : ::c_int = PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_STACK_MIN        : usize = 4096;

pub const EFAULT : ::c_int = 14;
pub const EBUSY : ::c_int = 16;
pub const EEXIST : ::c_int = 17;
pub const ENODEV : ::c_int = 19;
pub const EINVAL : ::c_int = 22;
pub const EPIPE : ::c_int = 32;
pub const ERANGE : ::c_int = 34;

// ERRNO STUFF
pub const EPERM           : ::c_int = 1;         /* Not owner */
pub const ENOENT          : ::c_int = 2;        /* No such file or directory */
pub const ESRCH           : ::c_int = 3;        /* No such process */
pub const EINTR           : ::c_int = 4;        /* Interrupted system call */
pub const EIOA            : ::c_int = 5;        /* I/O error */
pub const ENXIO           : ::c_int = 6;         /* No such device or address */
pub const E2BIG           : ::c_int = 7;        /* Arg list too long */
pub const ENOEXEC         : ::c_int = 8;    /* Exec format error */
pub const EBADF           : ::c_int = 9;        /* Bad file number */
pub const CHILD           : ::c_int = 10;        /* No children */
pub const EAGAIN          : ::c_int = 11;    /* No more processes */
pub const ENOMEM          : ::c_int = 12;    /* Not enough core */
pub const EACCES          : ::c_int = 13;    /* Permission denied */
pub const EDEADLK         : ::c_int = 33;
pub const EINPROGRESS     : ::c_int = 68;
pub const EALREADY        : ::c_int = 69;
pub const EWOULDBLOCK     : ::c_int = 70;
pub const ENOSYS          : ::c_int = 71;
pub const EDESTADDRREQ    : ::c_int = 40;
pub const EPROTOTYPE      : ::c_int = 41;
pub const ENOPROTOOPT     : ::c_int = 42;
pub const EPROTONOSUPPORT : ::c_int = 43;
pub const ESOCKTNOSUPPORT : ::c_int = 44;
pub const EOPNOTSUPP      : ::c_int = 45;
pub const EPFNOSUPPORT    : ::c_int = 46;
pub const EAFNOSUPPORT    : ::c_int = 47;
pub const EADDRINUSE      : ::c_int = 48;
pub const EADDRNOTAVAIL   : ::c_int = 49;
pub const ENOTSOCK        : ::c_int = 50;
pub const ENETUNREACH     : ::c_int = 51;
pub const ENETRESET       : ::c_int = 52;
pub const ECONNABORTED    : ::c_int = 53;
pub const ECONNRESET      : ::c_int = 54;
pub const ENOBUFS         : ::c_int = 55;
pub const EISCONN         : ::c_int = 56;
pub const ENOTCONN        : ::c_int = 57;
pub const ESHUTDOWN       : ::c_int = 58;
pub const ETOOMANYREFS    : ::c_int = 59;
pub const ETIMEDOUT       : ::c_int = 60;
pub const ECONNREFUSED    : ::c_int = 61;

// NFS errnos: Refer to pkgs_v2/storage/fs/nfs/h/nfs/nfsCommon.h


const M_nfsStat : ::c_int = 48 << 16; // This should technically be c_uint, but there is
                                      // no difference between c_int and c_uint when you are
                                      // doing the << operation, (as far as I know) so 
                                      // doesn't look that important.
enum nfsstat {
    NFS_OK             = 0,
    NFSERR_PERM        = 1,
    NFSERR_NOENT       = 2,
    NFSERR_IO          = 5,
    NFSERR_NXIO        = 6,
    NFSERR_ACCESS      = 13,
    NFSERR_EXIST       = 17,
    NFSERR_XDEV        = 18,
    NFSERR_NODEV       = 19,
    NFSERR_NOTDIR      = 20,
    NFSERR_ISDIR       = 21,
    NFSERR_INVAL       = 22,
    NFSERR_FBIG        = 27,
    NFSERR_NOSPC       = 28,
    NFSERR_ROFS        = 30,
    NFSERR_MLINK       = 31,
    NFSERR_NAMETOOLONG = 63,
    NFSERR_NOTEMPTY    = 66,
    NFSERR_DQUOT       = 69,
    NFSERR_STALE       = 70,
    NFSERR_REMOTE      = 71,
    NFSERR_WFLUSH      = 99,
    NFSERR_BADHANDLE   = 10001,
    NFSERR_NOT_SYNC    = 10002,
    NFSERR_BAD_COOKIE  = 10003,
    NFSERR_NOTSUPP     = 10004,
    NFSERR_TOOSMALL    = 10005,
    NFSERR_SERVERFAULT = 10006,
    NFSERR_BADTYPE     = 10007,
    NFSERR_JUKEBOX     = 10008,
}

pub const S_nfsLib_NFS_OK            : ::c_int = M_nfsStat | nfsstat::NFS_OK as ::c_int;
pub const S_nfsLib_NFSERR_PERM       : ::c_int = M_nfsStat | nfsstat::NFSERR_PERM as ::c_int;
pub const S_nfsLib_NFSERR_NOENT      : ::c_int = M_nfsStat | nfsstat::NFSERR_NOENT as ::c_int;
pub const S_nfsLib_NFSERR_IO         : ::c_int = M_nfsStat | nfsstat::NFSERR_IO as ::c_int;
pub const S_nfsLib_NFSERR_NXIO       : ::c_int = M_nfsStat | nfsstat::NFSERR_NXIO as ::c_int;
pub const S_nfsLib_NFSERR_ACCESS     : ::c_int = M_nfsStat | nfsstat::NFSERR_ACCESS as ::c_int;
pub const S_nfsLib_NFSERR_EXIST      : ::c_int = M_nfsStat | nfsstat::NFSERR_EXIST as ::c_int;
pub const S_nfsLib_NFSERR_NODEV      : ::c_int = M_nfsStat | nfsstat::NFSERR_NODEV as ::c_int;
pub const S_nfsLib_NFSERR_NOTDIR     : ::c_int = M_nfsStat | nfsstat::NFSERR_NOTDIR as ::c_int;
pub const S_nfsLib_NFSERR_ISDIR      : ::c_int = M_nfsStat | nfsstat::NFSERR_ISDIR as ::c_int;
pub const S_nfsLib_NFSERR_INVAL      : ::c_int = M_nfsStat | nfsstat::NFSERR_INVAL as ::c_int;
pub const S_nfsLib_NFSERR_FBIG       : ::c_int = M_nfsStat | nfsstat::NFSERR_FBIG as ::c_int;
pub const S_nfsLib_NFSERR_NOSPC      : ::c_int = M_nfsStat | nfsstat::NFSERR_NOSPC as ::c_int;
pub const S_nfsLib_NFSERR_ROFS       : ::c_int = M_nfsStat | nfsstat::NFSERR_ROFS as ::c_int;
pub const S_nfsLib_NFSERR_NAMETOOLONG: ::c_int = M_nfsStat | nfsstat::NFSERR_NAMETOOLONG as ::c_int;
pub const S_nfsLib_NFSERR_NOTEMPTY   : ::c_int = M_nfsStat | nfsstat::NFSERR_NOTEMPTY as ::c_int;
pub const S_nfsLib_NFSERR_DQUOT      : ::c_int = M_nfsStat | nfsstat::NFSERR_DQUOT as ::c_int;
pub const S_nfsLib_NFSERR_STALE      : ::c_int = M_nfsStat | nfsstat::NFSERR_STALE as ::c_int;
pub const S_nfsLib_NFSERR_WFLUSH     : ::c_int = M_nfsStat | nfsstat::NFSERR_WFLUSH as ::c_int;
pub const S_nfsLib_NFSERR_REMOTE     : ::c_int = M_nfsStat | nfsstat::NFSERR_REMOTE as ::c_int;
pub const S_nfsLib_NFSERR_BADHANDLE  : ::c_int = M_nfsStat | nfsstat::NFSERR_BADHANDLE as ::c_int;
pub const S_nfsLib_NFSERR_NOT_SYNC   : ::c_int = M_nfsStat | nfsstat::NFSERR_NOT_SYNC as ::c_int;
pub const S_nfsLib_NFSERR_BAD_COOKIE : ::c_int = M_nfsStat | nfsstat::NFSERR_BAD_COOKIE as ::c_int;
pub const S_nfsLib_NFSERR_NOTSUPP    : ::c_int = M_nfsStat | nfsstat::NFSERR_NOTSUPP as ::c_int;
pub const S_nfsLib_NFSERR_TOOSMALL   : ::c_int = M_nfsStat | nfsstat::NFSERR_TOOSMALL as ::c_int;
pub const S_nfsLib_NFSERR_SERVERFAULT: ::c_int = M_nfsStat | nfsstat::NFSERR_SERVERFAULT as ::c_int;
pub const S_nfsLib_NFSERR_BADTYPE    : ::c_int = M_nfsStat | nfsstat::NFSERR_BADTYPE as ::c_int;
pub const S_nfsLib_NFSERR_JUKEBOX    : ::c_int = M_nfsStat | nfsstat::NFSERR_JUKEBOX as ::c_int;


// IP Stuff? These are allll guesswork
pub const IPPROTO_IP         : ::c_int = 0; 
pub const IP_TTL             : ::c_int = 4; // not sure if this is right
pub const IP_ADD_MEMBERSHIP  : ::c_int = 11;
pub const IP_DROP_MEMBERSHIP : ::c_int = 12;
pub const IPV6_V6ONLY        : ::c_int = 26;
pub const IP_MULTICAST_TTL   : ::c_int = 33;
pub const IP_MULTICAST_LOOP  : ::c_int = 34;
pub const IPV6_MULTICAST_LOOP: ::c_int = 19;
pub const IPPROTO_IPV6       : ::c_int = 41; // or this one, for that matter
pub type in_addr_t = u32;


// STAT Stuff
pub const S_IFMT     : ::c_int = 0xf000; /* file type field */
pub const S_IFIFO    : ::c_int = 0x1000;/*  fifo */
pub const S_IFCHR    : ::c_int = 0x2000;/*  character special */
pub const S_IFDIR    : ::c_int = 0x4000;/*  directory */
pub const S_IFBLK    : ::c_int = 0x6000;/*  block special */
pub const S_IFREG    : ::c_int = 0x8000;/*  regular */
pub const S_IFLNK    : ::c_int = 0xa000;/*  symbolic link */
pub const S_IFSHM    : ::c_int = 0xb000;/*  shared memory object */
pub const S_IFDEVMEM : ::c_int = 0xd000;/*  device memory object */
pub const S_IFSOCK   : ::c_int = 0xc000;/*  socket */
pub const S_ISUID    : ::c_int = 0x0800;/* set user id on execution */
pub const S_ISGID    : ::c_int = 0x0400;/* set group id on execution */
pub const S_ISTXT    : ::c_int = 0x0200;/* sticky bit */
pub const S_IRUSR    : ::c_int = 0x0100;/* read permission, owner */
pub const S_IWUSR    : ::c_int = 0x0080;/* write permission, owner */
pub const S_IXUSR    : ::c_int = 0x0040;/* execute/search permission, owner */
pub const S_IRWXU    : ::c_int = 0x01c0;/* read/write/execute permission, owner */
pub const S_IRGRP    : ::c_int = 0x0020;/* read permission, group */
pub const S_IWGRP    : ::c_int = 0x0010;/* write permission, group */
pub const S_IXGRP    : ::c_int = 0x0008;/* execute/search permission, group */
pub const S_IRWXG    : ::c_int = 0x0038;/* read/write/execute permission, group */
pub const S_IROTH    : ::c_int = 0x0004;/* read permission, other */
pub const S_IWOTH    : ::c_int = 0x0002;/* write permission, other */
pub const S_IXOTH    : ::c_int = 0x0001;/* execute/search permission, other */
pub const S_IRWXO    : ::c_int = 0x0007;/* read/write/execute permission, other */

pub const SOL_SOCKET     : ::c_int = 0xffff; /* options for socket level - more socket stuff below */
pub const SO_BROADCAST   : ::c_int = 0x001e;
pub const SO_SNDTIMEO    : ::c_int = 0x1005;
pub const SO_RCVTIMEO    : ::c_int = 0x1006;
pub const SOCK_STREAM    : ::c_int = 1;
pub const SOCK_DGRAM     : ::c_int = 2;
pub const SOCK_RAW       : ::c_int = 3;
pub const SOCK_RDM       : ::c_int = 4;
pub const SOCK_SEQPACKET : ::c_int = 5;
pub const SOCK_PACKET    : ::c_int = 10;
pub const SO_DEBUG       : ::c_int = 0x0001;
pub const SO_REUSEADDR   : ::c_int = 0x0004;
pub const SO_KEEPALIVE   : ::c_int = 0x0008;
pub const SO_DONTROUTE   : ::c_int = 0x0010;
pub const SO_RCVLOWAT    : ::c_int = 0x0012;

pub const _SS_MAXSIZE    : usize = 128;
pub const _SS_ALIGNSIZE  : usize = size_of::<::uint32_t>();
pub const _SS_PAD1SIZE   : usize = (_SS_ALIGNSIZE - size_of::<::c_uchar>()
                                     - size_of::<::sa_family_t>());
pub const _SS_PAD2SIZE   : usize = (_SS_MAXSIZE - size_of::<::c_uchar>() - size_of::<::sa_family_t>()
                                     -_SS_PAD1SIZE - _SS_ALIGNSIZE);


pub const MSG_OOB          : ::c_int = 0x0001;
pub const MSG_PEEK         : ::c_int = 0x0002;
pub const MSG_DONTROUTE    : ::c_int = 0x0004;
pub const MSG_EOR          : ::c_int = 0x0008;
pub const MSG_TRUNC        : ::c_int = 0x0010;
pub const MSG_CTRUNC       : ::c_int = 0x0020;
pub const MSG_WAITALL      : ::c_int = 0x0040;
pub const MSG_DONTWAIT     : ::c_int = 0x0080;
pub const MSG_EOF          : ::c_int = 0x0100;
pub const MSG_EXP          : ::c_int = 0x0200;
pub const MSG_MBUF         : ::c_int = 0x0400;
pub const MSG_NOTIFICATION : ::c_int = 0x0800;
pub const MSG_COMPAT       : ::c_int = 0x8000;

pub const AF_UNSPEC    : ::c_int = 0;
pub const AF_LOCAL     : ::c_int = 1;
pub const AF_UNIX      : ::c_int = AF_LOCAL;
pub const AF_INET      : ::c_int = 2;
pub const AF_NETLINK   : ::c_int = 16;
pub const AF_ROUTE     : ::c_int = 17;
pub const AF_LINK      : ::c_int = 18;
pub const AF_PACKET    : ::c_int = 19;
pub const pseudo_AF_KEY: ::c_int = 27;
pub const AF_KEY       : ::c_int = pseudo_AF_KEY; 
pub const AF_INET6     : ::c_int = 28;
pub const AF_SOCKDEV   : ::c_int = 31;
pub const AF_TIPC      : ::c_int = 33;
pub const AF_MIPC      : ::c_int = 34;
pub const AF_MIPC_SAFE : ::c_int = 35;
pub const AF_MAX       : ::c_int = 36;


pub const SHUT_RD      : ::c_int = 0; /* shut down the reading side */
pub const SHUT_WR      : ::c_int = 1; /* shut down the writing side */
pub const SHUT_RDWR    : ::c_int = 2; /* shut down both sides */

pub const IPPROTO_TCP  : ::c_int = 6;
pub const TCP_NODELAY  : ::c_int = 1;  /* don't delay send to coalesce packets */
pub const TCP_MAXSEG   : ::c_int = 2;  /* set maximum segment size */
pub const TCP_NOPUSH   : ::c_int = 3;  /* don't push last block of write */
pub const TCP_KEEPIDLE : ::c_int = 4;  /* Send first keepalive probe when the connections
                                          been isdl this time (in seconds) */
pub const TCP_KEEPINTVL: ::c_int = 5;  /* Interval (in seconds) between keepalives */
pub const TCP_KEEPCNT  : ::c_int = 6;  /* Maximum number of keepalives before dropping
                                          the connection */
pub const SO_ERROR     : ::c_int = 4;  /* It's either 4, 0x1007, or doesn't exist, idk which */

//defined for the structs
pub type dev_t    = ::c_ulong;
pub type mode_t   = ::c_int;
pub type nlink_t  = ::c_ulong;
pub type uid_t    = ::c_ushort; //from glibc ... I can't seem to find this elsewhere
pub type gid_t    = ::c_ushort;
pub type sigset_t = ::c_ulonglong;
pub type key_t    = ::c_long;
pub type shmatt_t = ::c_ulong; /* Might not be stable .. do we even support shared memory? */

#[cfg(feature = "_WRS_KERNEL")]
pub type mqd_t = ::size_t; //type is struct mq_des * in kernel definition
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type mqd_t = ::c_int;

pub type nfds_t  = ::c_uint;
pub type nl_item = ::c_int; /* Can't find in VxWorks - from https://doc.rust-lang.org/1.10.0/libc/type.nl_item.html */
pub type stat64  = ::stat;

pub type pthread_key_t = ::c_ulong;

// From b_off_t.h
pub type off64_t = ::c_longlong;
pub type off_t64 = ::c_longlong;

// From b_BOOL.h
pub type BOOL      = ::c_int; // excuse me what

//Straight from vxWind.h ..

// definitions in krnl directory
#[cfg(feature = "_WRS_KERNEL")]
pub type OBJ_HANDLE = ::c_int;
#[cfg(feature = "_WRS_KERNEL")]
pub type CLASS_ID  = *mut ::wind_class;
#[cfg(feature = "_WRS_KERNEL")]
pub type RTP_ID    = *mut ::wind_rtp;
#[cfg(feature = "_WRS_KERNEL")]
pub type TASK_ID   = ::_Vx_TASK_ID;
#[cfg(feature = "_WRS_KERNEL")]
pub type SEM_ID    = *mut ::semaphore;
#[cfg(feature = "_WRS_KERNEL")]
pub type OBJ_ID    = *mut ::c_void;

// definitions in usr directory
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type _Vx_OBJ_HANDLE    = ::c_int;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type _Vx_TASK_ID       = ::_Vx_OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type _Vx_MSG_Q_ID      = ::_Vx_OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type _Vx_SEM_ID_KERNEL = ::_Vx_OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type _Vx_RTP_ID        = ::_Vx_OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type _Vx_SD_ID         = ::_Vx_OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type _Vx_CONDVAR_ID    = ::_Vx_OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type _Vx_SEM_ID        = *mut ::_Vx_semaphore;

#[cfg(not(feature = "_WRS_KERNEL"))]
pub type OBJ_HANDLE        = ::_Vx_OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type TASK_ID           = ::OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type MSG_Q_ID          = ::OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type SEM_ID_KERNEL     = ::OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type RTP_ID            = ::OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type SD_ID             = ::OBJ_HANDLE;
#[cfg(not(feature = "_WRS_KERNEL"))]
pub type CONDVAR_ID        = ::OBJ_HANDLE;

// From vxTypes.h
pub type _Vx_usr_arg_t   = ::ssize_t;   // c_int for LP32
pub type _Vx_exit_code_t = ::ssize_t;   // c_int for LP32
pub type _Vx_ticks_t     = ::c_uint;
pub type _Vx_ticks64_t   = ::c_ulonglong;

// From vxTypesBase.h
pub type va_list = *mut ::c_char;


// IO Lib Definitions:

pub const FIONREAD        : ::c_int = 1;
pub const FIOFLUSH        : ::c_int = 2;
pub const FIOOPTIONS      : ::c_int = 3; 
pub const FIOBAUDRATE     : ::c_int = 4;
pub const FIODISKFORMAT   : ::c_int = 5;
pub const FIODISKINIT     : ::c_int = 6;
pub const FIOSEEK         : ::c_int = 7;
pub const FIOWHERE        : ::c_int = 8;
pub const FIODIRENTRY     : ::c_int = 9;
pub const FIORENAME       : ::c_int = 10;
pub const FIOREADYCHANGE  : ::c_int = 11;
pub const FIOWRITE        : ::c_int = 12;
pub const FIODISKCHANGE   : ::c_int = 13;
pub const FIOCANCEL       : ::c_int = 14;
pub const FIOSQUEEZE      : ::c_int = 15;
pub const FIONBIO         : ::c_int = -1878786032; // it goes on ...
// ((int) ( 0x80000000 | (4 & 0xfff) << 16) | 0x10000000 | (0 << 8) | (16 & 0xff)))
// You can also try and print out the value after including ioLib.h in an rtp.
pub const _POSIX_PATH_MAX : ::c_int = 256;


// Some poll stuff
pub const POLLIN    : ::c_short = 0x0001;
pub const POLLPRI   : ::c_short = 0x0002;
pub const POLLOUT   : ::c_short = 0x0004;
pub const POLLRDNORM: ::c_short = 0x0040;
pub const POLLWRNORM: ::c_short = POLLOUT;
pub const POLLRDBAND: ::c_short = 0x0080;
pub const POLLWRBAND: ::c_short = 0x0100;
pub const POLLER    : ::c_short = 0x0008;
pub const POLLHUP   : ::c_short = 0x0010;
pub const POLLNVAL  : ::c_short = 0x0020;

//Some Fcntlcom Stuff (look at fcntlcom.h to find definitions)
pub const FD_CLOEXEC      : ::c_int = 1;
pub const F_DUPFD         : ::c_int = 0;
pub const F_GETFD         : ::c_int = 1;
pub const F_SETFD         : ::c_int = 2;
pub const F_GETFL         : ::c_int = 3;
pub const F_SETFL         : ::c_int = 4;
pub const F_GETOWN        : ::c_int = 5;
pub const F_SETOWN        : ::c_int = 6;
pub const F_GETLK         : ::c_int = 7;
pub const F_SETLK         : ::c_int = 8;
pub const F_SETLKW        : ::c_int = 9;
pub const F_DUPFD_CLOEXEC : ::c_int = 14;

//Some Dirent.h stuff
pub const DT_UNKNOWN : ::c_uchar =  0x0;
pub const DT_FIFO    : ::c_uchar = 0x1;
pub const DT_CHR     : ::c_uchar = 0x2;
pub const DT_DIR     : ::c_uchar = 0x4;
pub const DT_BLK     : ::c_uchar = 0x6;
pub const DT_REG     : ::c_uchar = 0x8;
pub const DT_LNK     : ::c_uchar = 0xA;
pub const DT_SOCK    : ::c_uchar = 0xC;
pub const DT_WHT     : ::c_uchar = 0xE;

// Other Random Stuff
pub const VXSIM_EWOULDBLOCK   : ::c_int = 70;
pub const IPV6_ADD_MEMBERSHIP : ::c_int = 20; //Might not be supported
pub const IPV6_DROP_MEMBERSHIP: ::c_int = 21; //Also might not be supported

pub type sa_family_t = ::c_uchar; //UINT16 for ipcom_sock.h, but uchar in socket.h

// Signal stuff?
pub const SIG_ERR : ::c_int = -1;    
pub const SIG_DEL : ::c_int = 0;    
pub const SIG_IGN : ::c_int = 1;    
pub const SIGHUP  : ::c_int = 1;     /* hangup */
pub const SIGINT  : ::c_int = 2;     /* interrupt */
pub const SIGQUIT : ::c_int = 3;     /* quit */
pub const SIGILL  : ::c_int = 4;     /* illegal instruction (not reset when caught) */
pub const SIGTRAP : ::c_int = 5;     /* trace trap (not reset when caught) */
pub const SIGABRT : ::c_int = 6;     /* used by abort, replace SIGIOT in the future */
pub const SIGEMT  : ::c_int = 7;     /* EMT instruction */
pub const SIGFPE  : ::c_int = 8;     /* floating point exception */
pub const SIGKILL : ::c_int = 9;     /* kill */    
pub const SIGBUS  : ::c_int = 10;    /* bus error */
pub const SIGSEGV : ::c_int = 11;    /* segmentation violation */
pub const SIGFMT  : ::c_int = 12;    /* STACK FORMAT ERROR (not posix) */
pub const SIGPIPE : ::c_int = 13;    /* write on a pipe with no one to read it */
pub const SIGALRM : ::c_int = 14;    /* alarm clock */    
pub const SIGTERM : ::c_int = 15;    /* software termination signal from kill */
pub const SIGCNCL : ::c_int = 16;    /* pthreads cancellation signal */
pub const SIGSTOP : ::c_int = 17;    /* sendable stop signal not from tty */
pub const SIGTSTP : ::c_int = 18;    /* stop signal from tty */
pub const SIGCONT : ::c_int = 19;    /* continue a stopped process */
pub const SIGCHLD : ::c_int = 20;    /* to parent on child stop or exit */
pub const SIGTTIN : ::c_int = 21;    /* to readers pgrp upon background tty read */
pub const SIGTTOU : ::c_int = 22;    /* like TTIN for output if (tp->t_local&LTOSTOP) */

pub const SIG_BLOCK  : ::c_int =  1;
pub const SIG_UNBLOCK: ::c_int =  2;
pub const SIG_SETMASK: ::c_int =  3;

pub const SI_SYNC    : ::c_int =  0;    /* (Not posix) gernerated by hardware */
pub const SI_USER    : ::c_int = -1;    /* signal from kill() function */
pub const SI_QUEUE   : ::c_int = -2;    /* signal from sigqueue() function */
pub const SI_TIMER   : ::c_int = -3;    /* signal from expiration of a timer */
pub const SI_ASYNCIO : ::c_int = -4;    /* signal from completion of async I/O */
pub const SI_MESGQ   : ::c_int = -5;    /* signal from arrival of a message */
pub const SI_CHILD   : ::c_int = -6;    /* signal from child, stopped or terminated */
pub const SI_KILL    : ::c_int = SI_USER;    /* signal from kill() function */
pub const SIG_DFL    : sighandler_t = 0 as sighandler_t;

// vxParams.h definitions
#[cfg(feature = "_WRS_CONFIG_DOSFS_NAME_LENGTH_COMPAT")]
pub const _PARM_NAME_MAX : usize = 1020;
#[cfg(feature = "_WRS_CONFIG_DOSFS_NAME_LENGTH_COMPAT")]
pub const _PARM_PATH_MAX : usize = 1080;
#[cfg(not(feature = "_WRS_CONFIG_DOSFS_NAME_LENGTH_COMPAT"))]
pub const _PARM_NAME_MAX : usize = 255;
#[cfg(not(feature = "_WRS_CONFIG_DOSFS_NAME_LENGTH_COMPAT"))]
pub const _PARM_PATH_MAX : usize = 1024;

// vxWindCommon.h
#[cfg(feature = "_WRS_KERNEL")]
pub enum windObjClassType
    {
    windInvalidClass	= 0,    /* invalid class type class		*/
    windSemClass,		        /* Wind native semaphore		*/
    windSemPxClass,		        /* POSIX semaphore			*/
    windMsgQClass,		        /* Wind native message queue		*/
    windMqPxClass,		        /* POSIX message queue			*/
    windRtpClass,		        /* real time process			*/
    windTaskClass,		        /* task					*/
    windWdClass,		        /* watchdog				*/
    windFdClass,		        /* file descriptor			*/
    windPgPoolClass,		    /* page pool				*/
    windPgMgrClass,		        /* page manager				*/
    windGrpClass,		        /* group				*/
    windVmContextClass,		    /* virtual memory context		*/
    windTrgClass,		        /* trigger				*/
    windMemPartClass,		    /* memory partition			*/
    windI2oClass,		        /* I2O					*/
    windDmsClass,		        /* device management system		*/
    windSetClass,		        /* Set                                  */
    windIsrClass,               /* ISR object                           */
    windTimerClass,             /* Timer services                       */
    windSdClass,                /* Shared data region 			*/
    windPxTraceClass,           /* POSIX trace                          */
    windCondVarClass,		    /* Condition Variables			*/
    windApexSamplingPortClass,  /* APEX Sampling Port                   */
    windApexQueuingPortClass,   /* APEX Queuing Port                    */
    windApexProcessClass,       /* APEX process                         */
    windApexBufferClass,        /* APEX buffer                          */
    windApexSemaphoreClass,     /* APEX semaphore                       */
    windApexBlackboardClass,    /* APEX blackboard                      */
    windApexEventClass,         /* APEX event                           */

   /* see comments in posix section on windNumObjClass */
    windNumObjClass
}



// WAIT STUFF
pub const WNOHANG   : ::c_int = 0x01;
pub const WUNTRACED : ::c_int = 0x02;


const PTHREAD_MUTEXATTR_INITIALIZER: pthread_mutexattr_t = pthread_mutexattr_t {
    mutexAttrStatus: PTHREAD_INITIALIZED_OBJ,
    mutexAttrProtocol: PTHREAD_PRIO_NONE,
    mutexAttrPrioceiling: 0,
    mutexAttrType: PTHREAD_MUTEX_DEFAULT,
};
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    mutexSemId: null_mut(),
    mutexValid: PTHREAD_VALID_OBJ,
    mutexInitted: PTHREAD_UNUSED_YET_OBJ,
    mutexCondRefCount: 0,
    mutexSavPriority: 0,
    mutexAttr: PTHREAD_MUTEXATTR_INITIALIZER,
};

const PTHREAD_CONDATTR_INITIALIZER: pthread_condattr_t = pthread_condattr_t {
    condAttrStatus: 0,
    _CondAttrClockId: CLOCK_REALTIME,
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    condSemId: null_mut(),
    condValid: PTHREAD_VALID_OBJ,
    condInitted: PTHREAD_UNUSED_YET_OBJ,
    condRefCount: 0,
    condMutex: null_mut(),
    condAttr: PTHREAD_CONDATTR_INITIALIZER,
};

const PTHREAD_RWLOCKATTR_INITIALIZER: pthread_rwlockattr_t = pthread_rwlockattr_t {
    rwlockAttrStatus: PTHREAD_INITIALIZED_OBJ,
    rwlockAttrMaxReaders: 0,
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    rwlockSemId: null_mut(),
    rwlockReadersRefCount: 0,
    rwlockValid: PTHREAD_VALID_OBJ,
    rwlockInitted: PTHREAD_UNUSED_YET_OBJ,
    rwlockAttr: PTHREAD_RWLOCKATTR_INITIALIZER,
};

pub const SEEK_SET: ::c_int = 0; 
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;

// rtpLibCommon.h
pub const VX_RTP_NAME_LENGTH : usize = 255;

//Some unsupported stuff
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = -1; // Via unistd.h
pub const _SC_PAGESIZE        : ::c_int = 64; // getpagesize(); // Via getpagesize.c - this might actually be supported? The _sc_pagesize depends on the underlying architecture - this is hacked because I can't get it to work
pub const O_ACCMODE           : ::c_int = 3; // from docs - the actual definition is a bunch of ors
pub const O_CLOEXEC           : ::c_int = 0x100000; // fcntlcom
pub const O_EXCL              : ::c_int = 0x0800;
pub const O_CREAT             : ::c_int = 0x0200;
pub const O_TRUNC             : ::c_int = 0x0400;
pub const O_APPEND            : ::c_int = 0x0008;
pub const O_RDWR              : ::c_int = 2;
pub const O_WRONLY            : ::c_int = 1;
pub const O_RDONLY            : ::c_int = 0;


// taskLib.h
// Exists in both krnl and usr directories, but not mentioned in
// usr functions in this liblibc.
// It's only used as a pointer, so only declared.
#[cfg(feature = "_WRS_KERNEL")]
pub struct windTcb;

// classLibP.h
// It's only used as a pointer, so only declared.
#[cfg(feature = "_WRS_KERNEL")]
pub struct wind_class;

// rtpLibP.h
// It's only used as a pointer, so only declared.
#[cfg(feature = "_WRS_KERNEL")]
pub struct wind_rtp;

// qLibP.h
// It's only used as a pointer, so only declared.
#[cfg(feature = "_WRS_KERNEL")]
pub struct Q_NODE;

// qLibP.h
// It's only used as a pointer, so only declared.
#[cfg(feature = "_WRS_KERNEL")]
pub struct q_class;


// dllLib.h
// This struct exists for both kernel and user, but it doesn't get used 
// for user.
// It's only used as a pointer, so only declared.
#[cfg(feature = "_WRS_KERNEL")]
pub struct DL_NODE;

// structs only used by kernel functions (that are ported)
#[cfg(feature = "_WRS_KERNEL")]
s! {
    // eventLibP.h
    pub struct EVENTS_RSRC {
        pub taskId     : ::TASK_ID,
        pub registered : ::c_uint,
        pub options    : ::c_uchar,
        pub pad        : [::c_uchar; 3],
    }


    // qLibP.h
    pub struct Q_HEAD {
        pub pFirstNode : *mut ::Q_NODE,
        pub qPriv1     : c_ulong,
        pub qPriv2     : c_ulong,
        pub pQClass    : *mut ::q_class,
    }

    // handleLibP.h
    // This struct exists for both kernel and user, but it doesn't get used 
    // for user.
    pub struct HANDLE {
        pub magic       : ::c_ulong,
        pub safeCnt     : u32,
        pub attributes  : u16,
        pub _type        : i8,
        pub contextType : u8,
        pub context     : *mut ::c_void,
    }

    // dllLib.h
    // This struct exists for both kernel and user, but it doesn't get used.
    // for user.
    pub struct DL_LIST { // DL_LIST is actually _Vx_DL_LIST
        pub head : *mut ::DL_NODE,
        pub tail : *mut ::DL_NODE,
    }

    // pthread.h
    pub fn pthread_cond_init (
        cond: *mut ::pthread_cond_t,
        attr: *mut ::pthread_condattr_t,
    ) -> ::c_int;
}

// structs that only exist in userspace
#[cfg(not(feature = "_WRS_KERNEL"))]
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
    }

        
    // b_struct_timeval.h
    pub struct timeval {
        pub tv_sec: ::time_t,
        pub tv_usec: ::suseconds_t,
    }
}

s! {
    
    // socket.h
    pub struct sockaddr {
        pub sa_len    : ::c_uchar,
        pub sa_family : sa_family_t,
        pub sa_data   : [::c_char; 14],
    }

    // socket.h
    pub struct sockaddr_storage { // Worth noting Sizeof is probably in bytres
        pub ss_len     : ::c_uchar,
        pub ss_family  : ::sa_family_t,
        pub __ss_pad1  : [::c_char; _SS_PAD1SIZE], //Size of UInt32_t - Size of U_char - Size of SA_Family(t), which is a Uchar?
        pub __ss_align : ::int32_t,
        pub __ss_pad2  : [::c_char; _SS_PAD2SIZE], // 128 - Sizeof Char (1) - Sizeof SA_Family_t (1) - ss_pad_1 (2?) -Size of Uint32 (4?)
    }

    // poll.h
    pub struct pollfd {
        pub fd      : ::c_int,
        pub events  : ::c_short,
        pub revents : ::c_short,
    }

    // dirent.h
    pub struct dirent {
    // It seems like in the newer version of vxworks, there is no dirent64
    // In pkgs_v2, there is no symbol for dirent64 in vx7-SR0600
        pub d_ino  : ::ino_t,
        pub d_name : [::c_char; (_PARM_NAME_MAX + 1)],   /* NAME_MAX = 255, length of d_name is 256 */
    }

    pub struct dirent64 {
        pub d_ino    : ::ino_t,
        pub d_off    : ::off64_t,
        pub d_reclen : u16,
        pub d_type   : u8,
        pub d_name   : [::c_char; 256],
    } // Doesn't seem like it exists anymore
    
    // resource.h
    pub struct rlimit { /* Is this really needed? Questionable ... */
        pub rlim_cur : ::size_t,
        pub rlim_max : ::size_t,
    }
    
    // stat.h
    pub struct stat { /* we might also need a stat 64 */
        pub st_dev       : ::dev_t,         /* Device ID number */
        pub st_ino       : ::ino_t,         /* File serial number */
        pub st_mode      : ::mode_t,        /* Mode of file */
        pub st_nlink     : ::nlink_t,      /* Number of hard links to file */
        pub st_uid       : ::uid_t,         /* User ID of file */
        pub st_gid       : ::gid_t,         /* Group ID of file */
        pub st_rdev      : ::dev_t,        /* Device ID if special file */
        pub st_size      : ::off_t,        /* File size in bytes */
        pub st_atime     : ::time_t,       /* Time of last access */
        pub st_mtime     : ::time_t,       /* Time of last modification */
        pub st_ctime     : ::time_t,       /* Time of last status change */
        pub st_blksize   : ::blksize_t, /* File system block size */
        pub st_blocks    : ::blkcnt_t,      /* Number of blocks containing file */
        pub st_attrib    : ::c_uchar,      /* DOSFS only - file attributes */
        pub st_reserved1 : ::c_int,      /* reserved for future use */
        pub st_reserved2 : ::c_int,      /* reserved for future use */
        pub st_reserved3 : ::c_int,      /* reserved for future use */
        pub st_reserved4 : ::c_int,      /* reserved for future use */
    }

    //b_struct__Timespec.h
    pub struct _Timespec {
        pub tv_sec  : ::time_t,
        pub tv_nsec : ::c_long,
    }
    
    // b_struct__Sched_param.h
    pub struct _Sched_param {
        pub sched_priority: ::c_int, /* scheduling priority */
        
        #[cfg(not(feature = "_WRS_KERNEL"))]
        pub sched_ss_low_priority: ::c_int,    /* low scheduling priority */
        #[cfg(not(feature = "_WRS_KERNEL"))]
        pub sched_ss_repl_period: ::_Timespec, /* replenishment period */
        #[cfg(not(feature = "_WRS_KERNEL"))]
        pub sched_ss_init_budget: ::_Timespec, /* initial budget */
        #[cfg(not(feature = "_WRS_KERNEL"))]
        pub sched_ss_max_repl: ::c_int,        /* max pending replenishment */

    }
    
    // b_pthread_attr_t.h
    pub struct pthread_attr_t {
        pub threadAttrStatus          : ::c_int,        /* status flag              */
        pub threadAttrStacksize       : ::size_t,       /* stack size               */
        pub threadAttrStackaddr       : *mut ::c_void,  /* stack address            */
        pub threadAttrGuardsize       : ::size_t,       /* guard address (RTP only) */
        pub threadAttrDetachstate     : ::c_int,        /* detach state             */
        pub threadAttrContentionscope : ::c_int,        /* contention scope         */
        pub threadAttrInheritsched    : ::c_int,        /* inherit scheduler        */
        pub threadAttrSchedpolicy     : ::c_int,        /* scheduling policy        */
        pub threadAttrName            : *mut ::c_char,  /* task name - VxWorks extension    */
        pub threadAttrOptions         : ::c_int,        /* task options - VxWorks extension  */
        pub threadAttrSchedparam      : ::_Sched_param, /* sched param struct */
    }

    // signal.h
    pub struct sigaction { // pulled from kernel side, 
        pub sa_u     : ::size_t, // Other libc implementation treats this as size_t as well
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
    pub struct siginfo_t { // Aliased as _SigInfo? - ask Brian from signal.h user???
        pub si_signo : ::c_int,
        pub si_code  : ::c_int,
        // This field is a union of int and void * in vxworks
        // The size has been set to the larger of the two
        pub si_value : ::size_t,
    }
    
    pub struct ipc_perm { /* See note on shmid_ds below ... we might not need this at all */
        pub __key : key_t,
        pub uid   : uid_t,
        pub gid   : gid_t,
        pub cuid  : uid_t,
        pub cgid  : gid_t,
        pub mode  : ::c_ushort,
        pub __seq : ::c_ushort,
    }
    
    pub struct shmid_ds { // kernel stuff? I literally can't find it in the source ...
        /* 
            This is all copied from a POSIX documentation - https://www.tldp.org/LDP/lpg/node68.html
            This probably doesn't work but we'll keep it like this for now. It seems to be a unix and not a POSIX thing/.
            
        */
        pub shm_perm  : ipc_perm,
        pub shm_segsz : ::c_int,
    }

    

    // pthread.h (krnl)
    // b_pthread_mutexattr_t.h (usr)
    pub struct pthread_mutexattr_t {
        mutexAttrStatus      : ::c_int,

        #[cfg(all(feature = "_POSIX_THREAD_PROCESS_SHARED", feature = "_WRS_KERNEL"))]
        mutexAttrPshared     : ::c_int,

        mutexAttrProtocol    : ::c_int,
        mutexAttrPrioceiling : ::c_int,
        mutexAttrType        : ::c_int,
    }

    // pthread.h (krnl)
    // b_pthread_mutex_t.h (usr)
    pub struct pthread_mutex_t  {

        #[cfg(feature = "_WRS_KERNEL")]
        pub mutexSemId: ::SEM_ID, /*_Vx_SEM_ID ..*/
        #[cfg(not(feature = "_WRS_KERNEL"))]
        pub mutexSemId: ::_Vx_SEM_ID, /*_Vx_SEM_ID ..*/ 

        pub mutexValid: ::c_int,
        pub mutexInitted: ::c_int,
        pub mutexCondRefCount: ::c_int,
        pub mutexSavPriority: ::c_int,
        pub mutexAttr: ::pthread_mutexattr_t,
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

    pub struct passwd { // Doesn't seem to exist, so we are copying the POSIX standard
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
        pub pathName  : [::c_char; (VX_RTP_NAME_LENGTH + 1)],
        pub taskCnt   : u32,
        pub textStart : *mut ::c_void,
        pub textEnd   : *mut ::c_void,
    }
    
}

extern { // this is gonna be a big one

    // stdlib.h
    // This function may not be defined for armv7
    pub fn memalign (
        block_size: ::size_t,
        size_arg: ::size_t,    
    ) -> *mut ::c_void;

    // ioLib.h
    pub fn getcwd (
        buf: *mut ::c_char,
        size: ::size_t,
    ) -> *mut ::c_char;

    // ioLib.h
    pub fn chdir (
        attr: *const ::c_char,
    ) -> ::c_int;
    
    // pthread.h
    pub fn pthread_mutexattr_init ( /* PTHREAD STUFF */
        attr: *mut pthread_mutexattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutexattr_destroy(
        attr: *mut pthread_mutexattr_t
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutexattr_settype (
        pAttr: *mut ::pthread_mutexattr_t,
        pType: ::c_int,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutex_init (
        mutex: *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutex_destroy (
        mutex: *mut pthread_mutex_t
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutex_lock(
        mutex: *mut pthread_mutex_t
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_mutex_trylock (
        mutex: *mut pthread_mutex_t
    ) -> ::c_int;

    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_mutex_timedlock ( 
        attr: *mut pthread_mutex_t,
        spec: *const timespec,
    ) -> ::c_int;
    
    // pthread.h
    pub fn pthread_mutex_unlock (
        mutex: *mut pthread_mutex_t
    ) -> ::c_int;
    
    // pthread.h
    pub fn pthread_attr_setname (
        pAttr: *mut ::pthread_attr_t,
        name : *mut ::c_char,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_attr_setstacksize (
        attr     : *mut ::pthread_attr_t,
        stacksize: ::size_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_attr_getstacksize (
        attr: *const ::pthread_attr_t,
        size: *mut ::size_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_attr_init(
        attr: *mut ::pthread_attr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_create (
        pThread: *mut ::pthread_t,
        pAttr: *const ::pthread_attr_t,
        start_routine : extern fn(*mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_attr_destroy (
        thread: *mut ::pthread_attr_t,
    ) -> ::c_int;
    
    // pthread.h
    pub fn pthread_detach (
        thread: ::pthread_t,
    ) -> ::c_int;
    
    // stat.h
    pub fn fstat(
        fildes: ::c_int, 
        buf: *mut stat,
    ) -> ::c_int;
        
    // stat.h
    pub fn lstat(
        path: *const ::c_char,
        buf: *mut stat,
    ) -> ::c_int;    

    // unistd.h
    pub fn ftruncate(
        fd: ::c_int,
        length: off_t
    ) -> ::c_int;    

    // dirent.h
    pub fn readdir_r( 
        pDir  :      *mut ::DIR,           /* pointer to directory descriptor */
        entry :      *mut ::dirent,        /* pointer to directory entry */
        result: *mut *mut ::dirent,        /* pointer to directory result of read */
    ) -> ::c_int;

    // dirent.h
    pub fn readdir(
        pDir: *mut ::DIR
    ) -> *mut ::dirent;

    // fcntl.h or
    // ioLib.h
    pub fn open ( // this might be hacked
        path: *const ::c_char,
        oflag: ::c_int,
        ...
    ) -> ::c_int;

    // poll.h
    pub fn poll(
        fds: *mut pollfd, // this is suppose to be an array, but doesn't seem to matter
                          // whether or not it is, so just keep this.
        nfds: nfds_t, 
        timeout: ::c_int
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_condattr_init(
        attr: *mut ::pthread_condattr_t
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_condattr_destroy(
        attr: *mut ::pthread_condattr_t
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_condattr_getclock (
        pAttr: *const ::pthread_condattr_t,
        pClockId: *mut ::clockid_t
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_condattr_setclock (
        pAttr: *mut ::pthread_condattr_t,
        clockId: ::clockid_t
    ) -> ::c_int;
    
    

    // pthread.h

    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_cond_init (
        cond: *mut ::pthread_cond_t,
        attr: *const ::pthread_condattr_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_cond_destroy (
        cond: *mut pthread_cond_t,
    ) -> ::c_int;
    
    // pthread.h
    pub fn pthread_cond_signal (
        cond: *mut ::pthread_cond_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_cond_broadcast (
        cond: *mut ::pthread_cond_t,
    ) -> ::c_int;
    
    // pthread.h
    pub fn pthread_cond_wait (
        cond: *mut ::pthread_cond_t,
        mutex: *mut ::pthread_mutex_t,
    ) -> ::c_int;

    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlockattr_init (
        attr: *mut ::pthread_rwlockattr_t,
    ) -> ::c_int;
    
    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlockattr_destroy (
        attr: *mut ::pthread_rwlockattr_t,
    ) -> ::c_int;

    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlockattr_setmaxreaders (
        attr: *mut ::pthread_rwlockattr_t,
        attr2: ::c_uint,
    ) -> ::c_int;

    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlock_init (
        attr: *mut ::pthread_rwlock_t,
        host: *const ::pthread_rwlockattr_t
    ) -> ::c_int;

    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlock_destroy (
        attr: *mut ::pthread_rwlock_t,
    ) -> ::c_int;
    
    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlock_rdlock (
        attr: *mut ::pthread_rwlock_t,
    ) -> ::c_int;

    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlock_tryrdlock (
        attr: *mut ::pthread_rwlock_t,
    ) -> ::c_int;
    
    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlock_timedrdlock (
        attr: *mut ::pthread_rwlock_t,
        host: *const ::timespec,
    ) -> ::c_int;

    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlock_wrlock (
        attr: *mut ::pthread_rwlock_t,
    ) -> ::c_int;

    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlock_trywrlock (
        attr: *mut ::pthread_rwlock_t,
    ) -> ::c_int;

    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlock_timedwrlock (
        attr: *mut ::pthread_rwlock_t,
        host: *const ::timespec,
    ) -> ::c_int;

    // pthread.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn pthread_rwlock_unlock (
        attr: *mut ::pthread_rwlock_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_key_create (
        key: *mut ::pthread_key_t,
        dtor: Option<unsafe extern fn(*mut ::c_void)>
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_key_delete (
        key: ::pthread_key_t,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_setspecific (
        key: ::pthread_key_t,
        value: *const ::c_void,
    ) -> ::c_int;

    // pthread.h
    pub fn pthread_getspecific(
        key: ::pthread_key_t,
    ) -> *mut ::c_void;

    // pthread.h
    pub fn pthread_cond_timedwait (
        cond: *mut ::pthread_cond_t,
        mutex: *mut ::pthread_mutex_t,
        abstime: *const ::timespec
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
    pub fn pthread_self(
    ) -> ::pthread_t;

    // clockLib.h
    pub fn clock_gettime (
        clock_id: ::clockid_t,
        tp: *mut ::timespec,    
    ) -> ::c_int;
    
    // clockLib.h
    pub fn clock_settime (
        clock_id: ::clockid_t,    
        tp: *const ::timespec
    ) -> ::c_int;

    // clockLib.h
    #[cfg(feature = "_WRS_KERNEL")]
    pub fn clock_adjtime (
        clock_id: ::clockid_t,
        delta: *const ::timespec,
        olddelta: *mut ::timespec
    ) -> ::c_int;

    // clockLib.h
    pub fn clock_getres (
        clock_id: ::clockid_t,
        res: *mut ::timespec,
    ) -> ::c_int;

    // clockLib.h
    pub fn clock_nanosleep (
        clock_id: ::clockid_t,
        flags: ::c_int,
        rqtp: *const ::timespec,
        rmtp: *mut ::timespec
    ) -> ::c_int;    
    
    // timerLib.h
    pub fn nanosleep (
        rqtp: *const ::timespec,
        rmtp: *mut ::timespec
    ) -> ::c_int;

    // socket.h
    pub fn accept (
        s: ::c_int,
        addr: *mut ::sockaddr,
        addrlen: *mut ::socklen_t,
    ) -> ::c_int;

    // socket.h
    pub fn bind(
        fd: ::c_int, 
        addr: *const sockaddr, 
        len: socklen_t
    ) -> ::c_int;

    // socket.h
    pub fn connect (
        s: ::c_int,
        name: *const ::sockaddr,
        namelen: ::socklen_t,
    ) -> ::c_int;

    // socket.h
    pub fn getpeername (
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
    pub fn listen(
        socket: ::c_int, 
        backlog: ::c_int,
    ) -> ::c_int;

    // socket.h
    pub fn recv (
        s: ::c_int,
        buf: *mut ::c_void,
        bufLen: ::size_t,
        flags: ::c_int,
    ) -> ::ssize_t;

    // socket.h
    pub fn recvfrom (
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
        addrlen: socklen_t
    ) -> ::ssize_t;
    
    // socket.h
    pub fn setsockopt(
        socket: ::c_int, 
        level: ::c_int, 
        name: ::c_int,
        value: *const ::c_void,
        option_len: socklen_t
    ) -> ::c_int;

    // socket.h
    pub fn shutdown (
        s: ::c_int,
        how: ::c_int,
    ) -> ::c_int; 

    // socket.h
    pub fn socket (
        domain: ::c_int,
        _type: ::c_int,
        protocol: ::c_int
    ) -> ::c_int;
    
    pub fn socketpair( // Doesn't exist
        domain: ::c_int,
        type_: ::c_int,
        protocol: ::c_int,
        socket_vector: *mut ::c_int,
    ) -> ::c_int;

    // icotl.h
    pub fn ioctl(
        fd: ::c_int, 
        request: ::c_int, 
        ...
    ) -> ::c_int;

    // fcntl.h
    pub fn fcntl(
        fd: ::c_int, 
        cmd: ::c_int, ...
    ) -> ::c_int;

    // ntp_rfc2553.h for kernel
    // netdb.h for user
    pub fn gai_strerror(
        errcode: ::c_int
    ) -> *mut ::c_char;

    // ioLib.h or
    // unistd.h
    pub fn close(
        fd: ::c_int
    ) -> ::c_int;

    // ioLib.h or
    // unistd.h
    pub fn read( // Since this is from FD< big errors might happen
        fd: ::c_int, 
        buf: *mut ::c_void, 
        count: ::size_t
    ) -> ::ssize_t;

    // ioLib.h or
    // unistd.h
    pub fn write(
        fd: ::c_int, 
        buf: *const ::c_void, 
        count: ::size_t
    ) -> ::ssize_t;

    // ioLib.h or
    // unistd.h
    pub fn isatty(
        fd : ::c_int
    ) -> ::c_int;

    // ioLib.h or
    // unistd.h
    pub fn dup(
        src: ::c_int,
    ) -> ::c_int;

    // ioLib.h or
    // unistd.h
    pub fn dup2(
        src: ::c_int, 
        dst: ::c_int,
    ) -> ::c_int;

    // ioLib.h or
    // unistd.h
    pub fn pipe (
        fds: *mut ::c_int // this is suppose to be an array, but doesn't seem to matter
                          // whether or not it is, so just keep this.
    ) -> ::c_int;

    // ioLib.h or
    // unistd.h
    pub fn unlink (
        pathname: *const ::c_char,
    ) -> ::c_int;

    // unistd.h and
    // ioLib.h
    pub fn lseek(
        fd: ::c_int,
        offset: off_t,
        whence: ::c_int,
    ) -> off_t;

    // netdb.h
    pub fn getaddrinfo(
        node: *const ::c_char,
        service: *const ::c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> ::c_int;

    // netdb.h
    pub fn freeaddrinfo(
        res: *mut addrinfo
    );

    // signal.h
    pub fn signal( // Probably wrong ...
        signum: ::c_int, 
        handler: sighandler_t
    ) -> sighandler_t;
    
    // unistd.h
    #[cfg(feature = "_WRS_KERNEL")]
    pub fn getpid(
    ) -> ::RTP_ID;

    // unistd.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn getpid(
    ) -> ::c_int; //should be pid_t, but is being dodged

    // unistd.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn getppid (
    ) -> ::c_int; // defined in b_pid_t.h - am dodging the pid_t thing
                  //should be pid_t, but is being dodged

    // wait.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn waitpid(
        pid: ::c_int, //should be pid_t, but is being dodged
        status: *mut ::c_int,
        optons: ::c_int,
    ) -> ::c_int; //should be pid_t, but is being dodged

    // unistd.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn sysconf (
        attr: ::c_int
    ) -> ::c_long;

    // unistd.h
    // For user space, return value is static inline int
    // For kernel space, exactly how it should be
    pub fn getpagesize ( 
    ) -> ::c_int;

    // stdlib.h
    pub fn setenv ( // setenv.c
        envVarName: *const ::c_char,     /* environment variable name */
        envVarValue: *const ::c_char,    /* environment variable value */
        overwrite: ::c_int        /* if non-zero, change value when var exists */
    ) -> ::c_int;

    // stdlib.h
    pub fn unsetenv ( // setenv.c
        envVarName: *const ::c_char, /* name of environment variable to remove */
    ) -> ::c_int;

    // unistd.h
    pub fn link(
        src: *const ::c_char, 
        dst: *const ::c_char,
    ) -> ::c_int;

    // unistd.h
    pub fn readlink (
        path: *const ::c_char,
        buf: *mut ::c_char,
        bufsize: ::size_t,
    ) -> ::ssize_t;

    // unistd.h
    pub fn symlink (
        path1: *const ::c_char,
        path2: *const ::c_char,
    ) -> ::c_int;

    // dirent.h
    pub fn opendir (
        name: *const ::c_char,
    ) -> *mut ::DIR;

    // unistd.h
    pub fn rmdir (
        path: *const ::c_char,
    ) -> ::c_int;

    // stat.h
    #[cfg(feature = "_WRS_KERNEL")]
    pub fn mkdir (
        dirName: *const ::c_char,
    ) -> ::c_int;

    // stat.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn mkdir (
        dirName: *const ::c_char,
        mode: ::mode_t,
    ) -> ::c_int;

    // stat.h
    pub fn chmod(
        path: *const ::c_char, 
        mode: ::mode_t,
    ) -> ::c_int;

    // stat.h
    pub fn fchmod (
        attr1: ::c_int,
        attr2: ::mode_t,
    ) -> ::c_int;

    // unistd.h
    pub fn fsync (
        fd: ::c_int,
    ) -> ::c_int;

    // dirent.h
    pub fn closedir (
        ptr: *mut ::DIR,
    ) -> ::c_int;

    pub fn pwrite64( //pwrite and pread are dummy functions in the 64 bit form, I haven't verified that they work or exist as of the time of this being written
        fd: ::c_int,  // if you want to use fd, you gotta fix these
        buf: *const ::c_void, 
        count: ::size_t,
        offset: off64_t
    ) -> ::ssize_t;

    pub fn pread64(
        fd: ::c_int, 
        buf: *const ::c_void, 
        count: ::size_t,
        offset: off64_t
    ) -> ::ssize_t;

    // mdep.h
    #[cfg(feature = "_WRS_KERNEL")]
    pub fn pwrite( 
        fd: ::c_int,
        buf: *const ::c_void, 
        count: ::size_t,
        offset: ::off_t
    ) -> ::ssize_t;

    // mdep.h
    #[cfg(feature = "_WRS_KERNEL")]
    pub fn pread(
        fd: ::c_int, 
        buf: *const ::c_void, 
        count: ::size_t,
        offset: ::off_t
    ) -> ::ssize_t;
    
    // sched.h
    pub fn sched_yield (
    ) -> ::c_int;

    // errnoLib.h
    pub fn errnoSet (
        err: ::c_int,
    ) -> ::c_int;

    // errnoLib.h
    pub fn errnoGet (
    ) -> ::c_int;

    pub fn fork( // Does not exist at all
    ) -> ::c_int;

    // unistd.h
    #[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn _exit(
        status : ::c_int
    ) -> !;

    // unistd.h
    pub fn setgid(
        gid: ::gid_t
    ) -> ::c_int;

    // unistd.h
	pub fn getgid(

	) -> ::gid_t;

    // unistd.h
    pub fn setuid(
        uid: ::uid_t
    ) -> ::c_int;


    // unistd.h
	pub fn getuid(

	) -> ::uid_t;

    pub fn setgroups( // Does not exist at all
        ngroups: ::c_int, 
        grouplist: *const ::gid_t
    ) -> ::c_int;

    // signal.h
    pub fn sigemptyset(
        __set: *mut sigset_t
    ) -> ::c_int;

    // pthread.h for kernel
    // signal.h for user
    pub fn pthread_sigmask(
        __how: ::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::c_int;

    pub fn execvp( // Does not exist at all
        c: *const ::c_char,
        argv: *const *const ::c_char
    ) -> ::c_int;

    // signal.h for user
    #[cfg(feature = "_WRS_KERNEL")]
    pub fn kill(
        __tid  : ::_Vx_TASK_ID,
        __signo: ::c_int,
    ) -> ::c_int;

    // signal.h for user
	#[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn kill(
        __pid  : ::c_int, //should be pid_t, but is being dodged
        __signo: ::c_int, 
    ) -> ::c_int;

    // signal.h for user
    #[cfg(feature = "_WRS_KERNEL")]
	pub fn sigqueue(
		__tid  : ::_Vx_TASK_ID,
		__signo: ::c_int,
		__value: ::size_t, // Actual type is const union sigval value,
                           // which is a union of int and void * 
	) -> ::c_int;

    // signal.h for user
	#[cfg(not(feature = "_WRS_KERNEL"))]
	pub fn sigqueue(
		__pid  : ::c_int, //should be pid_t, but is being dodged
		__signo: ::c_int,
		__value: ::size_t, // Actual type is const union sigval value,
                           // which is a union of int and void * 
	) -> ::c_int;

    // signal.h for user
	pub fn _sigqueue(
		rtpId  : ::RTP_ID,
		signo  : ::c_int,
		pValue : *mut ::size_t, // Actual type is const union * sigval value,
                                // which is a union of int and void *
		sigCode: ::c_int,
	) -> ::c_int;

	// signal.h
    // It seems like for kernel space, this function doesn't actually exist,
    // it just macros to kill
	pub fn taskKill(
		taskId: ::TASK_ID,
		signo : ::c_int
	) -> ::c_int;

	// signal.h
	pub fn raise (
		__signo: ::c_int,
	) -> ::c_int;

    // taskLibCommon.h
	pub fn taskIdSelf(

	) -> ::TASK_ID;

    // rtpLibCommon.h
    pub fn rtpInfoGet(
        rtpId : ::RTP_ID,
        rtpStruct : *mut ::RTP_DESC,
    ) -> ::c_int;

    // ioLib.h
	#[cfg(not(feature = "_WRS_KERNEL"))]
    pub fn _realpath
    (
        fileName: *const ::c_char,
        resolvedName: *mut ::c_char,
    ) -> *mut ::c_char;

    // pathLib.h
    //#[cfg(feature = "__RTP__")]
    pub fn _pathIsAbsolute (
        filepath: *const ::c_char,
        pNameTail: *const *const ::c_char,
    ) -> bool;

}

//Dummy functions, these don't really exist in VxWorks.

// wait.h macros
pub fn WIFEXITED(status: ::c_int) -> bool {
    (status & 0xFF00) == 0
}
pub fn WIFSIGNALED(status: ::c_int) -> bool{
	(status & 0xFF00) != 0
}	
pub fn WIFSTOPPED(status: ::c_int) -> bool{
	(status & 0xFF0000) != 0
}
pub fn WEXITSTATUS(status: ::c_int) -> ::c_int{
	status & 0xFF
}
pub fn WTERMSIG(status: ::c_int) -> ::c_int{	
	(status >> 8) & 0xFF
}
pub fn WSTOPSIG(status: ::c_int) -> ::c_int{
	(status >> 16) & 0xFF
}

#[cfg(not(feature = "_WRS_KERNEL"))]
pub fn pread(fd: ::c_int, buf: *mut ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t {
    1
}

#[cfg(not(feature = "_WRS_KERNEL"))]
pub fn pwrite(fd: ::c_int, buf: *const ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t {
    1
}
pub fn posix_memalign (mut memptr: *mut *mut ::c_void, align: ::size_t, size: ::size_t) -> ::c_int {
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
        }
        else {
            *memptr = temp;
            0
        }
    }
}

// From sysconf.c -> doesn't seem to be supported?
pub fn getpwuid_r(uid: ::uid_t,    pwd: *mut passwd, buf: *mut ::c_char, buflen: ::size_t,    result: *mut *mut passwd) -> ::c_int {
    0
}

// VxWorks requires that resolvedName be allocated in userspace
pub fn realpath (fileName: *const ::c_char, resolvedName: *mut ::c_char,) -> *mut ::c_char {
    unsafe{
        if(resolvedName == null_mut::<::c_char>()){
            let emptyResolvedName = super::malloc(::_POSIX_PATH_MAX as _) as *mut ::c_char;
            let r = _realpath (fileName, emptyResolvedName);

            if (r == null_mut::<::c_char>()) {
                super::free(emptyResolvedName as *mut _);
            }
            r
        } else {
            _realpath (fileName, resolvedName) 
        }
    }
}

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(any(target_arch = "armv7"))] {
        mod armv7;
        pub use self::armv7::*;
    }  else if #[cfg(any(target_arch = "x86"))] {
        mod x86;
        pub use self::x86::*;
    } else if #[cfg(any(target_arch = "x86_64"))] {
        mod x86_64;
        pub use self::x86_64::*;
    } else {
        // Unknown target_arch
    }
}
