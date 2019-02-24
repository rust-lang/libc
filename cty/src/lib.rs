//! Type aliases for C FFI
#![cfg_attr(not(feature = "rustc-dep-of-std"), deny(warnings))]
#![allow(non_camel_case_types)]
#![cfg_attr(
    feature = "rustc-dep-of-std",
    feature(no_core)
)]
#![no_std]
#![cfg_attr(feature = "rustc-dep-of-std", no_core)]

#[macro_use]
mod macros;

/// Architecture independent
mod ai {
    pub type int8_t = i8;
    pub type int16_t = i16;
    pub type int32_t = i32;
    pub type int64_t = i64;

    pub type uint8_t = u8;
    pub type uint16_t = u16;
    pub type uint32_t = u32;
    pub type uint64_t = u64;

    pub type c_schar = i8;
    pub type c_short = i16;
    pub type c_longlong = i64;

    pub type c_uchar = u8;
    pub type c_ushort = u16;
    pub type c_ulonglong = u64;

    pub type c_float = f32;
    pub type c_double = f64;

    pub type intmax_t = i64;
    pub type uintmax_t = u64;

    pub type size_t = usize;
    pub type ptrdiff_t = isize;
    pub type intptr_t = isize;
    pub type uintptr_t = usize;

    cfg_if! {
        if #[cfg(libc_core_cvoid)] {
            pub use core::ffi::c_void;
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
}

/// Architecture dependent
mod ad {
    cfg_if! {
        if #[cfg(any(target_arch = "aarch64",
                     target_arch = "arm",
                     target_arch = "asmjs",
                     target_arch = "wasm32",
                     target_arch = "wasm64",
                     target_arch = "powerpc",
                     target_arch = "powerpc64",
                     target_arch = "s390x"))] {
            cfg_if! {
                if #[cfg(not(any(target_os = "apple", target_os = "ios")))] {
                    pub type c_char = ::c_uchar;

                    cfg_if! {
                        if #[cfg(any(
                            target_arch = "powerpc",
                            target_arch = "powerpc64",
                            target_arch = "s390x",
                        ))] {
                            // FIXME: ??
                            pub type wchar_t = i32;
                        } else {
                            pub type wchar_t = u32;
                        }
                    }
                } else {
                    // Apple and iOS:
                    pub type c_char = ::c_schar;
                    pub type wchar_t = i32;
                }
            }

            pub type c_int = i32;
            pub type c_uint = u32;
        } else if #[cfg(any(target_arch = "mips",
                            target_arch = "mips64",
                            target_arch = "sparc64",
                            target_arch = "x86",
                            target_arch = "x86_64",
                            target_arch = "nvptx",
                            target_arch = "nvptx64"))] {
            pub type c_char = ::c_schar;
            #[cfg(not(windows))]
            pub type wchar_t = i32;

            pub type c_int = i32;
            pub type c_uint = u32;
        } else if #[cfg(target_arch = "msp430")] {
            pub type c_char = ::c_uchar;
            #[cfg(not(windows))]
            pub type wchar_t = u16;

            pub type c_int = i16;
            pub type c_uint = u16;
        } else {
            // unknown architecture => empty
        }
    }
}

/// Operating system / environment dependent types
cfg_if! {
    if #[cfg(windows)] {
        pub type c_long = i32;
        pub type c_ulong = u32;
        pub type wchar_t = u16;

        pub use self::ai::*;
        pub use self::ad::*;
    } else if #[cfg(target_os = "emscripten")] {
        pub type c_long = i32;
        pub type c_ulong = u32;
        pub type wchar_t = i32;

        pub use self::ai::*;
        pub use self::ad::*;
    } else if #[cfg(any(
        target_os = "redox",
        target_os = "fuchsia",
        target_os = "solaris",
        target_os = "switch",
        target_env = "sgx",
        target_os = "dragonfly",
        target_os = "hermit",
        target_os = "illumos"
    ))] {
        pub type c_long = i64;
        pub type c_ulong = u64;
        pub type ssize_t = isize;

        pub use self::ai::*;
        pub use self::ad::*;
    } else if #[cfg(any(
        target_os = "android",
        target_os = "linux",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "cloudabi",
        target_os = "openbsd",
        target_os = "bitrig",
        target_os = "netbsd",
        target_os = "haiku",
        target_env = "uclib",
        target_env = "newlib",
        feature = "generic"
    ))]{
        #[cfg(any(target_pointer_width = "16",
                  target_pointer_width = "32"))]
        pub type c_long = i32;
        #[cfg(any(target_pointer_width = "16",
                  target_pointer_width = "32"))]
        pub type c_ulong = u32;

        #[cfg(target_pointer_width = "64")]
        pub type c_long = i64;
        #[cfg(target_pointer_width = "64")]
        pub type c_ulong = u64;

        pub type ssize_t = isize;

        pub use self::ai::*;
        pub use self::ad::*;
    } else {
        // unknown OS / environment => empty
    }
}
