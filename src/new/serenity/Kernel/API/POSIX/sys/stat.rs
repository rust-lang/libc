//! File `Kernel/API/POSIX/sys/stat.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/sys/stat.h>
use crate::prelude::*;

pub const S_IFMT: crate::mode_t = 0o170000;
pub const S_IFDIR: crate::mode_t = 0o040000;
pub const S_IFCHR: crate::mode_t = 0o020000;
pub const S_IFBLK: crate::mode_t = 0o060000;
pub const S_IFREG: crate::mode_t = 0o100000;
pub const S_IFIFO: crate::mode_t = 0o010000;
pub const S_IFLNK: crate::mode_t = 0o120000;
pub const S_IFSOCK: crate::mode_t = 0o140000;

pub const S_IRUSR: crate::mode_t = 0o400;
pub const S_IWUSR: crate::mode_t = 0o200;
pub const S_IXUSR: crate::mode_t = 0o100;
pub const S_IREAD: crate::mode_t = S_IRUSR;
pub const S_IWRITE: crate::mode_t = S_IWUSR;
pub const S_IEXEC: crate::mode_t = S_IXUSR;
pub const S_IRGRP: crate::mode_t = 0o040;
pub const S_IWGRP: crate::mode_t = 0o020;
pub const S_IXGRP: crate::mode_t = 0o010;
pub const S_IROTH: crate::mode_t = 0o004;
pub const S_IWOTH: crate::mode_t = 0o002;
pub const S_IXOTH: crate::mode_t = 0o001;

pub const S_IRWXU: crate::mode_t = S_IRUSR | S_IWUSR | S_IXUSR;
pub const S_IRWXG: crate::mode_t = S_IRWXU >> 3;
pub const S_IRWXO: crate::mode_t = S_IRWXG >> 3;

s! {
    pub struct stat {
        pub st_dev: crate::dev_t,
        pub st_ino: crate::ino_t,
        pub st_mode: crate::mode_t,
        pub st_nlink: crate::nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: crate::dev_t,
        pub st_size: crate::off_t,
        pub st_blksize: crate::blksize_t,
        pub st_blocks: crate::blkcnt_t,
        pub st_atim: crate::timespec,
        pub st_mtim: crate::timespec,
        pub st_ctim: crate::timespec,
    }
}

pub const UTIME_OMIT: c_int = -1;
pub const UTIME_NOW: c_int = -2;
