#![allow(dead_code)]

use libc::*;

pub type T1Foo = i32;
pub const T1S: &'static str = "foo";

pub const T1N: i32 = 5;

#[repr(C)]
pub struct T1Bar {
    pub a: i32,
    pub b: u32,
    pub c: T1Foo,
    pub d: u8,
    pub e: [i64; T1N as usize],
    pub f: [[i64; 2]; T1N as usize],
}

#[repr(C)]
pub struct T1Baz {
    pub a: u64,
    pub b: T1Bar,
}

pub const T1C: u32 = 4;

const NOT_PRESENT: u32 = 5;

extern {
    pub fn T1a();
    pub fn T1b() -> *mut c_void;
    pub fn T1c(a: *mut c_void) -> *mut c_void;
    pub fn T1d(a: c_uint) -> i32;
    pub fn T1e(a: c_uint, b: *const T1Bar);

    #[link_name = "T1f"]
    pub fn f() -> ();
}
