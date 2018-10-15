pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub type c_char = u8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;
pub type off_t = u64;

const MALLOC_HEADER : isize = 8;
const MALLOC_ALIGN : usize = 8;

use super::c_void;
use std::alloc::{self, Layout};
use std::ptr;

pub unsafe fn malloc(size: size_t) -> *mut c_void {
    let lay = Layout::from_size_align_unchecked(MALLOC_HEADER as usize + size, MALLOC_ALIGN);
    let p = alloc::alloc(lay);
    if p.is_null() {
        return ptr::null_mut();
    }
    *(p as *mut size_t) = size;
    p.offset(MALLOC_HEADER) as *mut c_void
}
pub unsafe fn free(p: *mut c_void) {
    let p = p.offset(-MALLOC_HEADER) as *mut u8;
    let size = *(p as *mut size_t);
    let lay = Layout::from_size_align_unchecked(MALLOC_HEADER as usize + size, MALLOC_ALIGN);
    alloc::dealloc(p, lay);
}
pub unsafe fn realloc(p: *mut c_void, _size: size_t) -> *mut c_void {
    let p = p.offset(-MALLOC_HEADER) as *mut u8;
    let size = *(p as *mut size_t);
    let lay = Layout::from_size_align_unchecked(MALLOC_HEADER as usize + size, MALLOC_ALIGN);
    let p = alloc::realloc(p, lay, size);
    if p.is_null() {
        return ptr::null_mut();
    }
    *(p as *mut size_t) = size;
    p.offset(MALLOC_HEADER) as *mut c_void
}

