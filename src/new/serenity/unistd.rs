//! Header: `unistd.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/unistd.h>

pub use super::Kernel::API::POSIX::unistd::*;
use crate::prelude::*;

pub const HZ: c_int = 1000;

pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

extern "C" {
    pub fn get_process_name(buffer: *mut c_char, buffer_size: c_int) -> c_int;
    pub fn set_process_name(name: *const c_char, name_length: size_t) -> c_int;
    pub fn dump_backtrace();
    pub fn gettid() -> c_int;
    pub fn getpagesize() -> c_int;
    pub fn vfork() -> crate::pid_t;
    pub fn daemon(nochdir: c_int, noclose: c_int) -> c_int;
    pub fn execvpe(
        filename: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_int;
    pub fn sync();
    pub fn getresuid(
        ruid: *mut crate::uid_t,
        euid: *mut crate::uid_t,
        suid: *mut crate::uid_t,
    ) -> c_int;
    pub fn getresgid(
        rgid: *mut crate::gid_t,
        egid: *mut crate::gid_t,
        sgid: *mut crate::gid_t,
    ) -> c_int;
    pub fn setgroups(size: size_t, list: *const crate::gid_t) -> c_int;
    pub fn setresuid(ruid: crate::uid_t, euid: crate::uid_t, suid: crate::uid_t) -> c_int;
    pub fn setresgid(rgid: crate::gid_t, egid: crate::gid_t, sgid: crate::gid_t) -> c_int;
    pub fn getwd(buffer: *mut c_char) -> *mut c_char;
    pub fn sethostname(name: *const c_char, len: ssize_t) -> c_int;
    pub fn pipe2(pipefd: *mut c_int, flags: c_int) -> c_int;
    pub fn faccessat(dirfd: c_int, pathname: *const c_char, mode: c_int, flags: c_int) -> c_int;
    pub fn fsopen(fs_type: *const c_char, flags: c_int) -> c_int;
    pub fn fsmount(
        vfs_context_id: c_int,
        mount_fd: c_int,
        source_fd: c_int,
        target: *const c_char,
    ) -> c_int;
    pub fn bindmount(
        vfs_context_id: c_int,
        source_fd: c_int,
        target: *const c_char,
        flags: c_int,
    ) -> c_int;
    pub fn mount(
        source_fd: c_int,
        target: *const c_char,
        fs_type: *const c_char,
        flags: c_int,
    ) -> c_int;
    pub fn umount(mountpoint: *const c_char) -> c_int;
    pub fn pledge(promises: *const c_char, execpromises: *const c_char) -> c_int;
    pub fn unveil(path: *const c_char, permissions: *const c_char) -> c_int;
    pub fn getpass(prompt: *const c_char) -> *mut c_char;
    pub fn getdtablesize() -> c_int;
    pub fn brk(addr: *mut c_void) -> c_int;
    pub fn sbrk(incr: intptr_t) -> *mut c_void;
}

pub const _PC_NAME_MAX: c_int = 0;
pub const _PC_PATH_MAX: c_int = 1;
pub const _PC_PIPE_BUF: c_int = 2;
pub const _PC_VDISABLE: c_int = 3;
pub const _PC_LINK_MAX: c_int = 4;

pub const _POSIX_VDISABLE: crate::cc_t = 0;

// Header `serenity.h`
extern "C" {
    // TODO: Add futex bindings

    // NOTE: We are lying about the first argument.
    // In C bindings, it is a uintptr_t*. However, this causes provenence problems in rust.
    // The provenence of the returned stack pointer is, in fact, the stack.
    pub fn get_stack_bounds(
        user_stack_base: *mut *mut c_void,
        user_stack_size: *mut usize,
    ) -> c_int;
    pub fn anon_create(size: usize, options: c_int) -> c_int;
}
