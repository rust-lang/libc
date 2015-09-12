//! All Unix OS-es that are not android

extern {
    pub fn getifaddrs(ifap: *mut *mut ::ifaddrs) -> ::c_int;
    pub fn freeifaddrs(ifa: *mut ::ifaddrs);
    pub fn glob(pattern: *const ::c_char,
                flags: ::c_int,
                errfunc: Option<extern "C" fn(epath: *const ::c_char,
                                                  errno: ::c_int) -> ::c_int>,
                pglob: *mut ::glob_t);
    pub fn globfree(pglob: *mut ::glob_t);

    pub fn posix_madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int)
                         -> ::c_int;

    #[cfg(target_os = "macos")]
    pub fn shm_open(name: *const ::c_char, oflag: ::c_int, ...)
                    -> ::c_int;
    #[cfg(target_os = "linux")]
    pub fn shm_open(name: *const ::c_char, oflag: ::c_int,
                    mode: ::mode_t) -> ::c_int;
    pub fn shm_unlink(name: *const ::c_char) -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86_64"),
               link_name = "seekdir$INODE64")]
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "seekdir$INODE64$UNIX2003")]
    pub fn seekdir(dirp: *mut ::DIR, loc: ::c_long);

    #[cfg_attr(all(target_os = "macos", target_arch = "x86_64"),
               link_name = "telldir$INODE64")]
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "telldir$INODE64$UNIX2003")]
    pub fn telldir(dirp: *mut ::DIR) -> ::c_long;

    pub fn getsid(pid: ::pid_t) -> ::pid_t;
    pub fn madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int)
                   -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "putenv$UNIX2003")]
    pub fn putenv(string: *mut ::c_char) -> ::c_int;
    pub fn readlink(path: *const ::c_char,
                    buf: *mut ::c_char,
                    bufsz: ::size_t)
                    -> ::ssize_t;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "msync$UNIX2003")]
    pub fn msync(addr: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "mprotect$UNIX2003")]
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn sysconf(name: ::c_int) -> ::c_long;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "usleep$UNIX2003")]
    pub fn usleep(secs: ::c_uint) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "recvfrom$UNIX2003")]
    pub fn recvfrom(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                    flags: ::c_int, addr: *mut ::sockaddr,
                    addrlen: *mut ::socklen_t) -> ::ssize_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "send$UNIX2003")]
    pub fn send(socket: ::c_int, buf: *const ::c_void, len: ::size_t,
                flags: ::c_int) -> ::ssize_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "recv$UNIX2003")]
    pub fn recv(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                flags: ::c_int) -> ::ssize_t;
    pub fn mkfifo(path: *const ::c_char, mode: ::mode_t) -> ::c_int;
}

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
