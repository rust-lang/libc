use std::ffi::{c_char, c_void};

pub type Byte = u8;

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

unsafe extern "C" {
    pub fn calloc(num: usize, size: usize) -> *mut c_void;

    pub static byte: Byte;
}
