#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_int};

pub type T2Foo = u32;
pub type T2Bar = u32;

pub type T2TypedefFoo = T2Foo;
pub type T2TypedefInt = c_int;

macro_rules! i {
    ($i:item) => {
        $i
    };
}

#[repr(C)]
#[derive(Debug)]
pub struct T2Baz {
    pub a: i64,
    pub b: u32,
}

#[repr(C)]
pub union T2Union {
    pub a: u32,
    pub b: i64,
}

pub const T2C: i32 = 5;

i! {
    pub const T2S: *const c_char = b"b\0".as_ptr().cast();
}

extern "C" {
    pub fn T2a();
}

#[cfg(target_env = "msvc")]
pub type enum_repr_too_small = i16;
#[cfg(not(target_env = "msvc"))]
pub type enum_repr_too_small = u16;
pub const ENUM_REPR_TOO_SMALL_A: enum_repr_too_small = 0;

#[cfg(target_env = "msvc")]
pub type enum_wrong_signedness = u32;
#[cfg(not(target_env = "msvc"))]
pub type enum_wrong_signedness = i32;
pub const ENUM_WRONG_SIGNEDNESS_A: enum_wrong_signedness = 0;
