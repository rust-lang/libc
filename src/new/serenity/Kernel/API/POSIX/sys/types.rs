//! File `Kernel/API/POSIX/sys/types.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/sys/types.h>
use crate::prelude::*;

pub type u_char = c_uchar;
pub type u_short = c_ushort;
pub type u_int = c_int;
pub type u_long = c_long;

pub type caddr_t = *mut c_char;

pub type id_t = i32;

pub type ino_t = u64;
pub type off_t = i64;

pub type blkcnt_t = u64;
pub type blksize_t = u64;
pub type dev_t = u64;
pub type mode_t = u16;
pub type nlink_t = u64;

pub type time_t = i64;
pub type useconds_t = u32;
pub type suseconds_t = i64;
pub type clock_t = u64;

pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;

pub type socklen_t = u32;

pub type pthread_t = i32;
pub type pthread_key_t = i32;
pub type pthread_once_t = u32;

s! {
    pub struct pthread_mutex_t {
        pub lock: u32,
        pub owner: pthread_t,
        pub level: c_int,
        pub type_: c_int,
    }
}

pub type pthread_attr_t = *mut c_void;

s! {
    pub struct pthread_mutexattr_t {
        pub type_: c_int,
    }

    pub struct pthread_cond_t {
        pub mutex: *mut pthread_mutex_t,
        pub value: u32,
        pub clockid: c_int,
    }
}

pub type pthread_rwlock_t = u64;
pub type pthread_rwlockattr_t = *mut c_void;

s! {
    pub struct pthread_spinlock_t {
        pub m_lock: c_int,
    }

    pub struct pthread_condattr_t {
        pub clockid: c_int,
    }
}
