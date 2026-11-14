//! Header: `sys/socket.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/sys/socket.h>
pub use super::super::Kernel::API::POSIX::sys::socket::*;
use crate::prelude::*;

extern "C" {
    pub fn bind(sockfd: c_int, addr: *const crate::sockaddr, len: crate::socklen_t) -> c_int;
    pub fn accept4(
        fd: c_int,
        addr: *mut crate::sockaddr,
        len: *mut crate::socklen_t,
        flags: c_int,
    ) -> c_int;
    pub fn sendmsg(sockfd: c_int, msg: *const crate::msghdr, flags: c_int) -> c_int;
    pub fn recvmsg(sockfd: c_int, msg: *mut crate::msghdr, flags: c_int) -> c_int;
    pub fn recvfrom(
        sockfd: c_int,
        buf: *mut c_void,
        len: crate::size_t,
        flags: c_int,
        src_addr: *mut crate::sockaddr,
        addrlen: *mut crate::socklen_t,
    ) -> c_int;
    // Omit serenity-specific sendfd/recvfd
}
