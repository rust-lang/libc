//! Header: `sys/socket.h`
//!
//! <https://gitlab.redox-os.org/redox-os/relibc/-/blob/master/src/header/sys_socket/mod.rs>

pub use crate::new::common::posix::sys::socket::{
    CMSG_ALIGN,
    CMSG_DATA,
    CMSG_FIRSTHDR,
    CMSG_LEN,
    CMSG_NXTHDR,
    CMSG_SPACE,
};
