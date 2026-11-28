//! Header: `fd_set.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/fd_set.h>
use crate::prelude::*;

s! {
    pub struct fd_set {
        pub fds_bits: [c_uchar; crate::FD_SETSIZE / 8],
    }
}

f! {
    pub unsafe fn FD_CLR(fd: c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = size_of_val(&(*set).fds_bits[0]) * 8;
        let Some(slot) = (*set).fds_bits.get_mut(fd / size) else {
            panic!(
                "fd {fd} out of range: valid fds are 0..FD_SETSIZE (0..{})",
                crate::FD_SETSIZE
            );
        };
        *slot &= !(1 << (fd % size));
    }

    pub unsafe fn FD_ISSET(fd: c_int, set: *const fd_set) -> bool {
        let fd = fd as usize;
        let size = size_of_val(&(*set).fds_bits[0]) * 8;
        let Some(slot) = (*set).fds_bits.get(fd / size) else {
            panic!(
                "fd {fd} out of range: valid fds are 0..FD_SETSIZE (0..{})",
                crate::FD_SETSIZE
            );
        };
        (*slot & (1 << (fd % size))) != 0
    }

    pub unsafe fn FD_SET(fd: c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = size_of_val(&(*set).fds_bits[0]) * 8;
        let Some(slot) = (*set).fds_bits.get_mut(fd / size) else {
            panic!(
                "fd {fd} out of range: valid fds are 0..FD_SETSIZE (0..{})",
                crate::FD_SETSIZE
            );
        };
        *slot |= 1 << (fd % size);
    }

    pub unsafe fn FD_ZERO(set: *mut fd_set) -> () {
        (*set).fds_bits.fill(0);
    }
}
