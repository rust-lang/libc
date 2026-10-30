//! File `Kernel/API/POSIX/sys/statvfs.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/sys/statvfs.h>
use crate::prelude::*;

pub const ST_RDONLY: c_ulong = 0x1;
pub const ST_NOSUID: c_ulong = 0x2;

pub const FSTYPSZ: usize = 16;

s! {
    pub struct statvfs {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_files: crate::fsfilcnt_t,
        pub f_ffree: crate::fsfilcnt_t,
        pub f_favail: crate::fsfilcnt_t,
        pub f_fsid: c_ulong,
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
        pub f_basetype: [c_char; FSTYPSZ],
    }
}
