//! File `Kernel/API/POSIX/poll.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/poll.h>
use crate::prelude::*;

pub const POLLIN: c_short = 1 << 0;
pub const POLLRDNORM: c_short = POLLIN;
pub const POLLPRI: c_short = 1 << 1;
pub const POLLOUT: c_short = 1 << 2;
pub const POLLWRNORM: c_short = POLLOUT;
pub const POLLERR: c_short = 1 << 3;
pub const POLLHUP: c_short = 1 << 4;
pub const POLLNVAL: c_short = 1 << 5;
pub const POLLWRBAND: c_short = 1 << 12;
pub const POLLRDHUP: c_short = 1 << 13;

pub type nfds_t = c_uint;
