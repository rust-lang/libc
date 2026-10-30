//! Header: `poll.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/poll.h>
pub use super::Kernel::API::POSIX::poll::*;
use crate::prelude::*;

extern "C" {
    pub fn ppoll(
        fds: *mut crate::pollfd,
        nfds: crate::nfds_t,
        ts: *const crate::timespec,
        sigmask: *const crate::sigset_t,
    ) -> c_int;
}
