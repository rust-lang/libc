//! File `Kernel/API/POSIX/netinet/tcp.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/netinet/tcp.h>
use crate::prelude::*;

pub const TCP_NODELAY: c_int = 10;
pub const TCP_MAXSEG: c_int = 11;
