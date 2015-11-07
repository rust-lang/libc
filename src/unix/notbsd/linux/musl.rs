pub const BUFSIZ: ::c_uint = 1024;
pub const TMP_MAX: ::c_uint = 10000;
pub const FOPEN_MAX: ::c_uint = 1000;
pub const POSIX_MADV_DONTNEED: ::c_int = 0;
pub const O_ACCMODE: ::c_int = 0o10000003;
pub const RUSAGE_CHILDREN: ::c_int = 1;
pub const NI_MAXHOST: ::socklen_t = 255;
pub const PTHREAD_STACK_MIN: ::size_t = 2048;

extern {
    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;
}
