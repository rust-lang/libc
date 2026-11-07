//! Values are sourced from the Hexagon toolchain's own `errno.h`
//! (`tools/HEXAGON_Tools/<ver>/Tools/target/hexagon/include/errno.h`).

use super::*;
use crate::prelude::*;

// -- Unconditionally defined (shared across all library conventions) --
pub const ENOENT: c_int = 2; // 0x0002
pub const EFAULT: c_int = 14; // 0x000E
pub const ENFILE: c_int = 23; // 0x0017
pub const ENOTTY: c_int = 25; // 0x0019
pub const EFBIG: c_int = 27; // 0x001B
pub const ENOSPC: c_int = 28; // 0x001C
pub const ESPIPE: c_int = 29; // 0x001D
pub const EROFS: c_int = 30; // 0x001E
pub const EMLINK: c_int = 31; // 0x001F
pub const EPIPE: c_int = 32; // 0x0020
pub const EDOM: c_int = 33; // 0x0021
pub const ERANGE: c_int = 34; // 0x0022
pub const EFPOS: c_int = 152; // 0x0098

// -- Non-Borland branch (the one that applies to Hexagon) --
pub const EPERM: c_int = 1; // 0x0001
pub const EINTR: c_int = 4; // 0x0004
pub const EIO: c_int = 5; // 0x0005
pub const ENXIO: c_int = 6; // 0x0006
pub const E2BIG: c_int = 7; // 0x0007
pub const ENOEXEC: c_int = 8; // 0x0008
pub const EBADF: c_int = 9; // 0x0009
pub const ECHILD: c_int = 10; // 0x000A
pub const EAGAIN: c_int = 11; // 0x000B
pub const ENOMEM: c_int = 12; // 0x000C
pub const EACCES: c_int = 13; // 0x000D
pub const EBUSY: c_int = 16; // 0x0010
pub const EEXIST: c_int = 17; // 0x0011
pub const EXDEV: c_int = 18; // 0x0012
pub const ENODEV: c_int = 19; // 0x0013
pub const ENOTDIR: c_int = 20; // 0x0014
pub const EISDIR: c_int = 21; // 0x0015
pub const EINVAL: c_int = 22; // 0x0016
pub const EMFILE: c_int = 24; // 0x0018
pub const ESRCH: c_int = 3; // 0x0003
pub const EWOULDBLOCK: c_int = EAGAIN;

// -- "library type" default branch (the one that applies to Hexagon) --
pub const EDEADLK: c_int = 45; // 0x002D
pub const ENOLCK: c_int = 46; // 0x002E
pub const ECANCELED: c_int = 47; // 0x002F
pub const ENOTSUP: c_int = 48; // 0x0030
pub const EBADMSG: c_int = 77; // 0x004D
pub const ENAMETOOLONG: c_int = 78; // 0x004E
pub const EILSEQ: c_int = 88; // 0x0058
pub const ENOSYS: c_int = 89; // 0x0059
pub const EMSGSIZE: c_int = 97; // 0x0061
pub const ENOTEMPTY: c_int = 93; // 0x005D
pub const EINPROGRESS: c_int = 150; // 0x0096
pub const ETIMEDOUT: c_int = 145; // 0x0091 

// -- Open-std-specific, unconditionally defined --
pub const EOVERFLOW: c_int = 146; // 0x0092

// -- Placeholder constants (NOT present in the Hexagon toolchain's errno.h) --
// Since hexagon-unknown-qurt target in rust-lang/rust borrows lots of unix code, the following are needed
// for a successful build even though they are not declared in QuRT dinkumware libc headers.
pub const ELOOP: c_int = 40;
pub const ETXTBSY: c_int = 26;
pub const EOPNOTSUPP: c_int = 95;
pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const ENETDOWN: c_int = 100;
pub const ENETUNREACH: c_int = 101;
pub const ECONNABORTED: c_int = 103;
pub const ECONNRESET: c_int = 104;
pub const ENOTCONN: c_int = 107;
pub const ECONNREFUSED: c_int = 111;
pub const EHOSTUNREACH: c_int = 113;
pub const ESTALE: c_int = 116;
pub const EDQUOT: c_int = 122;

extern "C" {
    // Error number access
    #[link_name = "__errno_location"]
    pub fn __errno_location() -> *mut c_int;
}

pub unsafe fn errno() -> c_int {
    *__errno_location()
}

pub unsafe fn set_errno(value: c_int) {
    *__errno_location() = value;
}
