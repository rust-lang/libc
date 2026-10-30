//! File `Kernel/API/POSIX/sys/socket.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/sys/socket.h>
use crate::prelude::*;

pub const AF_MASK: i32 = 0xff;
pub const AF_UNSPEC: i32 = 0;
pub const AF_LOCAL: i32 = 1;
pub const AF_UNIX: i32 = AF_LOCAL;
pub const AF_INET: i32 = 2;
pub const AF_INET6: i32 = 3;

/// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
/// for details.
pub const AF_MAX: i32 = 4;

pub const PF_LOCAL: i32 = AF_LOCAL;
pub const PF_UNIX: i32 = PF_LOCAL;
pub const PF_INET: i32 = AF_INET;
pub const PF_INET6: i32 = AF_INET6;
pub const PF_UNSPEC: i32 = AF_UNSPEC;

/// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
/// for details.
pub const PF_MAX: i32 = AF_MAX;

pub const SOCK_TYPE_MASK: i32 = 0xff;
pub const SOCK_STREAM: i32 = 1;
pub const SOCK_DGRAM: i32 = 2;
pub const SOCK_RAW: i32 = 3;
pub const SOCK_RDM: i32 = 4;
pub const SOCK_SEQPACKET: i32 = 5;
pub const SOCK_NONBLOCK: i32 = 0o4000;
pub const SOCK_CLOEXEC: i32 = 0o2000000;

pub const SHUT_RD: i32 = 0;
pub const SHUT_WR: i32 = 1;
pub const SHUT_RDWR: i32 = 2;

pub const IPPROTO_IGMP: i32 = 2;
pub const IPPROTO_IPIP: i32 = 4;
pub const IPPROTO_ESP: i32 = 50;
pub const IPPROTO_AH: i32 = 51;
pub const IPPROTO_RAW: i32 = 256;

pub const MSG_TRUNC: i32 = 0x1;
pub const MSG_CTRUNC: i32 = 0x2;
pub const MSG_PEEK: i32 = 0x4;
pub const MSG_OOB: i32 = 0x8;
pub const MSG_DONTROUTE: i32 = 0x10;
pub const MSG_WAITALL: i32 = 0x20;
pub const MSG_DONTWAIT: i32 = 0x40;
pub const MSG_NOSIGNAL: i32 = 0x80;
pub const MSG_EOR: i32 = 0x100;

pub type sa_family_t = u16;

s! {
    pub struct cmsghdr {
        cmsg_len: crate::socklen_t,
        cmsg_level: i32,
        cmsg_type: i32,
    }

    pub struct msghdr {
        pub msg_name: *mut c_void,
        pub msg_namelen: crate::socklen_t,
        pub msg_iov: *mut crate::iovec,
        pub msg_iovlen: c_int,
        pub msg_control: *mut c_void,
        pub msg_controllen: crate::socklen_t,
        pub msg_flags: c_int,
    }
}

const fn CMSG_ALIGN(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

f! {
    pub const unsafe fn CMSG_SPACE(length: c_uint) -> c_uint {
        (CMSG_ALIGN(length as usize) + CMSG_ALIGN(size_of::<cmsghdr>())) as c_uint
    }

    pub const unsafe fn CMSG_LEN(length: c_uint) -> c_uint {
        CMSG_ALIGN(size_of::<cmsghdr>()) as c_uint + length
    }

    pub unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
        if (*mhdr).msg_controllen as usize >= size_of::<cmsghdr>() {
            (*mhdr).msg_control.cast()
        } else {
            ptr::null_mut()
        }
    }

    pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
        if ((*cmsg).cmsg_len as usize) < size_of::<cmsghdr>() {
            return ptr::null_mut();
        }
        let next = (cmsg as usize + CMSG_ALIGN((*cmsg).cmsg_len as usize)) as *mut cmsghdr;
        let max = (*mhdr).msg_control as usize + (*mhdr).msg_controllen as usize;
        if (next.offset(1)) as usize > max
            || next as usize + CMSG_ALIGN((*next).cmsg_len as usize) > max
        {
            ptr::null_mut()
        } else {
            next.cast()
        }
    }

    pub unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
        (cmsg as *mut c_uchar).offset(CMSG_ALIGN(size_of::<cmsghdr>()) as isize)
    }
}

s! {
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 26],
    }

    pub struct ucred {
        pub pid: crate::pid_t,
        pub uid: crate::uid_t,
        pub gid: crate::gid_t,
    }
}

pub const SOL_SOCKET: i32 = 1;
pub const SOMAXCONN: i32 = 128;

pub const SO_RCVTIMEO: i32 = 0;
pub const SO_SNDTIMEO: i32 = 1;
pub const SO_TYPE: i32 = 2;
pub const SO_ERROR: i32 = 3;
pub const SO_PEERCREDS: i32 = 4;
pub const SO_RECVBUF: i32 = 5;
pub const SO_SNDBUF: i32 = 6;
pub const SO_DEBUG: i32 = 7;
pub const SO_REUSEADDR: i32 = 8;
pub const SO_BINDTODEVICE: i32 = 9;
pub const SO_KEEPALIVE: i32 = 10;
pub const SO_TIMESTAMP: i32 = 11;
pub const SO_BROADCAST: i32 = 12;
pub const SO_LINGER: i32 = 13;
pub const SO_ACCEPTCONN: i32 = 14;
pub const SO_DONTROUTE: i32 = 15;
pub const SO_OOBINLINE: i32 = 16;
pub const SO_SNDLOWAT: i32 = 17;
pub const SO_RCVLOWAT: i32 = 18;

pub const SCM_TIMESTAMP: i32 = 0;
pub const SCM_RIGHTS: i32 = 1;

s! {
    #[repr(align(16))]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        pub data: [c_char; 110], // sizeof(sockaddr_un)
    }
}
