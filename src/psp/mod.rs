//! PSP C type definitions
//!
//! These type declarations are not enough, as they must be ultimately resolved
//! by the linker. Crates that use these definitions must, somewhere in the
//! crate graph, include a stub provider crate such as the `psp` crate.

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
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

pub type c_char = u8;
pub type c_long = i64;
pub type c_ulong = u64;

pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;

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

mod audio;
pub use self::audio::*;

mod atrac;
pub use self::atrac::*;

mod ctrl;
pub use self::ctrl::*;

mod display;
pub use self::display::*;

mod ge;
pub use self::ge::*;

mod kernel;
pub use self::kernel::*;

mod usb;
pub use self::usb::*;

mod power;
pub use self::power::*;

mod wlan;
pub use self::wlan::*;

mod rtc;
pub use self::rtc::*;

mod io;
pub use self::io::*;

mod jpeg;
pub use self::jpeg::*;

mod umd;
pub use self::umd::*;

mod mpeg;
pub use self::mpeg::*;

mod hprm;
pub use self::hprm::*;

mod gu;
pub use self::gu::*;

mod gum;
pub use self::gum::*;

mod types;
pub use self::types::*;

mod mp3;
pub use self::mp3::*;

mod registry;
pub use self::registry::*;

mod openpsid;
pub use self::openpsid::*;

mod utility;
pub use self::utility::*;

mod net;
pub use self::net::*;
