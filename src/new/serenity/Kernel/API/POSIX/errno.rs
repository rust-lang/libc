//! File `Kernel/API/POSIX/errno.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/errno.h>
use crate::prelude::*;

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
pub const ERANGE: c_int = 33;
pub const ENAMETOOLONG: c_int = 34;
pub const ELOOP: c_int = 35;
pub const EOVERFLOW: c_int = 36;
pub const EOPNOTSUPP: c_int = 37;
pub const ENOSYS: c_int = 38;
pub const ENOTIMPL: c_int = 39;
pub const EAFNOSUPPORT: c_int = 40;
pub const ENOTSOCK: c_int = 41;
pub const EADDRINUSE: c_int = 42;
pub const ENOTEMPTY: c_int = 43;
pub const EDOM: c_int = 44;
pub const ECONNREFUSED: c_int = 45;
pub const EHOSTDOWN: c_int = 46;
pub const EADDRNOTAVAIL: c_int = 47;
pub const EISCONN: c_int = 48;
pub const ECONNABORTED: c_int = 49;
pub const EALREADY: c_int = 50;
pub const ECONNRESET: c_int = 51;
pub const EDESTADDRREQ: c_int = 52;
pub const EHOSTUNREACH: c_int = 53;
pub const EILSEQ: c_int = 54;
pub const EMSGSIZE: c_int = 55;
pub const ENETDOWN: c_int = 56;
pub const ENETUNREACH: c_int = 57;
pub const ENETRESET: c_int = 58;
pub const ENOBUFS: c_int = 59;
pub const ENOLCK: c_int = 60;
pub const ENOMSG: c_int = 61;
pub const ENOPROTOOPT: c_int = 62;
pub const ENOTCONN: c_int = 63;
pub const ESHUTDOWN: c_int = 64;
pub const ETOOMANYREFS: c_int = 65;
pub const ESOCKTNOSUPPORT: c_int = 66;
pub const EPROTONOSUPPORT: c_int = 67;
pub const EDEADLK: c_int = 68;
pub const ETIMEDOUT: c_int = 69;
pub const EPROTOTYPE: c_int = 70;
pub const EINPROGRESS: c_int = 71;
pub const ENOTHREAD: c_int = 72;
pub const EPROTO: c_int = 73;
pub const ENOTSUP: c_int = 74;
pub const EPFNOSUPPORT: c_int = 75;
pub const EDIRINTOSELF: c_int = 76;
pub const EDQUOT: c_int = 77;
pub const ENOTRECOVERABLE: c_int = 78;
pub const ECANCELED: c_int = 79;
pub const EPROMISEVIOLATION: c_int = 80;
pub const ESTALE: c_int = 81;
pub const ESRCNOTFOUND: c_int = 82;

/// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
/// for details.
pub const EMAXERRNO: c_int = 83;

// From LibC/errno_codes.h
pub const EWOULDBLOCK: c_int = EAGAIN;

/// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
/// for details.
pub const ELAST: c_int = EMAXERRNO;
