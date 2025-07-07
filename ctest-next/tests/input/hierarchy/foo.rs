use crate::bar::in6_addr;
use std::os::raw::c_void;

pub const ON: bool = true;

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;

    static in6addr_any: in6_addr;
}
