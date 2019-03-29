#![allow(dead_code)]

use libc::*;

pub type T1Foo = i32;
pub const T1S: &'static str = "foo";

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

pub type T1TypedefDouble = c_double;
pub type T1TypedefPtr = *mut c_int;
pub type T1TypedefStruct = T1Bar;

i! {
    pub const T1C: u32 = 4;
}

const NOT_PRESENT: u32 = 5;

pub type Arr = [i32; 4];

extern "C" {
    pub fn T1a();
    pub fn T1b() -> *mut c_void;
    pub fn T1c(a: *mut c_void) -> *mut c_void;
    pub fn T1d(a: c_uint) -> i32;
    pub fn T1e(a: c_uint, b: *const T1Bar);

    #[link_name = "T1f"]
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
    assert_eq!(1, 1);
}

extern "C" {
    pub static T1_static_u8: u8;
    pub static mut T1_static_mut_u8: u8;
    pub static mut T1_static_mut_fn_ptr: extern "C" fn(u8, u8) -> u8;
    pub static T1_static_const_fn_ptr_unsafe: unsafe extern "C" fn(u8, u8) -> u8;
    pub static T1_static_const_fn_ptr_unsafe2: unsafe extern "C" fn(u8) -> ();
    pub static T1_static_const_fn_ptr_unsafe3: unsafe extern "C" fn() -> ();

    #[link_name = "T1_static_right"]
    pub static T1_static_wrong: u8;
    #[link_name = "T1_static_right2"]
    pub static mut T1_static_wrong2: extern "C" fn(u8, u8) -> u8;

    pub static T1_fn_ptr_s: unsafe extern "C" fn(u8) -> extern "C" fn(u16) -> u32;
    pub static T1_fn_ptr_s2: unsafe extern "C" fn(
        extern "C" fn(u8) -> u8,
        extern "C" fn(u16) -> u16,
    ) -> extern "C" fn(u16) -> u32;

    pub static T1_arr0: [i32; 2];
    pub static T1_arr1: [[i32; 3]; 2];
    pub static T1_arr2: [[[i32; 3]; 2]; 1];

    pub static mut T1_arr3: [i32; 2];
    pub static mut T1_arr4: [[i32; 3]; 2];
    pub static mut T1_arr5: [[[i32; 3]; 2]; 1];

    #[link_name = "T1_arr42"]
    pub static mut T1_arr6: [[[i32; 3]; 2]; 1];

    pub static mut T1_sref: &'static i16;

    pub static mut T1_mut_opt_ref: Option<&'static i32>;
    pub static mut T1_mut_opt_mut_ref: Option<&'static mut i32>;
    pub static T1_const_opt_const_ref: Option<&'static i32>;

    pub static T1_opt_fn1: Option<unsafe extern "C" fn() -> ()>;
    pub static T1_opt_fn2: Option<unsafe extern "C" fn(u8) -> extern "C" fn(u16) -> u32>;
    pub static T1_opt_fn3: Option<
        unsafe extern "C" fn(
            extern "C" fn(u8) -> u8,
            extern "C" fn(u16) -> u16,
        ) -> extern "C" fn(u16) -> u32,
    >;
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

#[repr(C)]
pub struct V {
    pub v: *mut u8,
}

extern "C" {
    pub static mut vol_ptr: *mut u8;
    pub fn T1_vol0(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
    pub fn T1_vol1(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
    pub fn T1_vol2(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
    pub static T1_fn_ptr_vol: Option<unsafe extern "C" fn(u8, u8) -> u8>;
}
