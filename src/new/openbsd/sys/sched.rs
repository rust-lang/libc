//! Header: `sys/sched.h`
//!
//! <https://github.com/openbsd/src/blob/master/sys/sys/sched.h>

use crate::c_int;

pub const CP_USER: c_int = 0;
pub const CP_NICE: c_int = 1;
pub const CP_SYS: c_int = 2;
pub const CP_SPIN: c_int = 3;
pub const CP_INTR: c_int = 4;
pub const CP_IDLE: c_int = 5;
pub const CPUSTATES: c_int = 6;

s2! {
    pub struct cpustats {
        pub cs_time: [u64; CPUSTATES as usize],
        pub cs_flags: u64,
    }
}

pub const CPUSTATS_ONLINE: c_int = 0x0001;
