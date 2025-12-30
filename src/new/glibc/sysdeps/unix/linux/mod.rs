//! Source directory: `sysdeps/unix/sysv/linux` (the `sysv` is flattened).
//!
//! <https://github.com/bminor/glibc/tree/master/sysdeps/unix/sysv/linux>

/// Directory: `net/`
///
/// Source directory: `sysdeps/unix/sysv/linux/net`
pub(crate) mod net {
    pub(crate) mod route;
}

/// Directory: `sys/`
pub(crate) mod sys {
    pub(crate) mod socket {
        //! Header: `sys/socket.h`
        //!
        //! <https://github.com/bminor/glibc/blob/master/sysdeps/unix/sysv/linux/bits/socket.h>

        pub use crate::new::common::posix::sys::socket::{
            CMSG_ALIGN,
            CMSG_DATA,
            CMSG_FIRSTHDR,
            CMSG_LEN,
            CMSG_NXTHDR,
            CMSG_SPACE,
        };
    }
}
