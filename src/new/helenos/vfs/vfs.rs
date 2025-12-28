//! HelenOS libc virtual file system APIs
//!
//! * Header file: <https://github.com/HelenOS/helenos/tree/master/uspace/lib/c/include/vfs/vfs.h>

pub use crate::ipc::loc::*;
pub use crate::ipc::vfs::*;
pub use crate::offset::*;
pub use crate::stdio::*;
use crate::{
    c_char,
    c_int,
    c_uint,
    errno_t,
};

s! {
    pub struct vfs_stat_t {
        pub fs_handle: fs_handle_t,
        pub service_id: service_id_t,
        pub index: fs_index_t,
        pub lnkcnt: c_uint,
        pub is_file: bool,
        pub is_directory: bool,
        pub size: aoff64_t,
        pub service: service_id_t,
    }
}

c_enum! {
    pub enum vfs_file_kind_t {
        KIND_FILE,
        KIND_DIRECTORY,
    }
}

extern "C" {
    pub fn vfs_fhandle(file: *mut FILE, handle: *mut c_int) -> errno_t;
    pub fn vfs_stat(handle: c_int, stat: *mut vfs_stat_t) -> errno_t;
    pub fn vfs_link_path(
        path: *const c_char,
        kind: vfs_file_kind_t,
        linkedfd: *mut c_int,
    ) -> errno_t;
    pub fn vfs_unlink_path(path: *const c_char) -> errno_t;
    pub fn vfs_rename_path(oldpath: *const c_char, newpath: *const c_char) -> errno_t;
    pub fn vfs_stat_path(path: *const c_char, stat: *mut vfs_stat_t) -> errno_t;
}
