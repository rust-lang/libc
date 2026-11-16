//! Header: `qsee_heap.h`

use crate::prelude::*;

extern "C" {
    pub fn qsee_malloc(size: size_t) -> *mut c_void;
    pub fn qsee_free(ptr: *mut c_void);
}
