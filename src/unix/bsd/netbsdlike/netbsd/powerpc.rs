use crate::prelude::*;
use crate::PT_FIRSTMACH;

pub type __cpu_simple_lock_nv_t = c_int;

pub const PT_STEP: c_int = PT_FIRSTMACH + 0;
pub const PT_GETREGS: c_int = PT_FIRSTMACH + 1;
pub const PT_SETREGS: c_int = PT_FIRSTMACH + 2;
