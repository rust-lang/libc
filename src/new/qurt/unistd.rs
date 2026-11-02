//! Header: `unistd.h`

use super::*;
pub use crate::new::common::posix::unistd::{
    STDERR_FILENO,
    STDIN_FILENO,
    STDOUT_FILENO,
};
use crate::prelude::*;

// Access mode constants
pub const F_OK: c_int = 0;
pub const X_OK: c_int = 1;
pub const W_OK: c_int = 2;
pub const R_OK: c_int = 4;

// Whence constants for lseek
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

// pathconf constants (from bits/confname.h)
pub const _PC_LINK_MAX: c_int = 0;
pub const _PC_MAX_CANON: c_int = 1;
pub const _PC_MAX_INPUT: c_int = 2;
pub const _PC_NAME_MAX: c_int = 3;
pub const _PC_PATH_MAX: c_int = 4;
pub const _PC_PIPE_BUF: c_int = 5;
pub const _PC_CHOWN_RESTRICTED: c_int = 6;
pub const _PC_NO_TRUNC: c_int = 7;
pub const _PC_VDISABLE: c_int = 8;
pub const _PC_SYNC_IO: c_int = 9;
pub const _PC_ASYNC_IO: c_int = 10;
pub const _PC_PRIO_IO: c_int = 11;
pub const _PC_SOCK_MAXBUF: c_int = 12;
pub const _PC_FILESIZEBITS: c_int = 13;
pub const _PC_REC_INCR_XFER_SIZE: c_int = 14;
pub const _PC_REC_MAX_XFER_SIZE: c_int = 15;
pub const _PC_REC_MIN_XFER_SIZE: c_int = 16;
pub const _PC_REC_XFER_ALIGN: c_int = 17;
pub const _PC_ALLOC_SIZE_MIN: c_int = 18;
pub const _PC_SYMLINK_MAX: c_int = 19;
pub const _PC_2_SYMLINKS: c_int = 20;

// sysconf constants (from hexagon/include/sys/unistd.h)
pub const _SC_NPROCESSORS_ONLN: c_int = 1;


extern "C" {
    // File operations
    pub fn access(pathname: *const c_char, mode: c_int) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;
    pub fn unlink(pathname: *const c_char) -> c_int;

    // Directory operations
    pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    pub fn rmdir(pathname: *const c_char) -> c_int;

    // Process operations
    pub fn getpid() -> pid_t;
    pub fn getppid() -> pid_t;
    pub fn getpgid(pid: pid_t) -> pid_t;
    pub fn getpgrp() -> pid_t;
    pub fn getuid() -> uid_t;
    pub fn geteuid() -> uid_t;
    pub fn getgid() -> gid_t;
    pub fn getegid() -> gid_t;
    pub fn seteuid(uid: uid_t) -> c_int;
    pub fn setuid(uid: uid_t) -> c_int;
    pub fn setpgid(pid: pid_t, pgid: pid_t) -> c_int;
    pub fn setpgrp() -> pid_t;
    pub fn setsid() -> pid_t;

    // Sleep functions
    pub fn sleep(seconds: c_uint) -> c_uint;

    // Sync operations
    pub fn fsync(fd: c_int) -> c_int;

    // System configuration
    pub fn sysconf(name: c_int) -> c_long;
    pub fn pathconf(path: *const c_char, name: c_int) -> c_long;

    // Terminal functions
    pub fn isatty(fd: c_int) -> c_int;
}
