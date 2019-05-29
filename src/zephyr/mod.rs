//! Definitions found commonly among almost all Unix derivatives
//!
//! More functions and definitions can be found in the more specific modules
//! according to the platform in question.

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[cfg(target_arch = "aarch64")]
pub type c_char = u8;
#[cfg(not(target_arch = "aarch64"))]
pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
#[cfg(target_pointer_width = "32")]
pub type c_long = i32;
#[cfg(target_pointer_width = "32")]
pub type c_ulong = u32;
#[cfg(target_pointer_width = "64")]
pub type c_long = i64;
#[cfg(target_pointer_width = "64")]
pub type c_ulong = u64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type c_float = f32;
pub type c_double = f64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

cfg_if! {
    if #[cfg(libc_core_cvoid)] {
        pub use ::ffi::c_void;
    } else {
        // Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help
        // enable more optimization opportunities around it recognizing things
        // like malloc/free.
        #[repr(u8)]
        #[allow(missing_copy_implementations)]
        #[allow(missing_debug_implementations)]
        pub enum c_void {
            // Two dummy variants so the #[repr] attribute can be used.
            #[doc(hidden)]
            __variant1,
            #[doc(hidden)]
            __variant2,
        }
    }
}

extern {
    pub fn strlen(cs: *const c_char) -> size_t;

    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(dest: *mut c_void, src: *const c_void,
                  n: size_t) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void,
                   n: size_t) -> *mut c_void;
    pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
}
