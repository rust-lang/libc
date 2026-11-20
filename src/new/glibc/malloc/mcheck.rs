//! Header: `malloc/mcheck.h`
//!
//! <https://github.com/sailfishos-mirror/glibc/blob/master/malloc/mcheck.h>

use crate::prelude::*;

c_enum! {
    #[repr(c_int)]
    pub enum mcheck_status {
        pub MCHECK_DISABLED = -1,
        pub MCHECK_OK,
        pub MCHECK_FREE,
        pub MCHECK_HEAD,
        pub MCHECK_TAIL,
    }
}

extern "C" {
    pub fn mcheck(abortfunc: Option<unsafe extern "C" fn(mcheck_status)>) -> c_int;
    pub fn mcheck_check_all();
    pub fn mcheck_pedantic(abortfunc: Option<unsafe extern "C" fn(mcheck_status)>) -> c_int;

    pub fn mprobe(ptr: *mut c_void) -> mcheck_status;

    pub fn mtrace();
    pub fn muntrace();
}
