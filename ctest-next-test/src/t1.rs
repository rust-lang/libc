#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_void};

pub type T1Foo = i32;
pub const T1S: *const c_char = c"foo".as_ptr();

pub const T1N: i32 = 5;

macro_rules! i {
    ($i:item) => {
        $i
    };
}

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

#[repr(C)]
pub union T1Union {
    pub a: u64,
    pub b: u32,
}

#[repr(C)]
pub union T1NoTypedefUnion {
    pub a: u64,
    pub b: u32,
}

#[repr(C)]
pub struct T1StructWithUnion {
    pub u: T1NoTypedefUnion,
}

#[repr(transparent)]
pub struct Transparent(i32);

pub type T1TypedefDouble = c_double;
pub type T1TypedefPtr = *mut c_int;
pub type T1TypedefStruct = T1Bar;

i! {
    pub const T1C: u32 = 4;
}

#[expect(unused)]
const NOT_PRESENT: u32 = 5;

pub type Arr = [i32; 4];

extern "C" {
    pub fn T1a();
    pub fn T1b() -> *mut c_void;
    pub fn T1c(a: *mut c_void) -> *mut c_void;
    pub fn T1d(a: c_uint) -> i32;
    pub fn T1e(a: c_uint, b: *const T1Bar);

    #[link_name = "T1f"]
    #[allow(clippy::unused_unit)]
    pub fn f() -> ();

    pub fn T1g(a: *mut [i32; 4]);
    pub fn T1h(a: *const [i32; 4]) -> !;
    pub fn T1i(a: *mut [i32; 4]);
    pub fn T1j(a: *const [i32; 4]) -> !;
    pub fn T1o(a: *mut *mut [i32; 4]);
    pub fn T1p(a: *const *const [i32; 4]) -> !;

    pub fn T1r(a: *mut Arr);
    pub fn T1s(a: *const Arr) -> !;
    pub fn T1t(a: *mut *mut Arr);
    pub fn T1v(a: *const *const Arr) -> !;

    pub static T1static: c_uint;
}

pub fn foo() {
    let x = 1;
    assert_eq!(x, 1);
}

extern "C" {
    pub static T1_static_u8: u8;
    /* FIXME(#4365): duplicate symbol errors when enabled
    // pub static mut T1_static_mut_u8: u8;
    // pub static mut T1_static_mut_fn_ptr: extern "C" fn(u8, u8) -> u8;
    pub static T1_static_const_fn_ptr_unsafe: unsafe extern "C" fn(u8, u8) -> u8;
     */
    pub static T1_static_const_fn_ptr_unsafe2: unsafe extern "C" fn(u8) -> ();
    pub static T1_static_const_fn_ptr_unsafe3: unsafe extern "C" fn() -> ();

    #[link_name = "T1_static_right"]
    pub static T1_static_wrong: u8;
    /* FIXME(#4365): duplicate symbol errors when enabled
    // #[link_name = "T1_static_right2"]
    // pub static mut T1_static_wrong2: extern "C" fn(u8, u8) -> u8;

    pub static T1_fn_ptr_s: unsafe extern "C" fn(u8) -> extern "C" fn(u16) -> u32;
    pub static T1_fn_ptr_s2: unsafe extern "C" fn(
        extern "C" fn(u8) -> u8,
        extern "C" fn(u16) -> u16,
    ) -> extern "C" fn(u16) -> u32;
    */

    pub static T1_arr0: [i32; 2];
    pub static T1_arr1: [[i32; 3]; 2];
    pub static T1_arr2: [[[i32; 3]; 2]; 1];

    pub static mut T1_arr3: [i32; 2];
    pub static mut T1_arr4: [[i32; 3]; 2];
    pub static mut T1_arr5: [[[i32; 3]; 2]; 1];

    #[link_name = "T1_arr42"]
    pub static mut T1_arr6: [[[i32; 3]; 2]; 1];

    pub static mut T1_sref: &'static i16;
}

#[repr(C)]
pub struct Q {
    pub q0: *mut u8,
    pub q1: *mut *mut u8,
    pub q2: u8,
}

#[repr(C)]
pub struct T1_conflict_foo {
    a: i32,
}

#[repr(C)]
pub struct T1_conflict {
    pub foo: i32,
}

#[repr(C, packed)]
pub struct Pack {
    pub a: u8,
    pub b: u16,
}

#[repr(C, packed(4))]
pub struct Pack4 {
    pub a: u8,
    pub b: u32,
}

#[repr(C)]
pub struct V {
    pub v: *mut u8,
}

extern "C" {
    pub static mut vol_ptr: *mut u8;
    pub fn T1_vol0(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
    pub fn T1_vol1(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
    pub fn T1_vol2(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
}

pub const LOG_MAX_LINE_LENGTH: usize = 1400;

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_int,
}

#[expect(unused)]
#[repr(C)]
struct log_record_t {
    level: c_long,
    file: *const c_char,
    line: c_long,
    module: *const c_char,
    tv: timeval,
    message: [c_char; LOG_MAX_LINE_LENGTH],
}

#[expect(unused)]
#[cfg(not(any(target_pointer_width = "16", target_pointer_width = "32")))]
#[repr(C, align(16))]
struct LongDoubleWrap {
    inner: u128,
}

#[expect(unused)]
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
#[repr(C)]
struct LongDoubleWrap {
    inner: c_double,
}
