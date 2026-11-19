//! Header: `sys/mman.h`
//!
//! <https://github.com/NetBSD/src/tree/trunk/sys/sys/mman.h>

use crate::prelude::*;

pub const MFD_CLOEXEC: c_uint = 0x1;
pub const MFD_ALLOW_SEALING: c_uint = 0x2;
pub const MFD_CLOFORK: c_uint = 0x4;

extern "C" {
    pub fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
}
