//! Header: `sys/socket.h`
//!
//! <https://github.com/bminor/glibc/blob/master/sysdeps/mach/hurd/bits/socket.h>

pub use crate::new::common::posix::sys::socket::{
    CMSG_ALIGN,
    CMSG_DATA,
    CMSG_FIRSTHDR,
    CMSG_LEN,
    CMSG_NXTHDR,
    CMSG_SPACE,
};
