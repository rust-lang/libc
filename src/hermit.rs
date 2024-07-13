//! Hermit C type definitions

cfg_if! {
    if #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))] {
        pub type c_char = u8;
    } else {
        pub type c_char = i8;
    }
}

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type intptr_t = isize;
pub type uintptr_t = usize;

pub type c_float = f32;
pub type c_double = f64;

pub type size_t = usize;
pub type ssize_t = isize;
pub type ptrdiff_t = isize;

pub type clockid_t = i32;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type mode_t = u32;
pub type nfds_t = usize;
pub type pid_t = i32;
pub type sa_family_t = u8;
pub type socklen_t = u32;
pub type time_t = i64;

s! {
    pub struct addrinfo {
        pub ai_flags: i32,
        pub ai_family: i32,
        pub ai_socktype: i32,
        pub ai_protocol: i32,
        pub ai_addrlen: socklen_t,
        pub ai_canonname: *mut c_char,
        pub ai_addr: *mut sockaddr,
        pub ai_next: *mut addrinfo,
    }

    pub struct dirent64 {
        pub d_ino: u64,
        pub d_off: i64,
        pub d_reclen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 256],
    }

    #[repr(align(4))]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    pub struct in_addr {
        pub s_addr: in_addr_t,
    }

    pub struct iovec {
        iov_base: *mut c_void,
        iov_len: usize,
    }

    pub struct pollfd {
        pub fd: i32,
        pub events: i16,
        pub revents: i16,
    }

    pub struct sockaddr {
        pub sa_len: u8,
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [c_char; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_len: u8,
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_storage {
        pub ss_len: u8,
        pub ss_family: sa_family_t,
        __ss_pad1: [u8; 6],
        __ss_align: i64,
        __ss_pad2: [u8; 112],
    }

    pub struct stat {
        pub st_dev: u64,
        pub st_ino: u64,
        pub st_nlink: u64,
        pub st_mode: mode_t,
        pub st_uid: u32,
        pub st_gid: u32,
        pub st_rdev: u64,
        pub st_size: u64,
        pub st_blksize: i64,
        pub st_blocks: i64,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
    }

    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: i32,
    }
}

pub const AF_INET: i32 = 0;
pub const AF_INET6: i32 = 1;

pub const CLOCK_REALTIME: clockid_t = 1;
pub const CLOCK_MONOTONIC: clockid_t = 4;

pub const DT_UNKNOWN: u8 = 0;
pub const DT_FIFO: u8 = 1;
pub const DT_CHR: u8 = 2;
pub const DT_DIR: u8 = 4;
pub const DT_BLK: u8 = 6;
pub const DT_REG: u8 = 8;
pub const DT_LNK: u8 = 10;
pub const DT_SOCK: u8 = 12;
pub const DT_WHT: u8 = 14;

pub const EAI_AGAIN: i32 = 2;
pub const EAI_BADFLAGS: i32 = 3;
pub const EAI_FAIL: i32 = 4;
pub const EAI_FAMILY: i32 = 5;
pub const EAI_MEMORY: i32 = 6;
pub const EAI_NODATA: i32 = 7;
pub const EAI_NONAME: i32 = 8;
pub const EAI_SERVICE: i32 = 9;
pub const EAI_SOCKTYPE: i32 = 10;
pub const EAI_SYSTEM: i32 = 11;
pub const EAI_OVERFLOW: i32 = 14;

pub const EFD_SEMAPHORE: i16 = 0o1;
pub const EFD_NONBLOCK: i16 = 0o4000;
pub const EFD_CLOEXEC: i16 = 0o40000;

pub const F_DUPFD: i32 = 0;
pub const F_GETFD: i32 = 1;
pub const F_SETFD: i32 = 2;
pub const F_GETFL: i32 = 3;
pub const F_SETFL: i32 = 4;

pub const FD_CLOEXEC: i32 = 1;

pub const FIONBIO: i32 = 0x8008667e;

pub const FUTEX_RELATIVE_TIMEOUT: u32 = 1;

pub const IP_TOS: i32 = 1;
pub const IP_TTL: i32 = 2;
pub const IP_ADD_MEMBERSHIP: i32 = 3;
pub const IP_DROP_MEMBERSHIP: i32 = 4;
pub const IP_MULTICAST_TTL: i32 = 5;
pub const IP_MULTICAST_LOOP: i32 = 7;

pub const IPPROTO_IP: i32 = 0;
pub const IPPROTO_TCP: i32 = 6;
pub const IPPROTO_UDP: i32 = 17;
pub const IPPROTO_IPV6: i32 = 41;

pub const IPV6_ADD_MEMBERSHIP: i32 = 12;
pub const IPV6_DROP_MEMBERSHIP: i32 = 13;
pub const IPV6_MULTICAST_LOOP: i32 = 19;
pub const IPV6_V6ONLY: i32 = 27;

pub const MSG_PEEK: i32 = 1;

pub const O_RDONLY: i32 = 0o0;
pub const O_WRONLY: i32 = 0o1;
pub const O_RDWR: i32 = 0o2;
pub const O_CREAT: i32 = 0o100;
pub const O_EXCL: i32 = 0o200;
pub const O_TRUNC: i32 = 0o1000;
pub const O_APPEND: i32 = 0o2000;
pub const O_NONBLOCK: i32 = 0o4000;
pub const O_DIRECTORY: i32 = 0o200000;

pub const POLLIN: i16 = 0x1;
pub const POLLPRI: i16 = 0x2;
pub const POLLOUT: i16 = 0x4;
pub const POLLERR: i16 = 0x8;
pub const POLLHUP: i16 = 0x10;
pub const POLLNVAL: i16 = 0x20;
pub const POLLRDNORM: i16 = 0x040;
pub const POLLRDBAND: i16 = 0x080;
pub const POLLWRNORM: i16 = 0x0100;
pub const POLLWRBAND: i16 = 0x0200;
pub const POLLRDHUP: i16 = 0x2000;

pub const S_IRWXU: mode_t = 0o0700;
pub const S_IRUSR: mode_t = 0o0400;
pub const S_IWUSR: mode_t = 0o0200;
pub const S_IXUSR: mode_t = 0o0100;
pub const S_IRWXG: mode_t = 0o0070;
pub const S_IRGRP: mode_t = 0o0040;
pub const S_IWGRP: mode_t = 0o0020;
pub const S_IXGRP: mode_t = 0o0010;
pub const S_IRWXO: mode_t = 0o0007;
pub const S_IROTH: mode_t = 0o0004;
pub const S_IWOTH: mode_t = 0o0002;
pub const S_IXOTH: mode_t = 0o0001;

pub const S_IFMT: mode_t = 0o17_0000;
pub const S_IFSOCK: mode_t = 0o14_0000;
pub const S_IFLNK: mode_t = 0o12_0000;
pub const S_IFREG: mode_t = 0o10_0000;
pub const S_IFBLK: mode_t = 0o6_0000;
pub const S_IFDIR: mode_t = 0o4_0000;
pub const S_IFCHR: mode_t = 0o2_0000;
pub const S_IFIFO: mode_t = 0o1_0000;

pub const SHUT_RD: i32 = 0;
pub const SHUT_WR: i32 = 1;
pub const SHUT_RDWR: i32 = 2;

pub const SO_REUSEADDR: i32 = 0x0004;
pub const SO_KEEPALIVE: i32 = 0x0008;
pub const SO_BROADCAST: i32 = 0x0020;
pub const SO_LINGER: i32 = 0x0080;
pub const SO_SNDBUF: i32 = 0x1001;
pub const SO_RCVBUF: i32 = 0x1002;
pub const SO_SNDTIMEO: i32 = 0x1005;
pub const SO_RCVTIMEO: i32 = 0x1006;
pub const SO_ERROR: i32 = 0x1007;

pub const SOCK_STREAM: i32 = 1;
pub const SOCK_DGRAM: i32 = 2;
pub const SOCK_NONBLOCK: i32 = 0o4000;
pub const SOCK_CLOEXEC: i32 = 0o40000;

pub const SOL_SOCKET: i32 = 4095;

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

pub const TCP_NODELAY: i32 = 1;

pub const EPERM: ::c_int = 1;
pub const ENOENT: ::c_int = 2;
pub const ESRCH: ::c_int = 3;
pub const EINTR: ::c_int = 4;
pub const EIO: ::c_int = 5;
pub const ENXIO: ::c_int = 6;
pub const E2BIG: ::c_int = 7;
pub const ENOEXEC: ::c_int = 8;
pub const EBADF: ::c_int = 9;
pub const ECHILD: ::c_int = 10;
pub const EAGAIN: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
pub const ENOTBLK: ::c_int = 15;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const EXDEV: ::c_int = 18;
pub const ENODEV: ::c_int = 19;
pub const ENOTDIR: ::c_int = 20;
pub const EISDIR: ::c_int = 21;
pub const EINVAL: ::c_int = 22;
pub const ENFILE: ::c_int = 23;
pub const EMFILE: ::c_int = 24;
pub const ENOTTY: ::c_int = 25;
pub const ETXTBSY: ::c_int = 26;
pub const EFBIG: ::c_int = 27;
pub const ENOSPC: ::c_int = 28;
pub const ESPIPE: ::c_int = 29;
pub const EROFS: ::c_int = 30;
pub const EMLINK: ::c_int = 31;
pub const EPIPE: ::c_int = 32;
pub const EDOM: ::c_int = 33;
pub const ERANGE: ::c_int = 34;
pub const EDEADLK: ::c_int = 35;
pub const ENAMETOOLONG: ::c_int = 36;
pub const ENOLCK: ::c_int = 37;
pub const ENOSYS: ::c_int = 38;
pub const ENOTEMPTY: ::c_int = 39;
pub const ELOOP: ::c_int = 40;
pub const EWOULDBLOCK: ::c_int = EAGAIN;
pub const ENOMSG: ::c_int = 42;
pub const EIDRM: ::c_int = 43;
pub const ECHRNG: ::c_int = 44;
pub const EL2NSYNC: ::c_int = 45;
pub const EL3HLT: ::c_int = 46;
pub const EL3RST: ::c_int = 47;
pub const ELNRNG: ::c_int = 48;
pub const EUNATCH: ::c_int = 49;
pub const ENOCSI: ::c_int = 50;
pub const EL2HLT: ::c_int = 51;
pub const EBADE: ::c_int = 52;
pub const EBADR: ::c_int = 53;
pub const EXFULL: ::c_int = 54;
pub const ENOANO: ::c_int = 55;
pub const EBADRQC: ::c_int = 56;
pub const EBADSLT: ::c_int = 57;
pub const EDEADLOCK: ::c_int = EDEADLK;
pub const EBFONT: ::c_int = 59;
pub const ENOSTR: ::c_int = 60;
pub const ENODATA: ::c_int = 61;
pub const ETIME: ::c_int = 62;
pub const ENOSR: ::c_int = 63;
pub const ENONET: ::c_int = 64;
pub const ENOPKG: ::c_int = 65;
pub const EREMOTE: ::c_int = 66;
pub const ENOLINK: ::c_int = 67;
pub const EADV: ::c_int = 68;
pub const ESRMNT: ::c_int = 69;
pub const ECOMM: ::c_int = 70;
pub const EPROTO: ::c_int = 71;
pub const EMULTIHOP: ::c_int = 72;
pub const EDOTDOT: ::c_int = 73;
pub const EBADMSG: ::c_int = 74;
pub const EOVERFLOW: ::c_int = 75;
pub const ENOTUNIQ: ::c_int = 76;
pub const EBADFD: ::c_int = 77;
pub const EREMCHG: ::c_int = 78;
pub const ELIBACC: ::c_int = 79;
pub const ELIBBAD: ::c_int = 80;
pub const ELIBSCN: ::c_int = 81;
pub const ELIBMAX: ::c_int = 82;
pub const ELIBEXEC: ::c_int = 83;
pub const EILSEQ: ::c_int = 84;
pub const ERESTART: ::c_int = 85;
pub const ESTRPIPE: ::c_int = 86;
pub const EUSERS: ::c_int = 87;
pub const ENOTSOCK: ::c_int = 88;
pub const EDESTADDRREQ: ::c_int = 89;
pub const EMSGSIZE: ::c_int = 90;
pub const EPROTOTYPE: ::c_int = 91;
pub const ENOPROTOOPT: ::c_int = 92;
pub const EPROTONOSUPPORT: ::c_int = 93;
pub const ESOCKTNOSUPPORT: ::c_int = 94;
pub const EOPNOTSUPP: ::c_int = 95;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const EAFNOSUPPORT: ::c_int = 97;
pub const EADDRINUSE: ::c_int = 98;
pub const EADDRNOTAVAIL: ::c_int = 99;
pub const ENETDOWN: ::c_int = 100;
pub const ENETUNREACH: ::c_int = 101;
pub const ENETRESET: ::c_int = 102;
pub const ECONNABORTED: ::c_int = 103;
pub const ECONNRESET: ::c_int = 104;
pub const ENOBUFS: ::c_int = 105;
pub const EISCONN: ::c_int = 106;
pub const ENOTCONN: ::c_int = 107;
pub const ESHUTDOWN: ::c_int = 108;
pub const ETOOMANYREFS: ::c_int = 109;
pub const ETIMEDOUT: ::c_int = 110;
pub const ECONNREFUSED: ::c_int = 111;
pub const EHOSTDOWN: ::c_int = 112;
pub const EHOSTUNREACH: ::c_int = 113;
pub const EALREADY: ::c_int = 114;
pub const EINPROGRESS: ::c_int = 115;
pub const ESTALE: ::c_int = 116;
pub const EUCLEAN: ::c_int = 117;
pub const ENOTNAM: ::c_int = 118;
pub const ENAVAIL: ::c_int = 119;
pub const EISNAM: ::c_int = 120;
pub const EREMOTEIO: ::c_int = 121;
pub const EDQUOT: ::c_int = 122;
pub const ENOMEDIUM: ::c_int = 123;
pub const EMEDIUMTYPE: ::c_int = 124;
pub const ECANCELED: ::c_int = 125;
pub const ENOKEY: ::c_int = 126;
pub const EKEYEXPIRED: ::c_int = 127;
pub const EKEYREVOKED: ::c_int = 128;
pub const EKEYREJECTED: ::c_int = 129;
pub const EOWNERDEAD: ::c_int = 130;
pub const ENOTRECOVERABLE: ::c_int = 131;
pub const ERFKILL: ::c_int = 132;
pub const EHWPOISON: ::c_int = 133;

extern "C" {
    #[link_name = "sys_alloc"]
    pub fn alloc(size: usize, align: usize) -> *mut u8;

    #[link_name = "sys_alloc_zeroed"]
    pub fn alloc_zeroed(size: usize, align: usize) -> *mut u8;

    #[link_name = "sys_realloc"]
    pub fn realloc(ptr: *mut u8, size: usize, align: usize, new_size: usize) -> *mut u8;

    #[link_name = "sys_dealloc"]
    pub fn dealloc(ptr: *mut u8, size: usize, align: usize);

    #[link_name = "sys_exit"]
    pub fn exit(status: i32) -> !;

    #[link_name = "sys_abort"]
    pub fn abort() -> !;

    #[link_name = "sys_errno"]
    pub fn errno() -> i32;

    #[link_name = "sys_clock_gettime"]
    pub fn clock_gettime(clockid: clockid_t, tp: *mut timespec) -> i32;

    #[link_name = "sys_nanosleep"]
    pub fn nanosleep(req: *const timespec) -> i32;

    #[link_name = "sys_available_parallelism"]
    pub fn available_parallelism() -> usize;

    #[link_name = "sys_futex_wait"]
    pub fn futex_wait(
        address: *mut u32,
        expected: u32,
        timeout: *const timespec,
        flags: u32,
    ) -> i32;

    #[link_name = "sys_futex_wake"]
    pub fn futex_wake(address: *mut u32, count: i32) -> i32;

    #[link_name = "sys_stat"]
    pub fn stat(path: *const c_char, stat: *mut stat) -> i32;

    #[link_name = "sys_fstat"]
    pub fn fstat(fd: i32, stat: *mut stat) -> i32;

    #[link_name = "sys_lstat"]
    pub fn lstat(path: *const c_char, stat: *mut stat) -> i32;

    #[link_name = "sys_open"]
    pub fn open(path: *const c_char, flags: i32, mode: mode_t) -> i32;

    #[link_name = "sys_unlink"]
    pub fn unlink(path: *const c_char) -> i32;

    #[link_name = "sys_mkdir"]
    pub fn mkdir(path: *const c_char, mode: mode_t) -> i32;

    #[link_name = "sys_rmdir"]
    pub fn rmdir(path: *const c_char) -> i32;

    #[link_name = "sys_read"]
    pub fn read(fd: i32, buf: *mut u8, len: usize) -> isize;

    #[link_name = "sys_write"]
    pub fn write(fd: i32, buf: *const u8, len: usize) -> isize;

    #[link_name = "sys_readv"]
    pub fn readv(fd: i32, iov: *const iovec, iovcnt: usize) -> isize;

    #[link_name = "sys_writev"]
    pub fn writev(fd: i32, iov: *const iovec, iovcnt: usize) -> isize;

    #[link_name = "sys_close"]
    pub fn close(fd: i32) -> i32;

    #[link_name = "sys_dup"]
    pub fn dup(fd: i32) -> i32;

    #[link_name = "sys_fcntl"]
    pub fn fcntl(fd: i32, cmd: i32, arg: i32) -> i32;

    #[link_name = "sys_getdents64"]
    pub fn getdents64(fd: i32, dirp: *mut dirent64, count: usize) -> isize;

    #[link_name = "sys_getaddrinfo"]
    pub fn getaddrinfo(
        nodename: *const c_char,
        servname: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> i32;

    #[link_name = "sys_freeaddrinfo"]
    pub fn freeaddrinfo(ai: *mut addrinfo);

    #[link_name = "sys_socket"]
    pub fn socket(domain: i32, ty: i32, protocol: i32) -> i32;

    #[link_name = "sys_bind"]
    pub fn bind(sockfd: i32, addr: *const sockaddr, addrlen: socklen_t) -> i32;

    #[link_name = "sys_listen"]
    pub fn listen(sockfd: i32, backlog: i32) -> i32;

    #[link_name = "sys_accept"]
    pub fn accept(sockfd: i32, addr: *mut sockaddr, addrlen: *mut socklen_t) -> i32;

    #[link_name = "sys_connect"]
    pub fn connect(sockfd: i32, addr: *const sockaddr, addrlen: socklen_t) -> i32;

    #[link_name = "sys_recv"]
    pub fn recv(sockfd: i32, buf: *mut u8, len: usize, flags: i32) -> isize;

    #[link_name = "sys_recvfrom"]
    pub fn recvfrom(
        sockfd: i32,
        buf: *mut c_void,
        len: usize,
        flags: i32,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> isize;

    #[link_name = "sys_send"]
    pub fn send(sockfd: i32, buf: *const c_void, len: usize, flags: i32) -> isize;

    #[link_name = "sys_sendto"]
    pub fn sendto(
        sockfd: i32,
        buf: *const c_void,
        len: usize,
        flags: i32,
        to: *const sockaddr,
        tolen: socklen_t,
    ) -> isize;

    #[link_name = "sys_getpeername"]
    pub fn getpeername(sockfd: i32, addr: *mut sockaddr, addrlen: *mut socklen_t) -> i32;

    #[link_name = "sys_getsockname"]
    pub fn getsockname(sockfd: i32, addr: *mut sockaddr, addrlen: *mut socklen_t) -> i32;

    #[link_name = "sys_getsockopt"]
    pub fn getsockopt(
        sockfd: i32,
        level: i32,
        optname: i32,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> i32;

    #[link_name = "sys_setsockopt"]
    pub fn setsockopt(
        sockfd: i32,
        level: i32,
        optname: i32,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> i32;

    #[link_name = "sys_ioctl"]
    pub fn ioctl(sockfd: i32, cmd: i32, argp: *mut c_void) -> i32;

    #[link_name = "sys_shutdown"]
    pub fn shutdown(sockfd: i32, how: i32) -> i32;

    #[link_name = "sys_eventfd"]
    pub fn eventfd(initval: u64, flags: i16) -> i32;

    #[link_name = "sys_poll"]
    pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: i32) -> i32;
}

pub use ffi::c_void;
