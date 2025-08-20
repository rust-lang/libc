//! Header: `sys/socket.h`

use crate::prelude::*;
use crate::new::musl::arch::bits;

s! {
    pub struct msghdr {
        pub msg_name: *mut c_void,
        pub msg_namelen: crate::socklen_t,
        pub msg_iov: *mut crate::iovec,
        #[cfg(all(target_pointer_width = "64", target_endian = "big"))]
        __pad1: c_int,
        pub msg_iovlen: c_int,
        #[cfg(all(target_pointer_width = "64", target_endian = "little"))]
        __pad1: c_int,
        pub msg_control: *mut c_void,
        #[cfg(all(target_pointer_width = "64", target_endian = "big"))]
        __pad2: c_int,
        pub msg_controllen: crate::socklen_t,
        #[cfg(all(target_pointer_width = "64", target_endian = "little"))]
        __pad2: c_int,
        pub msg_flags: c_int,
    }

    pub struct cmsghdr {
        #[cfg(all(target_pointer_width = "64", target_endian = "big"))]
        pub __pad1: c_int,
        pub cmsg_len: crate::socklen_t,
        #[cfg(all(target_pointer_width = "64", target_endian = "little"))]
        pub __pad1: c_int,
        pub cmsg_level: c_int,
        pub cmsg_type: c_int,
    }
}

cfg_if! {
    // #ifdef _GNU_SOURCE in musl. In practice this will always be defined for our musl targets.
    if #[cfg(target_os = "linux")] {
        s! {
            // ucred is defined in the `linux` module

            pub struct mmsghdr {
                pub msg_hdr: crate::msghdr,
                pub msg_len: c_uint,
            }
        }

        extern "C" {
            pub fn sendmmsg(
                sockfd: c_int,
                msgvec: *mut mmsghdr,
                vlen: c_uint,
                flags: c_uint,
            ) -> c_int;
            pub fn recvmmsg(
                sockfd: c_int,
                msgvec: *mut mmsghdr,
                vlen: c_uint,
                flags: c_uint,
                timeout: *mut crate::timespec,
            ) -> c_int;
        }
    }
}

// SHUT_* constants are in the `linux` module

cfg_if! {
    if #[cfg(any(target_arch = "mips", target_arch = "mips64"))] {
        pub use bits::socket::{SOCK_STREAM, SOCK_DGRAM};
    } else {
        pub const SOCK_STREAM: c_int = 1;
        pub const SOCK_DGRAM: c_int = 2;
    }
}

// The rest of the source header is defined in `unix` or `linux` modules
