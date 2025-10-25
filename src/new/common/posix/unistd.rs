//! Header: `unistd.h`
//!
//! <https://pubs.opengroup.org/onlinepubs/007904975/basedefs/unistd.h.html>

use crate::prelude::*;
use crate::{
    gid_t,
    off_t,
    pid_t,
    uid_t,
};

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

extern "C" {
    pub fn access(path: *const c_char, amode: c_int) -> c_int;
    pub fn alarm(seconds: c_uint) -> c_uint;
    pub fn chdir(dir: *const c_char) -> c_int;
    pub fn chown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int;

    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "close$NOCANCEL$UNIX2003"
    )]
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86_64"),
        link_name = "close$NOCANCEL"
    )]
    pub fn close(fd: c_int) -> c_int;

    #[cfg(not(any(target_os = "android", target_os = "vxworks")))]
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "confstr$UNIX2003"
    )]
    #[cfg_attr(target_os = "solaris", link_name = "__confstr_xpg7")]
    pub fn confstr(name: c_int, buf: *mut c_char, len: size_t) -> size_t;

    // Not provided: crypt
    // Not provided: ctermid

    pub fn dup(fd: c_int) -> c_int;
    pub fn dup2(src: c_int, dst: c_int) -> c_int;

    // Not provided: encrypt

    #[cfg(not(target_os = "vxworks"))]
    pub fn execl(path: *const c_char, arg0: *const c_char, ...) -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    pub fn execle(path: *const c_char, arg0: *const c_char, ...) -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    pub fn execlp(file: *const c_char, arg0: *const c_char, ...) -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    pub fn execv(prog: *const c_char, argv: *const *mut c_char) -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    pub fn execve(prog: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char)
        -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    pub fn execvp(c: *const c_char, argv: *const *mut c_char) -> c_int;

    pub fn _exit(status: c_int) -> !;

    pub fn fchown(fd: c_int, owner: uid_t, group: gid_t) -> c_int;

    #[cfg(not(target_os = "fuchsia"))]
    pub fn fchdir(dirfd: c_int) -> c_int;

    // Support for fdatasync varies

    #[cfg(not(target_os = "vxworks"))]
    pub fn fork() -> pid_t;

    pub fn fpathconf(filedes: c_int, name: c_int) -> c_long;

    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "fsync$UNIX2003"
    )]
    pub fn fsync(fd: c_int) -> c_int;

    #[cfg_attr(gnu_file_offset_bits64, link_name = "ftruncate64")]
    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;

    pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    pub fn getegid() -> gid_t;
    pub fn geteuid() -> uid_t;
    pub fn getgid() -> gid_t;
    pub fn getgroups(ngroups_max: c_int, groups: *mut gid_t) -> c_int;
    // Support for gethostid varies

    #[cfg(not(target_os = "espidf"))]
    pub fn gethostname(name: *mut c_char, len: size_t) -> c_int;

    #[cfg_attr(target_os = "illumos", link_name = "getloginx")]
    pub fn getlogin() -> *mut c_char;

    // Support for getlogin_r varies

    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "getopt$UNIX2003"
    )]
    pub fn getopt(argc: c_int, argv: *const *mut c_char, optstr: *const c_char) -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    pub fn getpgid(pid: pid_t) -> pid_t;

    #[cfg(not(target_os = "vxworks"))]
    pub fn getpgrp() -> pid_t;
    pub fn getpid() -> pid_t;
    pub fn getppid() -> pid_t;

    #[cfg(not(any(target_os = "redox", target_os = "vxworks")))]
    pub fn getsid(pid: pid_t) -> pid_t;
    pub fn getuid() -> uid_t;

    pub fn isatty(fd: c_int) -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "lchown$UNIX2003"
    )]
    pub fn lchown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int;

    #[cfg_attr(target_os = "solaris", link_name = "__link_xpg4")]
    pub fn link(src: *const c_char, dst: *const c_char) -> c_int;

    #[cfg(not(any(target_os = "fuchsia", target_os = "vxworks")))]
    #[cfg_attr(gnu_file_offset_bits64, link_name = "lockf64")]
    pub fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int;

    #[cfg_attr(gnu_file_offset_bits64, link_name = "lseek64")]
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;

    #[cfg(not(any(target_os = "fuchsia", target_os = "vxworks")))]
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "nice$UNIX2003"
    )]
    pub fn nice(incr: c_int) -> c_int;

    pub fn pathconf(path: *const c_char, name: c_int) -> c_long;

    #[cfg(not(target_os = "redox"))]
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "pause$UNIX2003"
    )]
    pub fn pause() -> c_int;

    pub fn pipe(fds: *mut c_int) -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "pread$UNIX2003"
    )]
    #[cfg_attr(gnu_file_offset_bits64, link_name = "pread64")]
    pub fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;

    #[cfg(not(target_os = "vxworks"))]
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "pwrite$UNIX2003"
    )]
    #[cfg_attr(gnu_file_offset_bits64, link_name = "pwrite64")]
    pub fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t;

    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "read$UNIX2003"
    )]
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;

    #[cfg(not(target_os = "nto"))]
    pub fn readlink(path: *const c_char, buf: *mut c_char, bufsz: size_t) -> ssize_t;

    pub fn rmdir(path: *const c_char) -> c_int;

    pub fn setegid(gid: gid_t) -> c_int;
    pub fn seteuid(uid: uid_t) -> c_int;
    pub fn setgid(gid: gid_t) -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    pub fn setpgid(pid: pid_t, pgid: pid_t) -> c_int;

    // Support for setpgrp varies

    #[cfg(not(target_os = "vxworks"))]
    pub fn setregid(rgid: gid_t, egid: gid_t) -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    pub fn setreuid(ruid: uid_t, euid: uid_t) -> c_int;

    #[cfg(not(target_os = "vxworks"))]
    pub fn setsid() -> pid_t;
    pub fn setuid(uid: uid_t) -> c_int;

    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "sleep$UNIX2003"
    )]
    pub fn sleep(secs: c_uint) -> c_uint;
    // Not provided: swab

    pub fn symlink(path1: *const c_char, path2: *const c_char) -> c_int;
    // Support for sync varies

    #[cfg_attr(target_os = "solaris", link_name = "__sysconf_xpg7")]
    pub fn sysconf(name: c_int) -> c_long;

    #[cfg(not(target_os = "vxworks"))]
    pub fn tcgetpgrp(fd: c_int) -> pid_t;

    #[cfg(not(target_os = "vxworks"))]
    pub fn tcsetpgrp(fd: c_int, pgrp: pid_t) -> c_int;

    #[cfg(not(target_os = "fuchsia"))]
    #[cfg_attr(gnu_file_offset_bits64, link_name = "truncate64")]
    pub fn truncate(path: *const c_char, length: off_t) -> c_int;

    pub fn ttyname(fd: c_int) -> *mut c_char;

    #[cfg(not(any(target_os = "fuchsia", target_os = "vxworks")))]
    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "ttyname_r$UNIX2003"
    )]
    #[cfg_attr(
        any(target_os = "illumos", target_os = "solaris"),
        link_name = "__posix_ttyname_r"
    )]
    pub fn ttyname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int;

    // Not provided: ualarm

    pub fn unlink(c: *const c_char) -> c_int;

    // FIXME(unix): usleep could probably live here but our definition seems incorrect (c_uint vs.
    // useconds_t).

    #[cfg_attr(
        all(target_os = "macos", target_arch = "x86"),
        link_name = "write$UNIX2003"
    )]
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
}

// Statics optarg, optind, opterr, and optopt are not provided.
