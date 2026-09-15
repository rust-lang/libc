use crate::prelude::*;

// FIXME(1.0,deprecate): lfs binding to be removed
pub type statfs64 = statfs;

s! {
    pub struct statfs {
        pub f_type: c_uint,
        pub f_bsize: c_uint,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_files: crate::fsfilcnt_t,
        pub f_ffree: crate::fsfilcnt_t,
        pub f_fsid: crate::fsid_t,
        pub f_namelen: c_uint,
        pub f_frsize: c_uint,
        pub f_flags: c_uint,
        f_spare: Padding<[c_uint; 4]>,
    }
}
