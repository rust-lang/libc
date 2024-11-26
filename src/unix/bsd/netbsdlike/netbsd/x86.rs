use crate::{c_int, c_uchar};

pub type c_long = i32;
pub type c_ulong = u32;
pub type c_char = i8;
pub type __cpu_simple_lock_nv_t = c_uchar;

pub(crate) const _ALIGNBYTES: usize = crate::mem::size_of::<c_int>() - 1;
