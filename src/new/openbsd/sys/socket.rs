//! Header: `sys/socket.h`
//!
//! <https://github.com/openbsd/src/blob/master/sys/sys/socket.h>

pub use crate::new::common::bsd::sys::socket::CMSG_NXTHDR;
pub use crate::new::common::posix::sys::socket::{
    CMSG_DATA,
    CMSG_FIRSTHDR,
    CMSG_LEN,
    CMSG_SPACE,
};
