//! Header: `sched.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/sched.h>

pub use super::Kernel::API::POSIX::sched::*;
use crate::prelude::*;

pub const SCHED_FIFO: c_int = 0;
pub const SCHED_RR: c_int = 1;
pub const SCHED_OTHER: c_int = 2;
pub const SCHED_BATCH: c_int = 3;

extern "C" {
    pub fn sched_get_priority_min(policy: c_int) -> c_int;
    pub fn sched_get_priority_max(policy: c_int) -> c_int;
    pub fn sched_setparam(pid: crate::pid_t, param: *const crate::sched_param) -> c_int;
    pub fn sched_getparam(pid: crate::pid_t, param: *mut crate::sched_param) -> c_int;
}
