//! Header: `sys/socket.h`
//!
//! <https://github.com/emscripten-core/emscripten/blob/main/system/lib/libc/musl/include/sys/socket.h>

pub use crate::new::common::posix::sys::socket::{
    CMSG_ALIGN,
    CMSG_DATA,
    CMSG_FIRSTHDR,
    CMSG_LEN,
    CMSG_SPACE,
};

pub unsafe fn CMSG_NXTHDR(
    mhdr: *const crate::msghdr,
    cmsg: *const crate::cmsghdr,
) -> *mut crate::cmsghdr {
    crate::new::common::posix::sys::socket::next_impl(mhdr, cmsg, false)
}
