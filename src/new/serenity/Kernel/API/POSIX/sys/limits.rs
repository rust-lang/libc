//! File `Kernel/API/POSIX/sys/limits.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/sys/limits.h>
use crate::prelude::*;

pub const PAGE_SIZE: usize = 4096;
pub const PATH_MAX: usize = 4096;
pub const MAXPATHLEN: usize = PATH_MAX;
pub const NAME_MAX: usize = 255;
pub const HOST_NAME_MAX: usize = 64;
pub const TTY_NAME_MAX: usize = 32;
pub const NGROUPS_MAX: usize = 32;
pub const ARG_MAX: c_int = 65536;
pub const PTHREAD_STACK_MIN: usize = 64 * 1024;
