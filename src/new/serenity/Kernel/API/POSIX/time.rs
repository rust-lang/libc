//! File `Kernel/API/POSIX/time.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/time.h>
use crate::prelude::*;

pub const CLOCKS_PER_SEC: crate::clock_t = 1000;

pub type clockid_t = c_int;

pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_MONOTONIC_RAW: clockid_t = 2;
pub const CLOCK_REALTIME_COARSE: clockid_t = 3;
pub const CLOCK_MONOTONIC_COARSE: clockid_t = 4;

pub const TIMER_ABSTIME: c_int = 1;
