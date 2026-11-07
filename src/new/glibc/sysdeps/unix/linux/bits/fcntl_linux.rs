//! Header: `sysdeps/unix/sysv/linux/bits/fcntl-linux.h`
//!
//! <https://github.com/sailfishos-mirror/glibc/blob/master/sysdeps/unix/sysv/linux/bits/fcntl-linux.h>

pub use crate::open_how;
use crate::prelude::*;

extern "C" {
    pub fn openat2(dirfd: c_int, path: *const c_char, how: *const open_how, size: size_t) -> c_int;
}
