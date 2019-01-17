use dox::mem;

use PT_FIRSTMACH;

pub type c_long = i32;
pub type c_ulong = u32;
pub type c_char = u8;
pub type __cpu_simple_lock_nv_t = ::c_int;

// should be pub(crate), but that requires Rust 1.18.0
#[doc(hidden)]
pub const _ALIGNBYTES: usize = mem::size_of::<::c_longlong>() - 1;

pub const PT_GETREGS: ::c_int = PT_FIRSTMACH + 1;
pub const PT_SETREGS: ::c_int = PT_FIRSTMACH + 2;
pub const PT_GETFPREGS: ::c_int = PT_FIRSTMACH + 3;
pub const PT_SETFPREGS: ::c_int = PT_FIRSTMACH + 4;
