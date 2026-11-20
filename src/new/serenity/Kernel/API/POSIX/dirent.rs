//! File `Kernel/API/POSIX/dirent.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/dirent.h>
use crate::prelude::*;

pub const DT_WHT: c_int = 14;

pub const MAXNAMLEN: usize = crate::NAME_MAX;
