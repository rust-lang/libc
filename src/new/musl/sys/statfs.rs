//! Header: `sys/statfs.h`
//!
//! * Headers: <https://git.musl-libc.org/cgit/musl/blob/arch/include/sys/statfs.h> (official)
//! * Headers: <https://github.com/kraj/musl/blob/kraj/master/include/sys/statfs.h> (mirror)

cfg_if! {
    // MIPS/s390x implementation is special (see arch folders)
    if #[cfg(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "s390x"
    ))] {
        pub use crate::bits::statfs::{
            statfs,
            statfs64,
        };
    } else {
        use crate::prelude::*;

        s! {
            pub struct statfs {
                pub f_type: c_ulong,
                pub f_bsize: c_ulong,
                pub f_blocks: crate::fsblkcnt_t,
                pub f_bfree: crate::fsblkcnt_t,
                pub f_bavail: crate::fsblkcnt_t,
                pub f_files: crate::fsfilcnt_t,
                pub f_ffree: crate::fsfilcnt_t,
                pub f_fsid: crate::fsid_t,
                pub f_namelen: c_ulong,
                pub f_frsize: c_ulong,
                pub f_flags: c_ulong,
                f_spare: Padding<[c_ulong; 4]>,
            }
        }

        // FIXME(1.0,deprecate): lfs binding to be removed
        pub type statfs64 = statfs;
    }
}
