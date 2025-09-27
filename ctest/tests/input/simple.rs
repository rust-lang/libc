use std::ffi::{c_char, c_int, c_ulong, c_void};

pub type Byte = u8;

// This should be automatically skipped for roundtripping.
pub type gregset_t = [c_ulong; 32];

#[repr(C)]
pub struct Person {
    pub name: *const c_char,
    pub age: u8,
    pub job: extern "C" fn(u8, *const c_char),
}

#[repr(C)]
pub union Word {
    pub word: u16,
    pub byte: [Byte; 2],
}

const A: *const c_char = c"abc".as_ptr();
const B: *const c_char = c"bac".as_ptr();

pub type Color = c_int;
pub const RED: Color = 0;
pub const BLUE: Color = RED + 1;
pub const GREEN: Color = BLUE + 1;

unsafe extern "C" {
    pub fn calloc(num: usize, size: usize) -> *mut c_void;

    pub static byte: Byte;
}
