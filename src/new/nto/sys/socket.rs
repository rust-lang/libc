//! Header: `sys/socket.h`
//!
//! Manual pages: <https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.io_sock/topic/socket_api.html#socket_api__socket_api>

pub use crate::new::common::posix::sys::socket::{
    CMSG_DATA,
    CMSG_FIRSTHDR,
    CMSG_LEN,
    CMSG_NXTHDR,
    CMSG_SPACE,
};
