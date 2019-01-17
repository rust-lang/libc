use dox::mem;

pub type c_long = i64;
pub type c_ulong = u64;
pub type c_char = u8;

// should be pub(crate), but that requires Rust 1.18.0
#[doc(hidden)]
pub const _ALIGNBYTES: usize = mem::size_of::<::c_long>() - 1;
