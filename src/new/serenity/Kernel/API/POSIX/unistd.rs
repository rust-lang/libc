//! File `Kernel/API/POSIX/unistd.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/unistd.h>

pub use crate::new::common::posix::unistd::{
    STDERR_FILENO,
    STDIN_FILENO,
    STDOUT_FILENO,
};
use crate::prelude::*;

pub const R_OK: c_int = 4;
pub const W_OK: c_int = 2;
pub const X_OK: c_int = 1;
pub const F_OK: c_int = 0;

pub const MS_NODEV: c_int = 1;
pub const MS_NOEXEC: c_int = 2;
pub const MS_NOSUID: c_int = 4;
pub const MS_BIND: c_int = 8;
pub const MS_RDONLY: c_int = 16;
pub const MS_REMOUNT: c_int = 32;
pub const MS_WXALLOWED: c_int = 64;
pub const MS_AXALLOWED: c_int = 128;
pub const MS_NOREGULAR: c_int = 256;
pub const MS_SRCHIDDEN: c_int = 512;
pub const MS_IMMUTABLE: c_int = 1024;

pub const _SC_MONOTONIC_CLOCK: c_int = 0;
pub const _SC_NPROCESSORS_CONF: c_int = 1;
pub const _SC_NPROCESSORS_ONLN: c_int = 2;
pub const _SC_OPEN_MAX: c_int = 3;
pub const _SC_HOST_NAME_MAX: c_int = 4;
pub const _SC_TTY_NAME_MAX: c_int = 5;
pub const _SC_PAGESIZE: c_int = 6;
pub const _SC_GETPW_R_SIZE_MAX: c_int = 7;
pub const _SC_GETGR_R_SIZE_MAX: c_int = 8;
pub const _SC_CLK_TICK: c_int = 9;
pub const _SC_SYMLOOP_MAX: c_int = 10;
pub const _SC_MAPPED_FILES: c_int = 11;
pub const _SC_ARG_MAX: c_int = 12;
pub const _SC_IOV_MAX: c_int = 13;
pub const _SC_PHYS_PAGES: c_int = 14;
