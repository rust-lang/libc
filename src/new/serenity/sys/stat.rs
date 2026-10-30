//! Header: `sys/stat.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/sys/stat.h>
pub use super::super::Kernel::API::POSIX::sys::stat::*;
use crate::prelude::*;

extern "C" {
    pub fn mkfifoat(dirfd: c_int, pathname: *const c_char, mode: crate::mode_t) -> c_int;
    pub fn mknodat(
        dirfd: c_int,
        pathname: *const c_char,
        mode: crate::mode_t,
        dev: crate::dev_t,
    ) -> c_int;
    pub fn futimens(fd: c_int, times: *const crate::timespec) -> c_int;
    pub fn utimensat(
        dirfd: c_int,
        path: *const c_char,
        times: *const crate::timespec,
        flag: c_int,
    ) -> c_int;
}
