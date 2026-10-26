//! Source header: `sysdeps/unix/sysv/linux/sys/mount.h`
//!
//! <https://github.com/sailfishos-mirror/glibc/blob/master/sysdeps/unix/sysv/linux/sys/mount.h>

pub use crate::new::linux_uapi::linux::mount::*;
use crate::prelude::*;

extern "C" {
    pub fn fsconfig(
        fd: c_int,
        cmd: c_uint,
        key: *const c_char,
        value: *const c_void,
        aux: c_int,
    ) -> c_int;
    pub fn fsmount(fd: c_int, flags: c_uint, ms_flags: c_uint) -> c_int;
    pub fn fsopen(fs_name: *const c_char, flags: c_uint) -> c_int;
    pub fn fspick(dfd: c_int, path: *const c_char, flags: c_uint) -> c_int;

    pub fn mount_setattr(
        dfd: c_int,
        path: *const c_char,
        flags: c_uint,
        uattr: *mut mount_attr,
        size: size_t,
    ) -> c_int;
    pub fn move_mount(
        from_dfd: c_int,
        from_pathname: *const c_char,
        to_dfd: c_int,
        to_pathname: *const c_char,
        flags: c_uint,
    ) -> c_int;

    pub fn open_tree(dfd: c_int, filename: *const c_char, flags: c_uint) -> c_int;
}
