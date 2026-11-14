//! Header: `sys/resource.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/sys/resource.h>
use crate::prelude::*;

pub const RUSAGE_SELF: c_int = 1;
pub const RUSAGE_CHILDREN: c_int = 2;

pub const RLIMIT_CORE: c_int = 1;
pub const RLIMIT_CPU: c_int = 2;
pub const RLIMIT_DATA: c_int = 3;
pub const RLIMIT_FSIZE: c_int = 4;
pub const RLIMIT_NOFILE: c_int = 5;
pub const RLIMIT_STACK: c_int = 6;
pub const RLIMIT_AS: c_int = 7;

pub type rlim_t = size_t;
pub const RLIM_INFINITY: rlim_t = !0;

extern "C" {
    pub fn getrlimit(resource: c_int, rlim: *mut crate::rlimit) -> c_int;
    pub fn setrlimit(resource: c_int, rlim: *const crate::rlimit) -> c_int;
}

pub const PRIO_PROCESS: c_int = 0;
pub const PRIO_PGRP: c_int = 1;
pub const PRIO_USER: c_int = 2;

extern "C" {
    pub fn getpriority(which: c_int, who: crate::id_t) -> c_int;
    pub fn setpriority(which: c_int, who: crate::id_t, prio: c_int) -> c_int;
}
