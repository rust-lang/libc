//! Header: `sys/socket.h`

pub use crate::new::common::posix::sys::socket::{
    CMSG_FIRSTHDR,
    CMSG_SPACE,
};
pub use crate::new::common::solarish::sys::socket::{
    CMSG_DATA,
    CMSG_LEN,
    CMSG_NXTHDR,
};
