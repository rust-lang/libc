pub use crate::arch::c_char_def as c_char;
use crate::prelude::*;

pub type c_long = i32;
pub type c_ulong = u32;
pub type __cpu_simple_lock_nv_t = c_uchar;

pub(crate) const _ALIGNBYTES: usize = mem::size_of::<c_int>() - 1;
