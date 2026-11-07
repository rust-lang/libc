//! Header: `semaphore.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/semaphore.h>
use crate::prelude::*;

pub const SEM_FLAG_PROCESS_SHARED: c_int = 1;
pub const SEM_FLAG_NAMED: c_int = 2;

s! {
    pub struct sem_t {
        pub magic: u32,
        pub value: u32,
        pub flags: u8,
    }
}

extern "C" {
    pub fn sem_close(sem: *mut sem_t) -> c_int;
    pub fn sem_destroy(sem: *mut sem_t) -> c_int;
    pub fn sem_getvalue(sem: *mut sem_t, sval: *mut c_int) -> c_int;
    pub fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
    pub fn sem_open(name: *const c_char, oflag: c_int, ...) -> *mut sem_t;
    pub fn sem_unlink(name: *const c_char) -> c_int;
    pub fn sem_timedwait(sem: *mut sem_t, abstime: *const crate::timespec) -> c_int;
}

pub const SEM_FAILED: *mut sem_t = ptr::null_mut();
pub const SEM_VALUE_MAX: c_int = 2147483647;
