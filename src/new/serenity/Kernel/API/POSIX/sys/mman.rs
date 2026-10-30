//! File `Kernel/API/POSIX/sys/mman.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/sys/mman.h>
use crate::prelude::*;

pub const MAP_FILE: c_int = 0x00;
pub const MAP_SHARED: c_int = 0x01;
pub const MAP_PRIVATE: c_int = 0x02;
pub const MAP_FIXED: c_int = 0x10;
pub const MAP_ANONYMOUS: c_int = 0x20;
pub const MAP_ANON: c_int = MAP_ANONYMOUS;
pub const MAP_STACK: c_int = 0x40;
pub const MAP_NORESERVE: c_int = 0x80;
pub const MAP_RANDOMIZED: c_int = 0x100;
pub const MAP_PURGEABLE: c_int = 0x200;
pub const MAP_FIXED_NOREPLACE: c_int = 0x400;

pub const PROT_NONE: c_int = 0x00;
pub const PROT_READ: c_int = 0x01;
pub const PROT_WRITE: c_int = 0x02;
pub const PROT_EXEC: c_int = 0x04;

pub const MAP_FAILED: *mut c_void = usize::MAX as *mut c_void;

pub const MADV_NORMAL: c_int = 0x00;
pub const MADV_SET_VOLATILE: c_int = 0x01;
pub const MADV_SET_NONVOLATILE: c_int = 0x02;
pub const MADV_DONTNEED: c_int = 0x03;
pub const MADV_WILLNEED: c_int = 0x04;
pub const MADV_SEQUENTIAL: c_int = 0x05;
pub const MADV_RANDOM: c_int = 0x06;

pub const POSIX_MADV_NORMAL: c_int = MADV_NORMAL;
pub const POSIX_MADV_DONTNEED: c_int = MADV_DONTNEED;
pub const POSIX_MADV_WILLNEED: c_int = MADV_WILLNEED;
pub const POSIX_MADV_SEQUENTIAL: c_int = MADV_SEQUENTIAL;
pub const POSIX_MADV_RANDOM: c_int = MADV_RANDOM;

pub const MS_SYNC: c_int = 1;
pub const MS_ASYNC: c_int = 2;
pub const MS_INVALIDATE: c_int = 4;
