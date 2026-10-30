//! Header: `sys/mman.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/sys/mman.h>

pub use super::super::Kernel::API::POSIX::sys::mman::*;
use crate::prelude::*;

extern "C" {
    pub fn mmap_with_name(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: crate::off_t,
        name: *const c_char,
    ) -> *mut c_void;
    pub fn serenity_mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: crate::off_t,
        alignment: usize,
        name: *const c_char,
    ) -> *mut c_void;
    pub fn mremap(
        old_address: *mut c_void,
        old_size: usize,
        new_size: usize,
        flags: c_int,
    ) -> *mut c_void;
    pub fn mprotect(addr: *mut c_void, length: usize, advice: c_int) -> c_int;
    pub fn posix_madvise(addr: *mut c_void, length: usize, advice: c_int) -> c_int;
    pub fn msync(addr: *mut c_void, length: usize, flags: c_int) -> c_int;
}
