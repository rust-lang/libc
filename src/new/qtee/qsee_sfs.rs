//! Header: `qsee_sfs.h`

use crate::prelude::*;

pub type sfs_errno_enum_type = c_uint;

pub const SFS_NO_ERROR: sfs_errno_enum_type = 0;
pub const SFS_ERROR_GENERIC: sfs_errno_enum_type = 1;
pub const SFS_NO_MEMORY: sfs_errno_enum_type = 2;
pub const SFS_FS_ERROR: sfs_errno_enum_type = 3;
pub const SFS_NO_FILE: sfs_errno_enum_type = 4;
pub const SFS_VERI_HMAC_ERROR: sfs_errno_enum_type = 5;

extern "C" {
    pub fn qsee_sfs_open(path: *const c_char, flags: c_int) -> c_int;
    pub fn qsee_sfs_read(fd: c_int, buf: *mut c_char, nbytes: c_int) -> c_int;
    pub fn qsee_sfs_write(fd: c_int, buf: *const c_char, nbytes: c_int) -> c_int;
    pub fn qsee_sfs_seek(fd: c_int, offset: i32, whence: c_int) -> c_int;
    pub fn qsee_sfs_close(fd: c_int) -> c_int;
    pub fn qsee_sfs_rm(path: *const c_char) -> c_int;
    pub fn qsee_sfs_getSize(fd: c_int, size: *mut u32) -> c_int;
    pub fn qsee_sfs_error(fd: c_int) -> c_int;
}
