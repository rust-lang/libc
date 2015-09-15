//! 64-bit specific definitions for linux-like values

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
pub type size_t = u64;
pub type ptrdiff_t = i64;
pub type clock_t = i64;
pub type time_t = i64;
pub type suseconds_t = i64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intptr_t = i64;
pub type uintptr_t = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type off_t = i64;
pub type dev_t = u64;
pub type ino_t = u64;
pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type useconds_t = u32;
pub type mode_t = u32;
pub type ssize_t = i64;
pub type blkcnt_t = i64;

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else {
        mod x86_64;
        pub use self::x86_64::*;
    }
}
