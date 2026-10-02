//! Header: `sys/statvfs.h`
//!
//! * Headers: <https://git.musl-libc.org/cgit/musl/blob/arch/include/sys/statvfs.h> (official)
//! * Headers: <https://github.com/kraj/musl/blob/kraj/master/include/sys/statvfs.h> (mirror)

use crate::prelude::*;

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
        #[cfg(target_endian = "little")]
        pub f_fsid: c_ulong,
        #[cfg(target_pointer_width = "32")]
        __pad: Padding<c_int>,
        #[cfg(target_endian = "big")]
        pub f_fsid: c_ulong,
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
        __f_reserved: Padding<[c_int; 6]>,
    }
}

// FIXME(1.0,deprecate): lfs binding to be removed
pub type statvfs64 = statvfs;
