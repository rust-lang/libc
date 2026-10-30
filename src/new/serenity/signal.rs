//! Header: `signal.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/signal.h>
pub use super::Kernel::API::POSIX::signal::*;
pub use super::Kernel::API::POSIX::signal_numbers::*;
use crate::prelude::*;

extern "C" {
    pub fn sigaltstack(ss: *const crate::stack_t, oss: *mut crate::stack_t) -> c_int;
    pub fn sigsuspend(mask: *const crate::sigset_t) -> c_int;
    pub fn sigtimedwait(
        set: *const crate::sigset_t,
        info: *mut crate::siginfo_t,
        timeout: *const crate::timespec,
    ) -> c_int;
    pub fn sigwait(set: *const crate::sigset_t, signo: *mut c_int) -> c_int;
    pub fn sigwaitinfo(set: *const crate::sigset_t, info: *mut crate::siginfo_t) -> c_int;
}
