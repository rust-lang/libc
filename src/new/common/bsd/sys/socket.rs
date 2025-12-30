#[cfg(not(target_os = "dragonfly"))]
pub unsafe fn CMSG_NXTHDR(
    mhdr: *const crate::msghdr,
    cmsg: *const crate::cmsghdr,
) -> *mut crate::cmsghdr {
    if cmsg.is_null() {
        return crate::CMSG_FIRSTHDR(mhdr);
    }

    crate::new::common::posix::sys::socket::CMSG_NXTHDR(mhdr, cmsg)
}
