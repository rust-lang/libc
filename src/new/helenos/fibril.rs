//! HelenOS fibrils API
//!
//! * Header file: <https://github.com/HelenOS/helenos/tree/master/uspace/lib/c/include/fibril.h>

pub use crate::time::*;
use crate::{
    c_void,
    errno_t,
    size_t,
};

pub type fid_t = *mut fibril_t;

s! {
    pub struct fibril_owner_info_t {
        pub owned_by: *mut fibril_t,
    }
}

extern_ty! {
    pub enum fibril_t {}
}

extern "C" {
    pub fn fibril_create_generic(
        func: extern "C" fn(*mut c_void) -> errno_t,
        arg: *mut c_void,
        stacksize: size_t,
    ) -> fid_t;
    pub fn fibril_start(f: fid_t);
    pub fn fibril_exit(retval: c_long) -> !;
    pub fn fibril_detach(f: fid_t);
    pub fn fibril_yield();
    pub fn fibril_usleep(usec: usec_t);
    pub fn fibril_get_id() -> fid_t;
}
