//! File `Kernel/API/POSIX/sys/uio.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/sys/uio.h>
use crate::prelude::*;

pub const IOV_MAX: c_int = 1024;
