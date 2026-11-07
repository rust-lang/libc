//! Header: `sys/ioctl.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/sys/ioctl.h>
pub use super::super::Kernel::API::Ioctl::*;
use crate::prelude::*;

extern "C" {
    pub fn ioctl(fd: c_int, request: c_uint, ...) -> c_int;
}
