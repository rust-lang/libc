//! This file contains the BSD APIs available in Haiku. It corresponds to the
//! header files in `headers/compatibility/bsd`.
//!
//! Note that Haiku's BSD compatibility is a combination of system APIs and
//! utility libraries. There should only be system APIs in `libc`. When you are
//! trying to determine whether something should be included in this file, the
//! best indicator is whether it also exists in the BSD-specific definitions in
//! this libc crate.

use crate::prelude::*;

// stringlist.h (utility library)
// Note: this is kept because it was previously introduced
pub type StringList = _stringlist;

s! {
    // stringlist.h (utility library)
    // Note: this is kept because it was previously introduced
    pub struct _stringlist {
        pub sl_str: *mut *mut c_char,
        pub sl_max: size_t,
        pub sl_cur: size_t,
    }

    // sys/link_elf.h
    pub struct dl_phdr_info {
        pub dlpi_addr: crate::Elf_Addr,
        pub dlpi_name: *const c_char,
        pub dlpi_phdr: *const crate::Elf_Phdr,
        pub dlpi_phnum: crate::Elf_Half,
    }
}

#[link(name = "bsd")]
extern "C" {
    // stdlib.h
    pub fn daemon(nochdir: c_int, noclose: c_int) -> c_int;
    pub fn getprogname() -> *const c_char;
    pub fn setprogname(progname: *const c_char);
    pub fn arc4random() -> u32;
    pub fn arc4random_uniform(upper_bound: u32) -> u32;
    pub fn arc4random_buf(buf: *mut c_void, n: size_t);

    // pty.h
    pub fn openpty(
        amaster: *mut c_int,
        aslave: *mut c_int,
        name: *mut c_char,
        termp: *mut crate::termios,
        winp: *mut crate::winsize,
    ) -> c_int;
    pub fn login_tty(_fd: c_int) -> c_int;
    pub fn forkpty(
        amaster: *mut c_int,
        name: *mut c_char,
        termp: *mut crate::termios,
        winp: *mut crate::winsize,
    ) -> crate::pid_t;

    // string.h
    pub fn strsep(string: *mut *mut c_char, delimiters: *const c_char) -> *mut c_char;
    pub fn explicit_bzero(buf: *mut c_void, len: size_t);

    // stringlist.h (utility library)
    // Note: this is kept because it was previously introduced
    pub fn sl_init() -> *mut StringList;
    pub fn sl_add(sl: *mut StringList, n: *mut c_char) -> c_int;
    pub fn sl_free(sl: *mut StringList, i: c_int);
    pub fn sl_find(sl: *mut StringList, n: *mut c_char) -> *mut c_char;

    // sys/link_elf.h
    pub fn dl_iterate_phdr(
        callback: Option<
            unsafe extern "C" fn(info: *mut dl_phdr_info, size: usize, data: *mut c_void) -> c_int,
        >,
        data: *mut c_void,
    ) -> c_int;

    // sys/time.h
    pub fn lutimes(file: *const c_char, times: *const crate::timeval) -> c_int;
}
