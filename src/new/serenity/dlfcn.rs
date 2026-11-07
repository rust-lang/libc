//! Header: `dlfcn.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/dlfcn.h>
use crate::prelude::*;

pub const RTLD_DEFAULT: *mut c_void = ptr::null_mut();
pub const RTLD_LAZY: c_int = 2;
pub const RTLD_NOW: c_int = 4;
pub const RTLD_GLOBAL: c_int = 8;
pub const RTLD_LOCAL: c_int = 16;

s! {
    pub struct Dl_info {
        pub dli_fname: *const c_char,
        pub dli_fbase: *mut c_void,
        pub dli_sname: *const c_char,
        pub dli_saddr: *mut c_void,
    }
}
