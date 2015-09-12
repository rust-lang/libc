//! Android versions of standard Unix functions

extern {
    pub fn madvise(addr: *const ::c_void, len: ::size_t, advice: ::c_int)
                   -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;
    pub fn putenv(string: *const ::c_char) -> ::c_int;
    pub fn readlink(path: *const ::c_char,
                    buf: *mut ::c_char,
                    bufsz: ::size_t)
                    -> ::c_int;
    pub fn msync(addr: *const ::c_void, len: ::size_t,
                 flags: ::c_int) -> ::c_int;
    pub fn mprotect(addr: *const ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn sysconf(name: ::c_int) -> ::c_int;
    pub fn usleep(secs: ::c_ulong) -> ::c_int;
    pub fn recvfrom(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                    flags: ::c_uint, addr: *const ::sockaddr,
                    addrlen: *mut ::socklen_t) -> ::ssize_t;
    pub fn send(socket: ::c_int, buf: *const ::c_void, len: ::size_t,
                flags: ::c_uint) -> ::ssize_t;
    pub fn recv(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                flags: ::c_uint) -> ::ssize_t;
}
