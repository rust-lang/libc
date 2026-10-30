//! Header: `pthread.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/pthread.h>

pub use crate::new::common::posix::pthread::{
    pthread_atfork,
    pthread_attr_getguardsize,
    pthread_attr_getschedparam,
    pthread_attr_getstack,
    pthread_attr_setguardsize,
    pthread_attr_setschedparam,
    pthread_attr_setstack,
    pthread_cancel,
    pthread_condattr_getclock,
    pthread_condattr_setclock,
    pthread_create,
    pthread_getschedparam,
    pthread_kill,
    pthread_once,
    pthread_rwlockattr_getpshared,
    pthread_rwlockattr_setpshared,
    pthread_setschedparam,
    pthread_sigmask,
    pthread_spin_destroy,
    pthread_spin_init,
    pthread_spin_lock,
    pthread_spin_trylock,
    pthread_spin_unlock,
};
use crate::prelude::*;

extern "C" {
    pub fn pthread_cleanup_push(
        routine: Option<unsafe extern "C" fn(*mut c_void)>,
        arg: *mut c_void,
    );
    pub fn pthread_cleanup_pop(execute: c_int);
}

pub const PTHREAD_CREATE_JOINABLE: c_int = 0;
pub const PTHREAD_CREATE_DETACHED: c_int = 1;
pub const PTHREAD_CANCELED: *mut c_void = usize::MAX as *mut c_void;

pub const PTHREAD_SCOPE_SYSTEM: c_int = 0;
pub const PTHREAD_SCOPE_PROCESS: c_int = 1;

extern "C" {
    pub fn pthread_attr_getdetachstate(
        attr: *const crate::pthread_attr_t,
        state: *mut c_int,
    ) -> c_int;
    pub fn pthread_attr_getscope(attr: *const crate::pthread_attr_t, scope: *mut c_int) -> c_int;
    pub fn pthread_attr_setscope(attr: *mut crate::pthread_attr_t, scope: c_int) -> c_int;
}

pub const PTHREAD_ONCE_INIT: c_int = 0;

// From bits/pthread_integration.h
pub const PTHREAD_MUTEX_NORMAL: c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: c_int = 1;
pub const PTHREAD_MUTEX_INITIALIZER: crate::pthread_mutex_t = crate::pthread_mutex_t {
    lock: 0,
    owner: 0,
    level: 0,
    type_: PTHREAD_MUTEX_NORMAL,
};
pub const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t = crate::pthread_mutex_t {
    lock: 0,
    owner: 0,
    level: 0,
    type_: PTHREAD_MUTEX_RECURSIVE,
};

pub const PTHREAD_PROCESS_PRIVATE: c_int = 0;
pub const PTHREAD_PROCESS_SHARED: c_int = 1;

pub const PTHREAD_COND_INITIALIZER: crate::pthread_cond_t = crate::pthread_cond_t {
    mutex: ptr::null_mut(),
    value: 0,
    clockid: crate::CLOCK_REALTIME_COARSE,
};

pub const PTHREAD_RWLOCK_INITIALIZER: crate::pthread_rwlock_t = 0;

pub const PTHREAD_KEYS_MAX: c_int = 64;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: c_int = 4;

pub const PTHREAD_CANCEL_ENABLE: c_int = 1;
pub const PTHREAD_CANCEL_DISABLE: c_int = 2;

pub const PTHREAD_CANCEL_DEFERRED: c_int = 1;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: c_int = 2;

extern "C" {
    pub fn pthread_setcancelstate(state: c_int, oldstate: *mut c_int) -> c_int;
    pub fn pthread_setcanceltype(type_: c_int, oldtype: *mut c_int) -> c_int;
    pub fn pthread_testcancel();

    pub fn pthread_mutexattr_gettype(
        attr: *const crate::pthread_mutexattr_t,
        type_: *mut c_int,
    ) -> c_int;

    pub fn pthread_setname_np(thread: crate::pthread_t, name: *const c_char) -> c_int;
    pub fn pthread_getname_np(thread: crate::pthread_t, name: *mut c_char, len: usize) -> c_int;

    pub fn pthread_rwlock_timedrdlock(
        rwlock: *mut crate::pthread_rwlock_t,
        abstime: *const crate::timespec,
    ) -> c_int;
    pub fn pthread_rwlock_timedwrlock(
        rwlock: *mut crate::pthread_rwlock_t,
        abstime: *const crate::timespec,
    ) -> c_int;
}
