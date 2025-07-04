use std::os::raw::c_char;

pub type Byte = u8;

#[repr(C)]
pub struct Person {
    name: *const c_char,
    age: u8,
}

#[repr(C)]
pub union Word {
    word: u16,
    byte: [Byte; 2],
}

const A: *const c_char = c"abc".as_ptr();
const B: *const c_char = c"bac".as_ptr();
