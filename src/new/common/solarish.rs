//! Interfaces common in solaris-derived operating systems. This includes Solaris and Illumos.

pub(crate) mod sys {
    pub(crate) mod socket {
        use crate::prelude::*;
        use crate::{
            cmsghdr,
            msghdr,
        };

        const fn _CMSG_DATA_ALIGN(p: usize) -> usize {
            crate::new::common::posix::sys::socket::align_impl(p, size_of::<c_int>())
        }

        pub const fn CMSG_LEN(length: c_uint) -> c_uint {
            _CMSG_DATA_ALIGN(size_of::<cmsghdr>()) as c_uint + length
        }

        pub unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
            _CMSG_DATA_ALIGN(cmsg.offset(1) as usize) as *mut c_uchar
        }

        pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
            if cmsg.is_null() {
                return crate::CMSG_FIRSTHDR(mhdr);
            }

            crate::new::common::posix::sys::socket::CMSG_NXTHDR(mhdr, cmsg)
        }
    }
}
