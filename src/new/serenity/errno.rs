//! Header: `errno.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/errno.h>
pub use super::Kernel::API::POSIX::errno::*;
use crate::prelude::*;

extern "C" {
    pub fn __errno_location() -> *mut c_int;
}
