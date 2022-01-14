cfg_if! {
    if #[cfg(not(target_arch = "x86_64"))] {
        compile_error!("Aero is not ported to this architecture");
    }
}

// Basic data types
pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type wchar_t = ::c_int;

// options/posix/include/sys/resource.h
pub type rlim_t = ::c_ulong;

// abis/mlibc/mode_t.h
pub type mode_t = ::c_int;

// options/posix/include/bits/posix/socklen_t.h
pub type socklen_t = ::c_ulong;

// options/internal/include/bits/off_t.h
pub type off_t = ::c_long;

// options/ansi/include/time.h
pub type clock_t = ::c_ulong;

// options/ansi/include/bits/ansi/clockid_t.h
pub type clockid_t = ::c_long;

// options/ansi/include/bits/ansi/time_t.h
pub type time_t = ::c_long;

// options/posix/include/bits/posix/suseconds_t.h
pub type suseconds_t = ::c_long;

// abis/mlibc/dev_t.h
pub type dev_t = ::c_ulong;

// options/posix/include/bits/posix/fsblkcnt_t.h
pub type fsblkcnt_t = ::c_uint;

// options/posix/include/bits/posix/fsfilcnt_t.h
pub type fsfilcnt_t = ::c_uint;

// abis/mlibc/signal.h
pub type sigset_t = ::c_long;

// abis/mlibc/termios.h
pub type cc_t = ::c_uint;
pub type speed_t = ::c_uint;
pub type tcflag_t = ::c_uint;

// abis/mlibc/ino_t.h
pub type ino_t = ::c_long;

// abis/mlibc/blksize_t.h
pub type blksize_t = ::c_long;

// abis/mlibc/blkcnt_t.h
pub type blkcnt_t = ::c_long;

// abis/mlibc/nlink_t.h
pub type nlink_t = ::c_int;

// options/posix/include/bits/posix/in_addr_t.h
pub type in_addr_t = u32;

// options/posix/include/bits/posix/in_port_t.h
pub type in_port_t = u16;

// abis/mlibc/socket.h
pub type sa_family_t = ::c_uint;

// options/linux/include/sys/poll.h
pub type nfds_t = ::size_t;

// options/posix/include/bits/posix/pthread_t.h
pub type pthread_t = *mut __mlibc_thread_data;

// options/posix/include/bits/posix/pthread.h
pub type pthread_key_t = usize; // TODO: This is a big hack

s! {
    // options/ansi/include/time.h
    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
        pub tm_gmtoff: ::c_long,
        pub tm_zone: *const ::c_char,
    }

    // abis/mlibc/signal.h
    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_code: ::c_int,
        pub si_errno: ::c_int,
        pub si_pid: ::pid_t,
        pub si_uid: ::uid_t,
        pub si_addr: *mut ::c_void,
        pub si_status: ::c_int,
        pub si_value: sigval,
    }
    pub struct sigaction {
        pub sa_handler: ::Option<extern fn(::c_int)>,
        pub sa_mask: sigset_t,
        pub sa_flags: ::c_int,
        pub sa_sigaction: ::Option<extern fn(::c_int, *mut siginfo_t, *mut ::c_void)>,
    }

    // abis/mlibc/termios.h
    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_cc: [cc_t; NCCS],
        pub ibaud: speed_t,
        pub obaud: speed_t,
    }

    // options/posix/include/sys/statvfs.h
    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: fsblkcnt_t,
        pub f_bfree: fsblkcnt_t,
        pub f_bavail: fsblkcnt_t,
        pub f_files: fsfilcnt_t,
        pub f_ffree: fsfilcnt_t,
        pub f_favail: fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
    }

    // options/posix/include/dirent.h
    pub struct dirent {
        pub d_ino: ino_t,
        pub d_off: off_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 1024],
    }

    // options/ansi/include/bits/ansi/timespec.h
    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: ::c_long,
    }

    // options/posix/include/sys/un.h
    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [::c_char; 108],
    }

    // abis/mlibc/in.h
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub __padding: [u8; 8], // std relies on this being public
    }
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    // options/posix/include/dlfcn.h
    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    // abis/mlibc/socket.h
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __padding: [u8; 128 - ::mem::size_of::<sa_family_t>()],
    }

    // abis/mlibc/stat.h
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
    }

    // options/posix/include/pwd.h
    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }

    // options/posix/include/sys/socket.h
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    // options/posix/include/bits/posix/pthread_t.h
    pub struct __mlibc_thread_data {}

    // options/posix/include/bits/posix/pthread.h
    pub struct pthread_attr_t {
        pub __mlibc_deatchstate: ::c_int,
    }
    pub struct pthread_cond_t {
        pub __mlibc_seq: ::c_uint,
    }
    pub struct pthread_condattr_t {}
    pub struct pthread_mutex_t {
        pub __mlibc_state: ::c_uint,
        pub __mlibc_recursion: ::c_uint,
        pub __mlibc_flags: ::c_uint,
    }
    pub struct pthread_mutexattr_t {
        pub __mlibc_type: ::c_int,
        pub __mlibc_robust: ::c_int,
    }
    pub struct pthread_rwlock_t {
        pub __mlibc_m: ::c_uint,
        pub __mlibc_rc: ::c_uint,
        __mlibc_padding: ::c_uint,
    }
    pub struct pthread_rwlockattr_t {
        pub __mlibc_align: ::c_int,
    }

    // options/posix/include/netdb.h
    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: ::socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut ::c_char,
        pub ai_next: *mut addrinfo,
    }

    // options/ansi/include/locale.h
    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
        pub int_n_sign_posn: ::c_char,
    }

    // options/posix/include/semaphore.h
    pub struct sem_t {
        pub __mlibc_count: ::c_uint,
    }
}

// options/posix/include/sys/wait.h
safe_f! {
    pub {const} fn WCOREDUMP(x: ::c_int) -> bool {
        x & WCOREFLAG != 0
    }
    pub {const} fn WEXITSTATUS(x: ::c_int) -> ::c_int {
        x & 0xFF
    }
    pub {const} fn WIFCONTINUED(x: ::c_int) -> bool {
        x & 0x100 != 0
    }
    pub {const} fn WIFEXITED(x: ::c_int) -> bool {
        x & 0x200 != 0
    }
    pub {const} fn WIFSIGNALED(x: ::c_int) -> bool {
        x & 0x400 != 0
    }
    pub {const} fn WIFSTOPPED(x: ::c_int) -> bool {
        x & 0x800 != 0
    }
    pub {const} fn WSTOPSIG(x: ::c_int) -> ::c_int {
        (x & 0xFF_0000) >> 16
    }
    pub {const} fn WTERMSIG(x: ::c_int) -> ::c_int {
        (x & 0xFF00_0000) >> 24
    }
}

s_no_extra_traits! {
    // options/posix/include/bits/posix/fd_set.h
    pub union fd_set {
        pub __mlibc_elems: [c_char; 128],
        pub fds_bits: [c_char; 128],
    }

     // abis/mlibc/signal.h
     pub union sigval {
        pub sival_int: ::c_int,
        pub sival_ptr: *mut ::c_void,
    }
}

// abis/mlibc/vm-flags.h
pub const MAP_ANON: ::c_int = 8;
pub const MAP_PRIVATE: ::c_int = 1;
pub const MAP_SHARED: ::c_int = 2;
pub const PROT_EXEC: ::c_int = 4;
pub const PROT_READ: ::c_int = 1;
pub const PROT_WRITE: ::c_int = 2;

// options/posix/include/sys/mman.h
pub const MAP_FAILED: *mut ::c_void = usize::MAX as *mut ::c_void;
pub const MS_ASYNC: ::c_int = 1;
pub const MS_SYNC: ::c_int = 2;

// options/ansi/include/time.h
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_REALTIME: clockid_t = 0;

// options/posix/include/netdb.h
pub const EAI_SYSTEM: ::c_int = 9;

// abis/mlibc/in.h
pub const IPV6_ADD_MEMBERSHIP: ::c_int = 1;
pub const IPV6_DROP_MEMBERSHIP: ::c_int = 2;
pub const IPV6_MULTICAST_LOOP: ::c_int = 5;
pub const IPV6_V6ONLY: ::c_int = 7;
pub const IP_ADD_MEMBERSHIP: ::c_int = 35;
pub const IP_DROP_MEMBERSHIP: ::c_int = 36;
pub const IP_MULTICAST_LOOP: ::c_int = 34;
pub const IP_MULTICAST_TTL: ::c_int = 33;
pub const IP_TTL: ::c_int = 2;

// abis/mlibc/signal.h
pub const SIGKILL: ::c_int = 9;
pub const SIGPIPE: ::c_int = 13;
pub const SIG_SETMASK: ::c_int = 3;

// abis/mlibc/termios.h
pub const NCCS: usize = 11;

// options/posix/include/termios.h
pub const TIOCGWINSZ: ::c_ulong = 0x5413;

// options/ansi/include/stdlib.h
pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;

// options/posix/include/dlfcn.h
pub const RTLD_DEFAULT: *mut ::c_void = 0 as *mut ::c_void;

// options/posix/include/unistd.h
pub const STDERR_FILENO: ::c_int = 2;
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 1;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 6;
pub const _SC_PAGESIZE: ::c_int = _SC_PAGE_SIZE;
pub const _SC_PAGE_SIZE: ::c_int = 3;

// abis/mlibc/socket.h
pub const AF_INET6: ::c_int = PF_INET6;
pub const AF_INET: ::c_int = PF_INET;
pub const AF_UNIX: ::c_int = 3;
pub const MSG_PEEK: ::c_int = 0x20;
pub const PF_INET6: ::c_int = 2;
pub const PF_INET: ::c_int = 1;
pub const PF_UNIX: ::c_int = 3;
pub const SHUT_RD: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;
pub const SHUT_WR: ::c_int = 3;
pub const SOCK_DGRAM: ::c_int = 1;
pub const SOCK_STREAM: ::c_int = 4;
pub const SOL_SOCKET: ::c_int = 1;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_LINGER: ::c_int = 7;
pub const SO_ERROR: ::c_int = 5;
pub const SO_RCVTIMEO: ::c_int = 11;
pub const SO_REUSEADDR: ::c_int = 12;
pub const SO_SNDTIMEO: ::c_int = 15;

// abis/mlibc/errno.h
pub const EDOM: ::c_int = 1;
pub const EILSEQ: ::c_int = 2;
pub const ERANGE: ::c_int = 3;
pub const E2BIG: ::c_int = 1001;
pub const EACCES: ::c_int = 1002;
pub const EADDRINUSE: ::c_int = 1003;
pub const EADDRNOTAVAIL: ::c_int = 1004;
pub const EAFNOSUPPORT: ::c_int = 1005;
pub const EAGAIN: ::c_int = 1006;
pub const EALREADY: ::c_int = 1007;
pub const EBADF: ::c_int = 1008;
pub const EBADMSG: ::c_int = 1009;
pub const EBUSY: ::c_int = 1010;
pub const ECANCELED: ::c_int = 1011;
pub const ECHILD: ::c_int = 1012;
pub const ECONNABORTED: ::c_int = 1013;
pub const ECONNREFUSED: ::c_int = 1014;
pub const ECONNRESET: ::c_int = 1015;
pub const EDEADLK: ::c_int = 1016;
pub const EDESTADDRREQ: ::c_int = 1017;
pub const EDQUOT: ::c_int = 1018;
pub const EEXIST: ::c_int = 1019;
pub const EFAULT: ::c_int = 1020;
pub const EFBIG: ::c_int = 1021;
pub const EHOSTUNREACH: ::c_int = 1022;
pub const EIDRM: ::c_int = 1023;
pub const EINPROGRESS: ::c_int = 1024;
pub const EINTR: ::c_int = 1025;
pub const EINVAL: ::c_int = 1026;
pub const EIO: ::c_int = 1027;
pub const EISCONN: ::c_int = 1028;
pub const EISDIR: ::c_int = 1029;
pub const ELOOP: ::c_int = 1030;
pub const EMFILE: ::c_int = 1031;
pub const EMLINK: ::c_int = 1032;
pub const EMSGSIZE: ::c_int = 1034;
pub const EMULTIHOP: ::c_int = 1035;
pub const ENAMETOOLONG: ::c_int = 1036;
pub const ENETDOWN: ::c_int = 1037;
pub const ENETRESET: ::c_int = 1038;
pub const ENETUNREACH: ::c_int = 1039;
pub const ENFILE: ::c_int = 1040;
pub const ENOBUFS: ::c_int = 1041;
pub const ENODEV: ::c_int = 1042;
pub const ENOENT: ::c_int = 1043;
pub const ENOEXEC: ::c_int = 1044;
pub const ENOLCK: ::c_int = 1045;
pub const ENOLINK: ::c_int = 1046;
pub const ENOMEM: ::c_int = 1047;
pub const ENOMSG: ::c_int = 1048;
pub const ENOPROTOOPT: ::c_int = 1049;
pub const ENOSPC: ::c_int = 1050;
pub const ENOSYS: ::c_int = 1051;
pub const ENOTCONN: ::c_int = 1052;
pub const ENOTDIR: ::c_int = 1053;
pub const ENOTEMPTY: ::c_int = 1054;
pub const ENOTRECOVERABLE: ::c_int = 1055;
pub const ENOTSOCK: ::c_int = 1056;
pub const ENOTSUP: ::c_int = 1057;
pub const ENOTTY: ::c_int = 1058;
pub const ENXIO: ::c_int = 1059;
pub const EOPNOTSUPP: ::c_int = 1060;
pub const EOVERFLOW: ::c_int = 1061;
pub const EOWNERDEAD: ::c_int = 1062;
pub const EPERM: ::c_int = 1063;
pub const EPIPE: ::c_int = 1064;
pub const EPROTO: ::c_int = 1065;
pub const EPROTONOSUPPORT: ::c_int = 1066;
pub const EPROTOTYPE: ::c_int = 1067;
pub const EROFS: ::c_int = 1068;
pub const ESPIPE: ::c_int = 1069;
pub const ESRCH: ::c_int = 1070;
pub const ESTALE: ::c_int = 1071;
pub const ETIMEDOUT: ::c_int = 1072;
pub const ETXTBSY: ::c_int = 1073;
pub const EWOULDBLOCK: ::c_int = EAGAIN;
pub const EXDEV: ::c_int = 1075;
pub const ENODATA: ::c_int = 1076;
pub const ETIME: ::c_int = 1077;
pub const ENOKEY: ::c_int = 1078;
pub const ESHUTDOWN: ::c_int = 1079;
pub const EHOSTDOWN: ::c_int = 1080;
pub const EBADFD: ::c_int = 1081;
pub const ENOMEDIUM: ::c_int = 1082;
pub const ENOTBLK: ::c_int = 1083;

// options/posix/include/fcntl.h
pub const AT_FDCWD: ::c_int = -100;
pub const F_DUPFD_CLOEXEC: ::c_int = 2;
pub const F_GETFL: ::c_int = 5;
pub const F_SETFL: ::c_int = 6;
pub const O_ACCMODE: ::c_int = 7;
pub const O_APPEND: ::c_int = 8;
pub const O_CLOEXEC: ::c_int = 0x4000;
pub const O_CREAT: ::c_int = 0x10;
pub const O_EXCL: ::c_int = 0x40;
pub const O_NONBLOCK: ::c_int = 0x400;
pub const O_RDONLY: ::c_int = 2;
pub const O_RDWR: ::c_int = 3;
pub const O_TRUNC: ::c_int = 0x200;
pub const O_WRONLY: ::c_int = 5;

// options/mlibc/seek-whence.h
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
pub const SEEK_SET: ::c_int = 3;

// options/posix/include/netinet/tcp.h
pub const TCP_NODELAY: ::c_int = 1;

// abis/mlibc/stat.h
pub const S_IFBLK: mode_t = 0x6000;
pub const S_IFCHR: mode_t = 0x2000;
pub const S_IFDIR: mode_t = 0x4000;
pub const S_IFIFO: mode_t = 0x1000;
pub const S_IFLNK: mode_t = 0xA000;
pub const S_IFMT: mode_t = 0xF000;
pub const S_IFREG: mode_t = 0x8000;
pub const S_IFSOCK: mode_t = 0xC000;
pub const S_IRGRP: mode_t = 0o40;
pub const S_IROTH: mode_t = 0o4;
pub const S_IRUSR: mode_t = 0o400;
pub const S_IRWXG: mode_t = 0o70;
pub const S_IRWXO: mode_t = 0o7;
pub const S_IRWXU: mode_t = 0o700;
pub const S_IWGRP: mode_t = 0o20;
pub const S_IWOTH: mode_t = 0o2;
pub const S_IWUSR: mode_t = 0o200;
pub const S_IXGRP: mode_t = 0o10;
pub const S_IXOTH: mode_t = 0o1;
pub const S_IXUSR: mode_t = 0o100;

// options/posix/include/sys/wait.h
pub const WCOREFLAG: ::c_int = 0x80;
pub const WNOHANG: ::c_int = 2;

// options/linux/include/sys/poll.h
// TODO: Port epoll!
pub const POLLHUP: ::c_short = 8;
pub const POLLIN: ::c_short = 1;
pub const POLLNVAL: ::c_short = 0x40;
pub const POLLOUT: ::c_short = 2;

// options/glibc/include/sys/ioctl.h
pub const FIOCLEX: ::c_ulong = 0x5451;
pub const FIONBIO: ::c_ulong = 0x5421;

// options/ansi/include/limits.h
pub const PTHREAD_STACK_MIN: ::size_t = 16384;

extern "C" {
    pub fn bind(socket: ::c_int, address: *const ::sockaddr, address_len: ::socklen_t) -> ::c_int;
    pub fn clock_gettime(clk_id: clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_settime(clk_id: clockid_t, tp: *const ::timespec) -> ::c_int;
    pub fn endpwent();
    pub fn getpwent() -> *mut passwd;
    pub fn getgrgid_r(
        gid: ::gid_t,
        grp: *mut ::group,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut ::group,
    ) -> ::c_int;
    pub fn getgrnam_r(
        name: *const ::c_char,
        grp: *mut ::group,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut ::group,
    ) -> ::c_int;
    pub fn getgrouplist(
        user: *const ::c_char,
        group: ::gid_t,
        groups: *mut ::gid_t,
        ngroups: *mut ::c_int,
    ) -> ::c_int;
    pub fn getpwnam_r(
        name: *const ::c_char,
        pwd: *mut passwd,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;
    pub fn getpwuid_r(
        uid: ::uid_t,
        pwd: *mut passwd,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int) -> ::c_int;
    pub fn msync(addr: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::c_int;
    pub fn pthread_condattr_setclock(
        attr: *mut pthread_condattr_t,
        clock_id: ::clockid_t,
    ) -> ::c_int;
    pub fn pthread_create(
        thread: *mut ::pthread_t,
        attr: *const ::pthread_attr_t,
        f: extern "C" fn(*mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;
    pub fn pthread_setname_np(t: ::pthread_t, name: *const ::c_char) -> ::c_int;
    pub fn pthread_sigmask(how: ::c_int, set: *const sigset_t, oldset: *mut sigset_t) -> ::c_int;
    pub fn readv(fd: ::c_int, iov: *const ::iovec, count: ::c_int) -> ::ssize_t;
    pub fn recvfrom(
        socket: ::c_int,
        buf: *mut ::c_void,
        len: ::size_t,
        flags: ::c_int,
        addr: *mut ::sockaddr,
        addrlen: *mut ::socklen_t,
    ) -> ::ssize_t;
    pub fn setgroups(ngroups: ::c_int, ptr: *const ::gid_t) -> ::c_int;
    pub fn setpwent();
    pub fn writev(fd: ::c_int, iov: *const ::iovec, count: ::c_int) -> ::ssize_t;
}
