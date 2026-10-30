//! Header: `fcntl.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/fcntl.h>
pub use super::Kernel::API::POSIX::fcntl::*;
use crate::prelude::*;

pub const POSIX_FADV_DONTNEED: c_int = 1;
pub const POSIX_FADV_NOREUSE: c_int = 2;
pub const POSIX_FADV_NORMAL: c_int = 3;
pub const POSIX_FADV_RANDOM: c_int = 4;
pub const POSIX_FADV_SEQUENTIAL: c_int = 5;
pub const POSIX_FADV_WILLNEED: c_int = 6;

extern "C" {
    pub fn create_inode_watcher(flags: c_uint) -> c_int;
    pub fn inode_watcher_add_watch(
        fd: c_int,
        path: *const c_char,
        path_length: usize,
        event_mask: c_uint,
    ) -> c_int;
    pub fn inode_watcher_remove_watch(fd: c_int, wd: c_int) -> c_int;

    pub fn posix_fadvise(
        fd: c_int,
        offset: crate::off_t,
        len: crate::off_t,
        advice: c_int,
    ) -> c_int;
    pub fn posix_fallocate(fd: c_int, offset: crate::off_t, len: crate::off_t) -> c_int;
}
