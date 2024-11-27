use crate::c_int;

pub type c_long = i32;
pub type c_ulong = u32;
pub type c_char = i8;

pub(crate) const _ALIGNBYTES: usize = crate::mem::size_of::<c_int>() - 1;

pub const _MAX_PAGE_SHIFT: u32 = 12;
