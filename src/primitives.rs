//! This module contains type aliases for C's platform-specific types
//! and fixed-width integer types.
//!
//! The platform-specific types definitions were taken from rust-lang/rust in
//! library/core/src/ffi/primitives.rs
//!
//! The fixed-width integer aliases are deprecated: use the Rust types instead.

// FIXME(1.0): Deprecate these aliases in a few releases, remove in 1.0.
pub use core::ffi::{
    c_char,
    c_double,
    c_float,
    c_int,
    c_long,
    c_longlong,
    c_schar,
    c_short,
    c_uchar,
    c_uint,
    c_ulong,
    c_ulonglong,
    c_ushort,
};

#[deprecated(since = "0.2.55", note = "Use i8 instead.")]
pub type int8_t = i8;
#[deprecated(since = "0.2.55", note = "Use i16 instead.")]
pub type int16_t = i16;
#[deprecated(since = "0.2.55", note = "Use i32 instead.")]
pub type int32_t = i32;
#[deprecated(since = "0.2.55", note = "Use i64 instead.")]
pub type int64_t = i64;
#[deprecated(since = "0.2.55", note = "Use u8 instead.")]
pub type uint8_t = u8;
#[deprecated(since = "0.2.55", note = "Use u16 instead.")]
pub type uint16_t = u16;
#[deprecated(since = "0.2.55", note = "Use u32 instead.")]
pub type uint32_t = u32;
#[deprecated(since = "0.2.55", note = "Use u64 instead.")]
pub type uint64_t = u64;

cfg_if! {
    if #[cfg(all(
        target_arch = "aarch64",
        not(any(target_os = "windows", target_vendor = "apple"))
    ))] {
        /// C `__int128` (a GCC extension that's part of many ABIs)
        pub type __int128 = i128;
        /// C `unsigned __int128` (a GCC extension that's part of many ABIs)
        pub type __uint128 = u128;
        /// C __int128_t (alternate name for [__int128][])
        pub type __int128_t = i128;
        /// C __uint128_t (alternate name for [__uint128][])
        pub type __uint128_t = u128;
    } else if #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))] {
        /// C `__int128_t`
        pub type __int128_t = i128;
        /// C `__uint128_t`
        pub type __uint128_t = u128;
    }
}
