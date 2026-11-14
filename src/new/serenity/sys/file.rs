//! Header: `sys/file.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/sys/file.h>
use crate::prelude::*;

pub const LOCK_SH: c_int = 1 << 0;
pub const LOCK_EX: c_int = 1 << 1;
pub const LOCK_NB: c_int = 1 << 2;
pub const LOCK_UN: c_int = 1 << 3;
