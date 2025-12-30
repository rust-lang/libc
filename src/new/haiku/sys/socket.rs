//! Header: `sys/socket.h`
//!
//! <https://github.com/haiku/haiku/blob/master/headers/posix/sys/socket.h>

pub use crate::new::common::posix::sys::socket::{
    CMSG_DATA,
    CMSG_FIRSTHDR,
    CMSG_LEN,
    CMSG_SPACE,
};

pub unsafe fn CMSG_NXTHDR(
    mhdr: *const crate::msghdr,
    cmsg: *const crate::cmsghdr,
) -> *mut crate::cmsghdr {
    if cmsg.is_null() {
        return CMSG_FIRSTHDR(mhdr);
    }

    crate::new::common::posix::sys::socket::CMSG_NXTHDR(mhdr, cmsg)
}
