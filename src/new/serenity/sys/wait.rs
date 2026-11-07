//! Header: `sys/wait.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/sys/wait.h>
pub use super::super::Kernel::API::POSIX::sys::wait::*;
use crate::prelude::*;

extern "C" {
    pub fn waitid(
        idtype: crate::idtype_t,
        id: crate::id_t,
        infop: *mut crate::siginfo_t,
        options: c_int,
    ) -> c_int;
}
