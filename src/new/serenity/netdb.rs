//! Header: `netdb.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/netdb.h>
use crate::prelude::*;

s! {
    pub struct addrinfo {
        pub ai_flags: c_int,
        pub ai_family: c_int,
        pub ai_socktype: c_int,
        pub ai_protocol: c_int,
        pub ai_addrlen: crate::socklen_t,
        pub ai_addr: *mut crate::sockaddr,
        pub ai_canonname: *mut c_char,
        pub ai_next: *mut addrinfo,
    }
}

pub const HOST_NOT_FOUND: c_int = 101;
pub const NO_DATA: c_int = 102;
pub const NO_ADDRESS: c_int = NO_DATA;
pub const NO_RECOVERY: c_int = 103;
pub const TRY_AGAIN: c_int = 104;

pub const EAI_ADDRFAMILY: c_int = 1;
pub const EAI_AGAIN: c_int = 2;
pub const EAI_BADFLAGS: c_int = 3;
pub const EAI_FAIL: c_int = 4;
pub const EAI_FAMILY: c_int = 5;
pub const EAI_MEMORY: c_int = 6;
pub const EAI_NODATA: c_int = 7;
pub const EAI_NONAME: c_int = 8;
pub const EAI_SOCKTYPE: c_int = 9;
pub const EAI_SERVICE: c_int = 10;
pub const EAI_SYSTEM: c_int = 11;
pub const EAI_OVERFLOW: c_int = 12;

pub const AI_PASSIVE: c_int = 0x0001;
pub const AI_CANONNAME: c_int = 0x0002;
pub const AI_NUMERICHOST: c_int = 0x0004;
pub const AI_NUMERICSERV: c_int = 0x0008;
pub const AI_V4MAPPED: c_int = 0x0010;
pub const AI_ALL: c_int = 0x0020;
pub const AI_ADDRCONFIG: c_int = 0x0040;

pub const NI_MAXHOST: c_int = 1025;
pub const NI_MAXSERV: c_int = 32;

pub const NI_NUMERICHOST: c_int = 1;
pub const NI_NUMERICSERV: c_int = 2;
pub const NI_NAMEREQD: c_int = 4;
pub const NI_NOFQDN: c_int = 8;
pub const NI_DGRAM: c_int = 16;

extern "C" {
    pub fn getnameinfo(
        sa: *const crate::sockaddr,
        salen: crate::socklen_t,
        host: *mut c_char,
        hostlen: crate::socklen_t,
        serv: *mut c_char,
        servlen: crate::socklen_t,
        flags: c_int,
    ) -> c_int;
}
