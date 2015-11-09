s! {
    pub struct glob64_t {
        pub gl_pathc: ::size_t,
        pub gl_pathv: *mut *mut ::c_char,
        pub gl_offs: ::size_t,
        pub gl_flags: ::c_int,

        __unused1: *mut ::c_void,
        __unused2: *mut ::c_void,
        __unused3: *mut ::c_void,
        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
    }
}

pub const BUFSIZ: ::c_uint = 8192;
pub const TMP_MAX: ::c_uint = 238328;
pub const FOPEN_MAX: ::c_uint = 16;
pub const POSIX_MADV_DONTNEED: ::c_int = 4;
pub const _SC_2_C_VERSION: ::c_int = 96;
pub const RUSAGE_THREAD: ::c_int = 1;
pub const O_ACCMODE: ::c_int = 3;
pub const RUSAGE_CHILDREN: ::c_int = -1;
pub const ST_RELATIME: ::c_ulong = 4096;
pub const NI_MAXHOST: ::socklen_t = 1025;

pub const FIOCLEX: ::c_ulong = 0x5451;
pub const FIONBIO: ::c_ulong = 0x5421;

cfg_if! {
    if #[cfg(any(target_arch = "arm", target_arch = "x86",
                        target_arch = "x86_64"))] {
        pub const PTHREAD_STACK_MIN: ::size_t = 16384;
    } else {
        pub const PTHREAD_STACK_MIN: ::size_t = 131072;
    }
}

extern {
    pub fn sysctl(name: *mut ::c_int,
                  namelen: ::c_int,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *mut ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
    pub fn backtrace(buf: *mut *mut ::c_void,
                     sz: ::c_int) -> ::c_int;
    pub fn glob64(pattern: *const ::c_char,
                  flags: ::c_int,
                  errfunc: ::dox::Option<extern "C" fn(epath: *const ::c_char,
                                                       errno: ::c_int) -> ::c_int>,
                  pglob: *mut glob64_t) -> ::c_int;
    pub fn globfree64(pglob: *mut glob64_t);
}
