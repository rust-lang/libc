//! Header: `sys/socket.h`
//!
//! <https://github.com/kernkonzept/l4re-core/blob/master/libc/uclibc-ng/contrib/uclibc/libc/sysdeps/linux/common/bits/socket.h>

pub use crate::new::common::posix::sys::socket::{
    CMSG_ALIGN,
    CMSG_DATA,
    CMSG_FIRSTHDR,
    CMSG_LEN,
    CMSG_NXTHDR,
    CMSG_SPACE,
};
