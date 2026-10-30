//! Header: `sys/uio.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/sys/uio.h>
pub use super::super::Kernel::API::POSIX::sys::uio::*;
use crate::prelude::*;

extern "C" {
    pub fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> isize;
    pub fn preadv(
        fd: c_int,
        iov: *const crate::iovec,
        iovcnt: c_int,
        offset: crate::off_t,
    ) -> isize;
    pub fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> isize;
    pub fn pwritev(
        fd: c_int,
        iov: *const crate::iovec,
        iovcnt: c_int,
        offset: crate::off_t,
    ) -> isize;
}
