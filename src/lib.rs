//! Type aliases to C types like c_int for use with bindgen

#![allow(non_camel_case_types)]
#![deny(warnings)]
#![no_std]

// AD = Architecture dependent
pub use ad::*;
// PWD = Pointer Width Dependent
pub use pwd::*;

#[cfg(any(target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "asmjs",
          target_arch = "powerpc",
          target_arch = "powerpc64",
          target_arch = "s390x"))]
mod ad {
    pub type c_char = ::c_uchar;
}

#[cfg(any(target_arch = "mips",
          target_arch = "mips64",
          target_arch = "sparc64",
          target_arch = "x86",
          target_arch = "x86_64"))]
mod ad {
    pub type c_char = ::c_schar;
}

#[cfg(target_pointer_width = "32")]
mod pwd {}

#[cfg(target_pointer_width = "64")]
mod pwd {}

pub type c_int = i32;
pub type c_long = i32;
pub type c_longlong = i64;
pub type c_schar = i8;
pub type c_short = i16;
pub type c_uchar = u8;
pub type c_uint = u32;
pub type c_ulong = u32;
pub type c_ulonglong = u64;
pub type c_ushort = u16;

// NOTE from libc v0.2.23
// Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help enable
// more optimization opportunities around it recognizing things like
// malloc/free.
#[repr(u8)]
pub enum c_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}
