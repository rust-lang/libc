// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(bad_style, raw_pointer_derive)]

#[macro_use] mod macros;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub enum FILE {}
pub enum fpos_t {}
pub enum DIR {}
pub enum dirent {}

cfg_if! {
    if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        mod apple;
        pub use apple::*;
    } else if #[cfg(any(target_os = "openbsd", target_os = "netbsd",
                        target_os = "dragonfly"))] {
        mod openbsdlike;
        pub use openbsdlike::*;
    } else if #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))] {
        mod freebsdlike;
        pub use freebsdlike::*;
    } else if #[cfg(any(target_os = "android", target_os = "linux"))] {
        mod linuxlike;
        pub use linuxlike::*;
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::*;
    } else {
        // ...
    }
}

#[cfg(unix)] mod unix;
#[cfg(unix)] pub use unix::*;

cfg_if! {
    if #[cfg(any(target_os = "macos",
                 target_os = "ios",
                 target_os = "freebsd",
                 target_os = "dragonfly",
                 target_os = "bitrig",
                 target_os = "netbsd",
                 target_os = "openbsd"))] {
        extern {
            pub fn sysctl(name: *mut c_int,
                          namelen: c_uint,
                          oldp: *mut c_void,
                          oldlenp: *mut size_t,
                          newp: *mut c_void,
                          newlen: size_t)
                          -> c_int;
            pub fn mincore(addr: *const c_void, len: size_t, vec: *mut c_char)
                           -> c_int;
            pub fn sysctlbyname(name: *const c_char,
                                oldp: *mut c_void,
                                oldlenp: *mut size_t,
                                newp: *mut c_void,
                                newlen: size_t)
                                -> c_int;
            pub fn sysctlnametomib(name: *const c_char,
                                   mibp: *mut c_int,
                                   sizep: *mut size_t)
                                   -> c_int;
        }
    } else {
        extern {
            pub fn sysctl(name: *mut c_int,
                          namelen: c_int,
                          oldp: *mut c_void,
                          oldlenp: *mut size_t,
                          newp: *mut c_void,
                          newlen: size_t)
                          -> c_int;
            pub fn mincore(addr: *mut c_void, len: size_t, vec: *mut c_uchar)
                           -> c_int;
        }
    }
}

extern {
    pub fn isalnum(c: c_int) -> c_int;
    pub fn isalpha(c: c_int) -> c_int;
    pub fn iscntrl(c: c_int) -> c_int;
    pub fn isdigit(c: c_int) -> c_int;
    pub fn isgraph(c: c_int) -> c_int;
    pub fn islower(c: c_int) -> c_int;
    pub fn isprint(c: c_int) -> c_int;
    pub fn ispunct(c: c_int) -> c_int;
    pub fn isspace(c: c_int) -> c_int;
    pub fn isupper(c: c_int) -> c_int;
    pub fn isxdigit(c: c_int) -> c_int;
    pub fn tolower(c: c_int) -> c_int;
    pub fn toupper(c: c_int) -> c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "fopen$UNIX2003")]
    pub fn fopen(filename: *const c_char,
                 mode: *const c_char) -> *mut FILE;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "freopen$UNIX2003")]
    pub fn freopen(filename: *const c_char, mode: *const c_char,
                   file: *mut FILE)
                   -> *mut FILE;
    pub fn fflush(file: *mut FILE) -> c_int;
    pub fn fclose(file: *mut FILE) -> c_int;
    pub fn remove(filename: *const c_char) -> c_int;
    pub fn rename(oldname: *const c_char,
                  newname: *const c_char) -> c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn setvbuf(stream: *mut FILE,
                   buffer: *mut c_char,
                   mode: c_int,
                   size: size_t)
                   -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE)
                 -> *mut c_char;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "fputs$UNIX2003")]
    pub fn fputs(s: *const c_char, stream: *mut FILE)-> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fread(ptr: *mut c_void,
                 size: size_t,
                 nobj: size_t,
                 stream: *mut FILE)
                 -> size_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "fwrite$UNIX2003")]
    pub fn fwrite(ptr: *const c_void,
                  size: size_t,
                  nobj: size_t,
                  stream: *mut FILE)
                  -> size_t;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int)
                 -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn rewind(stream: *mut FILE);
    pub fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int;
    pub fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int;
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn perror(s: *const c_char);
    pub fn abs(i: c_int) -> c_int;
    pub fn labs(i: c_long) -> c_long;
    pub fn atof(s: *const c_char) -> c_double;
    pub fn atoi(s: *const c_char) -> c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "strtod$UNIX2003")]
    pub fn strtod(s: *const c_char,
                  endp: *mut *mut c_char) -> c_double;
    pub fn strtol(s: *const c_char,
                  endp: *mut *mut c_char, base: c_int) -> c_long;
    pub fn strtoul(s: *const c_char, endp: *mut *mut c_char,
                   base: c_int) -> c_ulong;
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn exit(status: c_int) -> !;
    pub fn _exit(status: c_int) -> !;
    pub fn atexit(cb: extern fn()) -> c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "system$UNIX2003")]
    pub fn system(s: *const c_char) -> c_int;
    pub fn getenv(s: *const c_char) -> *mut c_char;
    pub fn rand() -> c_int;
    pub fn srand(seed: c_uint);

    pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t)
                   -> *mut c_char;
    pub fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char;
    pub fn strncat(s: *mut c_char, ct: *const c_char, n: size_t) -> *mut c_char;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int;
    pub fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strlen(cs: *const c_char) -> size_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "strerror$UNIX2003")]
    pub fn strerror(n: c_int) -> *mut c_char;
    pub fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char;
    pub fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t;
    pub fn wcslen(buf: *const wchar_t) -> size_t;

    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
}
