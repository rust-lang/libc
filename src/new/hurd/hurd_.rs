//! Header: `hurd.h`
//!
//! <https://github.com/sailfishos-mirror/glibc/blob/master/hurd/hurd.h>

use crate::prelude::*;
use crate::{
    mode_t,
    pid_t,
    uid_t,
};

extern "C" {
    pub fn getproc() -> c_uint;
    pub fn getcwdir() -> c_uint;
    pub fn getcrdir() -> c_uint;
    pub fn getauth() -> c_uint;
    pub fn getcttyid() -> c_uint;
    pub fn setproc(proc: c_uint) -> c_int;
    pub fn setcwdir(cwdir: c_uint) -> c_int;
    pub fn setcrdir(crdir: c_uint) -> c_int;
    pub fn setcttyid(cttyid: c_uint) -> c_int;
    pub fn setauth(auth: c_uint) -> c_int;
    pub fn geteuids(n: c_int, uidset: *mut uid_t) -> c_int;
    pub fn seteuids(n: c_int, uidset: *const uid_t) -> c_int;
    pub fn file_name_split(file: *const c_char, name: *mut *mut c_char) -> c_uint;
    pub fn directory_name_split(file: *const c_char, name: *mut *mut c_char) -> c_uint;
    pub fn file_name_lookup(file: *const c_char, flags: c_int, mode: mode_t) -> c_uint;
    pub fn file_name_lookup_under(
        startdir: c_uint,
        file: *const c_char,
        flags: c_int,
        mode: mode_t,
    ) -> c_uint;
    pub fn file_name_path_lookup(
        file_name: *const c_char,
        path: *const c_char,
        flags: c_int,
        mode: mode_t,
        prefixed_name: *mut *mut c_char,
    ) -> c_uint;
    pub fn openport(port: c_uint, flags: c_int) -> c_int;
    pub fn fopenport(port: c_uint, mode: *const c_char) -> *mut crate::FILE;
    pub fn hurd_sig_post(pid: pid_t, sig: c_int, refport: c_uint) -> c_int;
    pub fn get_privileged_ports(
        host_priv_ptr: *mut c_uint,
        device_master_ptr: *mut c_uint,
    ) -> c_int;
    pub fn task2pid(task: c_uint) -> pid_t;
    pub fn pid2task(pid: pid_t) -> c_uint;
    pub fn hurd_thread_self() -> c_uint;
    pub fn hurd_thread_cancel(thread: c_uint) -> c_int;
    pub fn hurd_check_cancel() -> c_int;
    pub fn getdport(fd: c_int) -> c_uint;
    // The last argument is a `va_list`.
    pub fn vpprintf(port: c_uint, format: *const c_char, arg: *mut c_void) -> c_int;
}
