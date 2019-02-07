//! This module attempts to attempts to give reasonable definitions for most
//! basic C types. Note that these are essentially educated guesses and are NOT
//! guaranteed to match the types produced by your C compiler.

// Per the C11 specification, these type definitions are not guaranteed to be
// correct for all platforms. However, virtually all platforms use these
// definitions, including all targets supported by rustc.
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type ssize_t = isize;

// There are two ways that platforms (in practice) differ in their C types:
//   - chars can either be signed or unsigned
//   - longs can either be 32-bit or 64-bit

// Whether chars default to signed or unsigned is primarily determined by
// architecture (Windows is the main exception here).
cfg_if! {
    if #[cfg(any(target_arch = "aarch64",
                 target_arch = "arm",
                 target_arch = "asmjs",
                 target_arch = "wasm32",
                 target_arch = "wasm64",
                 target_arch = "powerpc",
                 target_arch = "powerpc64",
                 target_arch = "s390x"))] {
        pub type c_char = u8;
        pub type wchar_t = u32;
    } else if #[cfg(any(target_arch = "x86",
                        target_arch = "x86_64",
                        target_arch = "mips",
                        target_arch = "mips64",
                        target_arch = "msp430",
                        target_arch = "nvptx",
                        target_arch = "nvptx64",
                        target_arch = "sparc",
                        target_arch = "sparc64",
                        target_arch = "riscv32",
                        target_arch = "riscv64"))] {
        pub type c_char = i8;
        pub type wchar_t = i32;
    }
}

// On all platforms but Windows, longs are the same size as a pointer.
cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        pub type c_long = i64;
        pub type c_ulong = u64;
    } else {
        pub type c_long = i32;
        pub type c_ulong = u32;
    }
}

// POSIX specifications make no guarantees that off_t == long int, but this is
// what GNU and others always do.
pub type off_t = ::c_long;

// These follow directly from the definition of c_int
pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;