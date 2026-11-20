//! File `Kernel/API/POSIX/fcntl.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/fcntl.h>
use crate::prelude::*;

pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const F_ISTTY: c_int = 5;
pub const F_GETLK: c_int = 6;
pub const F_SETLK: c_int = 7;
pub const F_SETLKW: c_int = 8;
pub const F_DUPFD_CLOEXEC: c_int = 9;

pub const O_RDONLY: c_int = 1 << 0;
pub const O_WRONLY: c_int = 1 << 1;
pub const O_RDWR: c_int = O_RDONLY | O_WRONLY;
pub const O_ACCMODE: c_int = O_RDONLY | O_WRONLY;
pub const O_EXEC: c_int = 1 << 2;
pub const O_CREAT: c_int = 1 << 3;
pub const O_EXCL: c_int = 1 << 4;
pub const O_NOCTTY: c_int = 1 << 5;
pub const O_TRUNC: c_int = 1 << 6;
pub const O_APPEND: c_int = 1 << 7;
pub const O_NONBLOCK: c_int = 1 << 8;
pub const O_DIRECTORY: c_int = 1 << 9;
pub const O_NOFOLLOW: c_int = 1 << 10;
pub const O_CLOEXEC: c_int = 1 << 11;
pub const O_DIRECT: c_int = 1 << 12;
pub const O_SYNC: c_int = 1 << 13;

pub const F_RDLCK: c_short = 0;
pub const F_WRLCK: c_short = 1;
pub const F_UNLCK: c_short = 2;

pub const AT_FDCWD: c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const AT_REMOVEDIR: c_int = 0x200;
pub const AT_EACCESS: c_int = 0x400;

s! {
    pub struct flock {
        pub l_type: c_short,
        pub l_whence: c_short,
        pub l_start: crate::off_t,
        pub l_len: crate::off_t,
        pub l_pid: crate::pid_t,
    }
}
