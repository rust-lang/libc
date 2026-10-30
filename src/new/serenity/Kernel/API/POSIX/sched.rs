//! File `Kernel/API/POSIX/sched.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/sched.h>
use crate::prelude::*;

s! {
    pub struct sched_param {
        pub sched_priority: c_int,
    }
}

pub const THREAD_PRIORITY_MIN: c_int = 1;
pub const THREAD_PRIORITY_LOW: c_int = 10;
pub const THREAD_PRIORITY_NORMAL: c_int = 30;
pub const THREAD_PRIORITY_HIGH: c_int = 50;
pub const THREAD_PRIORITY_MAX: c_int = 99;
