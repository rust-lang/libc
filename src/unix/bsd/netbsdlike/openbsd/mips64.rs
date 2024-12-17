pub use crate::arch::c_char_def as c_char;

pub type c_long = i64;
pub type c_ulong = u64;

#[doc(hidden)]
pub const _ALIGNBYTES: usize = 7;

pub const _MAX_PAGE_SHIFT: u32 = 14;
