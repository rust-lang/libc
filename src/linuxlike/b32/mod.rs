//! 32-bit specific definitions for linux-like values

pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type size_t = u32;
pub type ptrdiff_t = i32;
pub type clock_t = i32;
pub type time_t = i32;
pub type suseconds_t = i32;
pub type wchar_t = i32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intptr_t = i32;
pub type uintptr_t = u32;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type off_t = i32;
pub type ino_t = u32;
pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type useconds_t = u32;
pub type ssize_t = i32;

cfg_if! {
    if #[cfg(target_os = "android")] {
        mod android;
        pub use self::android::*;
    } else if #[cfg(any(target_arch = "mips", target_arch = "mipsel"))] {
        mod mips;
        pub use self::mips::*;
    } else {
        mod other;
        pub use other::*;
    }
}
