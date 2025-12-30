//! Header: `sys/socket.h`
//!
//! <https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/sys_socket.h.html>

// TEMP: until allow(unsafe_op_in_unsafe_fn) is removed in global level.
#![deny(unsafe_op_in_unsafe_fn)]

use core::mem::size_of;

use crate::prelude::*;
use crate::{
    cmsghdr,
    msghdr,
};

/// Rounds `len` *up* to the next alignment boundary.
pub const fn CMSG_ALIGN(len: size_t) -> size_t {
    align_impl(len, size_of::<__ALIGN_BOUNDARY>())
}

pub(crate) const fn align_impl(len: usize, align: usize) -> usize {
    let mask = align - 1;
    (len + mask) & !mask
}

// TODO(#3240): consider changing signatures to `CMSG_{SPACE,LEN}(length: size_t) -> size_`

/// Total length of a non-padded control message for a payload of size `length`.
///
/// This function is almost exclusively used setting [`cmsghdr::cmsg_len`].
/// It should *not* be used for determining the actual ancillary data buffer
/// size. [`CMSG_SPACE`] should instead be for that.
#[cfg(not(any(target_os = "illumos", target_os = "solaris")))]
pub const fn CMSG_LEN(length: c_uint) -> c_uint {
    let length = length as size_t;

    // See `CMSG_SPACE` impl about this sometimes being a no-op.
    let len = CMSG_ALIGN(size_of::<cmsghdr>()) + length;

    len as c_uint
}

/// Total length of a padded control message for a payload of size `length`.
///
/// This can be used to allocate space dynamically for the ancillary data.
/// It should should *not* be used to initialize [cmsghdr::cmsg_len], given
/// that the returned value includes padding bytes. Use instead [`CMSG_LEN`]
/// for that.
pub const fn CMSG_SPACE(length: c_uint) -> c_uint {
    let length = length as size_t;

    // NB: left hand side `align` is no-op when `size_of::<cmsghdr>() %
    // size_of::<__ALIGN_BOUNDARY>() == 0`. Such is the case on Linux, and
    // probably why some implementations there don't bother with the lhs
    // align.
    let space = CMSG_ALIGN(size_of::<cmsghdr>()) + CMSG_ALIGN(length);

    space as c_uint
}

/// Returns a pointer to the payload data array associated with for the provided header.
///
/// # Safety
///
/// Safety in calling this function are based on the safety conditions needed for calling `ptr.offset()`.
///
/// * `cmsg` must point to some allocation and the resulting offset may not wrap around an address space.
//
// TODO: From https://man7.org/linux/man-pages/man3/cmsg.3.html, include it for the rest?
//
// The pointer returned cannot be assumed to be suitably aligned for
// accessing arbitrary payload data types. Applications should not cast it
// to a pointer type matching the payload, but should instead use `memcpy`
// to copy data to or from a suitably declared object.
#[cfg(not(any(target_os = "illumos", target_os = "solaris")))]
pub const unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
    let ptr = cmsg as *mut crate::c_uchar;
    // See `CMSG_SPACE` impl about this sometimes being a no-op.
    let byte_offset = CMSG_ALIGN(size_of::<cmsghdr>()) as isize;

    // SAFETY:
    // - Offset fits in isize (at most size_of::<u128>() = 16)
    // - Caller must uphold the safety contract of non-wrapping offset
    unsafe { ptr.offset(byte_offset) }
}

/// Return a pointer to the first [`cmsghdr`] in [`msghdr`].
///
/// A null pointer is returned if [`msghdr::msg_controllen`] is less than
/// `size_of::<cmsghdr>()`. This can mean that there was no ancillary data
/// in the first place (msg_controllen is zero), or that the ancillary data
/// has been truncated.
pub const unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
    // TODO: take mhdr by ref and make CMSG_FIRSTHDR safe?
    let msghdr = unsafe { *mhdr };

    // Linux's syscall interface uses `size_t` for `msg_controllen` even if
    // POSIX dictates it being `socklen_t` (>=u32). The cast accommodates
    // for both, even if redundant for Linux.
    #[allow(clippy::unnecessary_cast)]
    let msg_controllen = msghdr.msg_controllen as size_t;

    if msg_controllen < size_of::<cmsghdr>() {
        core::ptr::null_mut()
    } else {
        msghdr.msg_control.cast()
    }
}

/// # SAFETY
///
/// Safety in calling this function are based on the safety conditions needed for dereferencing `cmsg`:
///
/// * Caller must enure `cmsg` is not null.
///
/// * `cmsg` is returned from a previous call to `CMSG_NXTHDR`, or
///   originally retrieved from a call to `CMSG_FIRSTHDR`.
#[cfg(not(any(
    target_env = "musl",
    target_env = "ohos",
    target_os = "emscripten",
    target_os = "fuchsia"
)))]
pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
    unsafe { next_impl(mhdr, cmsg, true) }
}

pub(crate) unsafe fn next_impl(
    mhdr: *const msghdr,
    cmsg: *const cmsghdr,
    allow_zero_sized_payload: bool,
) -> *mut cmsghdr {
    // TODO: why not take mhdr by ref?
    let msghdr = unsafe { *mhdr };

    // SAFETY:
    // - Caller ensured pointer is not null.
    // - Pointer retrieved from either CMSG_FIRSTHDR or CMSG_NXTDR,
    //   thus ensuring that is lies between mhdr_addr and mhdr_addr +
    //   msg_controllen.
    let cmsghdr = unsafe { *cmsg };

    if (cmsghdr.cmsg_len as usize) < size_of::<cmsghdr>() {
        return core::ptr::null_mut();
    }

    // FIXME(msrv): `.wrapping_byte_add()` stabilized in 1.75
    let next_cmsg = cmsg
        .cast::<u8>()
        .wrapping_add(CMSG_ALIGN(cmsghdr.cmsg_len as usize))
        .cast::<cmsghdr>();

    // In case the addition wrapped. `next_addr > max_addr`
    // would otherwise not work as intended.
    if (next_cmsg as usize) < (cmsg as usize) {
        return core::ptr::null_mut();
    }

    let mut max_addr = msghdr.msg_control as usize + msghdr.msg_controllen as usize;

    if !allow_zero_sized_payload {
        // musl and some of its descendants do `>= max_addr`
        // comparisons in the if statement below.
        // https://www.openwall.com/lists/musl/2025/12/27/1
        max_addr -= 1;
    }

    if next_cmsg as usize + CMSG_ALIGN(size_of::<cmsghdr>()) > max_addr {
        core::ptr::null_mut()
    } else {
        next_cmsg as *mut cmsghdr
    }
}

// HACK: AIX does not use any alignment/padding for ancillary data. Setting
// this to 1 it makes possible to reuse the CMSG_* implementatinos as the extra
// CMSG_ALIGN calls become no-op.
#[cfg(target_os = "aix")]
pub(crate) type __ALIGN_BOUNDARY = u8;

#[cfg(target_os = "android")]
pub(crate) type __ALIGN_BOUNDARY = usize;

#[cfg(target_vendor = "apple")]
pub(crate) type __ALIGN_BOUNDARY = u32;

#[cfg(target_os = "hurd")]
pub(crate) type __ALIGN_BOUNDARY = usize;

#[cfg(target_os = "cygwin")]
pub(crate) type __ALIGN_BOUNDARY = usize;

#[cfg(target_os = "dragonfly")]
pub(crate) type __ALIGN_BOUNDARY = c_long;

#[cfg(target_os = "emscripten")]
pub(crate) type __ALIGN_BOUNDARY = usize;

#[cfg(target_os = "fuchsia")]
pub(crate) type __ALIGN_BOUNDARY = c_long;

#[cfg(target_os = "haiku")]
pub(crate) type __ALIGN_BOUNDARY = usize;

#[cfg(target_os = "l4re")]
pub(crate) type __ALIGN_BOUNDARY = usize;

#[cfg(target_os = "linux")]
pub(crate) type __ALIGN_BOUNDARY = usize;

#[cfg(target_os = "nto")]
pub(crate) type __ALIGN_BOUNDARY = usize;

#[cfg(target_os = "redox")]
pub(crate) type __ALIGN_BOUNDARY = size_t;

#[cfg(target_os = "vxworks")]
pub(crate) type __ALIGN_BOUNDARY = usize;

cfg_if! {
    if #[cfg(any(target_os = "illumos", target_os = "solaris"))] {
        #[cfg(target_arch = "sparc64")]
        pub(crate) type __ALIGN_BOUNDARY = u64;
        #[cfg(not(target_arch = "sparc64"))]
        pub(crate) type __ALIGN_BOUNDARY = u32;
    }
}

cfg_if! {
    if #[cfg(target_os = "netbsd")] {
        cfg_if! {
            if #[cfg(target_arch = "x86")] {
                pub(crate) type __ALIGN_BOUNDARY = c_int;
            } else if #[cfg(target_arch = "x86_64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_long;
            } else if #[cfg(target_arch = "aarch64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_int;
            } else if #[cfg(target_arch = "arm")] {
                pub(crate) type __ALIGN_BOUNDARY = c_longlong;
            } else if #[cfg(target_arch = "mips")] {
                pub(crate) type __ALIGN_BOUNDARY = c_longlong;
            } else if #[cfg(target_arch = "powerpc")] {
                pub(crate) type __ALIGN_BOUNDARY = c_double;
            } else if #[cfg(target_arch = "riscv64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_long;
            } else if #[cfg(target_arch = "sparc64")] {
                pub(crate) type __ALIGN_BOUNDARY = u128;
            }
        }
    }
}

cfg_if! {
    if #[cfg(target_os = "freebsd")] {
        cfg_if! {
            if #[cfg(target_arch = "x86")] {
                pub(crate) type __ALIGN_BOUNDARY = c_long;
            } else if #[cfg(target_arch = "x86_64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_long;
            } else if #[cfg(target_arch = "aarch64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_longlong;
            } else if #[cfg(target_arch = "arm")] {
                pub(crate) type __ALIGN_BOUNDARY = c_int;
            } else if #[cfg(target_arch = "powerpc")] {
                pub(crate) type __ALIGN_BOUNDARY = c_int;
            } else if #[cfg(target_arch = "powerpc64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_long;
            } else if #[cfg(target_arch = "riscv64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_longlong;
            }
        }
    }
}

cfg_if! {
    if #[cfg(target_os = "openbsd")] {
        cfg_if! {
            if #[cfg(target_arch = "x86")] {
                pub(crate) type __ALIGN_BOUNDARY = c_int;
            } else if #[cfg(target_arch = "x86_64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_long;
            } else if #[cfg(target_arch = "aarch64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_long;
            } else if #[cfg(target_arch = "arm")] {
                pub(crate) type __ALIGN_BOUNDARY = c_double;
            } else if #[cfg(target_arch = "mips64")] {
                pub(crate) type __ALIGN_BOUNDARY = u64;
            } else if #[cfg(target_arch = "powerpc")] {
                pub(crate) type __ALIGN_BOUNDARY = c_double;
            } else if #[cfg(target_arch = "powerpc64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_long;
            } else if #[cfg(target_arch = "riscv64")] {
                pub(crate) type __ALIGN_BOUNDARY = c_long;
            } else if #[cfg(target_arch = "sparc64")] {
                pub(crate) type __ALIGN_BOUNDARY = u128;
            }
        }
    }
}
