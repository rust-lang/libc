use dox::mem;

pub type sa_family_t = u16;
pub type pthread_key_t = ::c_uint;
pub type speed_t = ::c_uint;
pub type tcflag_t = ::c_uint;
pub type clockid_t = ::c_int;
pub type key_t = ::c_int;
pub type id_t = ::c_uint;

pub enum timezone {}

s! {
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [::c_char; 108]
    }

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_align: ::size_t,
        #[cfg(target_pointer_width = "32")]
        __ss_pad2: [u8; 128 - 2 * 4],
        #[cfg(target_pointer_width = "64")]
        __ss_pad2: [u8; 128 - 2 * 8],
    }

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: socklen_t,

        #[cfg(any(target_os = "linux",
                  target_os = "emscripten",
                  target_os = "fuchsia"))]
        pub ai_addr: *mut ::sockaddr,

        pub ai_canonname: *mut c_char,

        #[cfg(target_os = "android")]
        pub ai_addr: *mut ::sockaddr,

        pub ai_next: *mut addrinfo,
    }

    pub struct sockaddr_nl {
        pub nl_family: ::sa_family_t,
        nl_pad: ::c_ushort,
        pub nl_pid: u32,
        pub nl_groups: u32
    }

    pub struct sockaddr_ll {
        pub sll_family: ::c_ushort,
        pub sll_protocol: ::c_ushort,
        pub sll_ifindex: ::c_int,
        pub sll_hatype: ::c_ushort,
        pub sll_pkttype: ::c_uchar,
        pub sll_halen: ::c_uchar,
        pub sll_addr: [::c_uchar; 8]
    }

    pub struct fd_set {
        fds_bits: [::c_ulong; FD_SETSIZE / ULONG_SIZE],
    }

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

    pub struct sched_param {
        pub sched_priority: ::c_int,
        #[cfg(any(target_env = "musl", target_os = "emscripten"))]
        pub sched_ss_low_priority: ::c_int,
        #[cfg(any(target_env = "musl", target_os = "emscripten"))]
        pub sched_ss_repl_period: ::timespec,
        #[cfg(any(target_env = "musl", target_os = "emscripten"))]
        pub sched_ss_init_budget: ::timespec,
        #[cfg(any(target_env = "musl", target_os = "emscripten"))]
        pub sched_ss_max_repl: ::c_int,
    }

    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    #[cfg_attr(any(all(target_arch = "x86",
                       not(target_env = "musl"),
                       not(target_os = "android")),
                   target_arch = "x86_64"),
               repr(packed))]
    pub struct epoll_event {
        pub events: ::uint32_t,
        pub u64: ::uint64_t,
    }

    pub struct utsname {
        pub sysname: [::c_char; 65],
        pub nodename: [::c_char; 65],
        pub release: [::c_char; 65],
        pub version: [::c_char; 65],
        pub machine: [::c_char; 65],
        pub domainname: [::c_char; 65]
    }

    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
        pub int_n_sign_posn: ::c_char,
    }

    pub struct sigevent {
        pub sigev_value: ::sigval,
        pub sigev_signo: ::c_int,
        pub sigev_notify: ::c_int,
        // Actually a union.  We only expose sigev_notify_thread_id because it's
        // the most useful member
        pub sigev_notify_thread_id: ::c_int,
        #[cfg(target_pointer_width = "64")]
        __unused1: [::c_int; 11],
        #[cfg(target_pointer_width = "32")]
        __unused1: [::c_int; 12]
    }

    pub struct in_pktinfo {
        pub ipi_ifindex: ::c_int,
        pub ipi_spec_dst: ::in_addr,
        pub ipi_addr: ::in_addr,
    }

    pub struct ifaddrs {
        pub ifa_next: *mut ifaddrs,
        pub ifa_name: *mut c_char,
        pub ifa_flags: ::c_uint,
        pub ifa_addr: *mut ::sockaddr,
        pub ifa_netmask: *mut ::sockaddr,
        pub ifa_ifu: *mut ::sockaddr, // FIXME This should be a union
        pub ifa_data: *mut ::c_void
    }

    pub struct rtattr {
        rta_len: ::c_ushort,
        rta_type: ::c_ushort,
    }

    pub struct rtmsg {
        rtm_family: ::c_uchar,
        rtm_dst_len: ::c_uchar,
        rtm_src_len: ::c_uchar,
        rtm_tos: ::c_uchar,
        rtm_table: ::c_uchar,
        rtm_protocol: ::c_uchar,
        rtm_scope: ::c_uchar,
        rtm_type: ::c_uchar,
        rtm_flags: ::c_uint,
    }
    
    pub struct rtnexthop {
        rtnh_len: ::c_ushort,
        rtnh_flags: ::c_uchar,
        rtnh_hops: ::c_uchar,
        rtnh_ifindex: ::c_int,
    }

    pub struct rta_cacheinfo {
        rta_clntref: u32,
        rta_lastuse: u32,
        rta_expires: u32,
        rta_error: u32,
        rta_used: u32,
        rta_id: u32,
        rta_ts: u32,
        rta_tsage: u32,
    }

    pub struct rta_mfc_stats {
        mfcs_packets: u64,
        mfcs_bytes: u64,
        mfcs_wrong_if: u64,
    }

    pub struct rtgenmsg {
        rtgen_family: ::c_uchar,
    }

    pub struct ifinfomsg {
        ifi_family: ::c_uchar,
        __ifi_pad: ::c_uchar,
        ifi_type: ::c_ushort,
        ifi_index: ::c_int,
        ifi_flags: ::c_uint,
        ifi_change: ::c_uint,
    }

    pub struct prefixmsg {
        prefix_family: ::c_uchar,
        prefix_pad1: ::c_uchar,
        prefix_pad2: ::c_ushort,
        prefix_ifindex: ::c_int,
        prefix_type: ::c_uchar,
        prefix_len: ::c_uchar,
        prefix_flags: ::c_uchar,
        prefix_pad3: ::c_uchar,
    }

    pub struct prefix_cacheinfo {
        preferred_time: u32,
        valid_time: u32,
    }

    pub struct tcmsg {
        tcm_family: ::c_uchar,
        tcm__pad1: ::c_uchar,
        tcm__pad2: ::c_ushort,
        tcm_ifindex: ::c_int,
        tcm_handle: u32,
        tcm_parent: u32,
        tcm_info: u32,
    }

    pub struct nduseroptmsg {
        nduseropt_family: ::c_uchar,
        nduseropt_pad1: ::c_uchar,
        nduseropt_opts_len: ::c_ushort,
        nduseropt_ifindex: ::c_int,
        nduseropt_icmp_type: u8,
        nduseropt_icmp_code: u8,
        nduseropt_pad2: ::c_ushort,
        nduseropt_pad3: ::c_uint,
    }

    pub struct tcamsg {
        tca_family: ::c_uchar,
        tca_pad1: ::c_uchar,
        tca_pad2: ::c_ushort,
    }
}

// intentionally not public, only used for fd_set
cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        const ULONG_SIZE: usize = 32;
    } else if #[cfg(target_pointer_width = "64")] {
        const ULONG_SIZE: usize = 64;
    } else {
        // Unknown target_pointer_width
    }
}

pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;
pub const RAND_MAX: ::c_int = 2147483647;
pub const EOF: ::c_int = -1;
pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
pub const _IOFBF: ::c_int = 0;
pub const _IONBF: ::c_int = 2;
pub const _IOLBF: ::c_int = 1;

pub const F_DUPFD: ::c_int = 0;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;

// Linux-specific fcntls
pub const F_SETLEASE: ::c_int = 1024;
pub const F_GETLEASE: ::c_int = 1025;
pub const F_NOTIFY: ::c_int = 1026;
pub const F_CANCELLK: ::c_int = 1029;
pub const F_DUPFD_CLOEXEC: ::c_int = 1030;
pub const F_SETPIPE_SZ: ::c_int = 1031;
pub const F_GETPIPE_SZ: ::c_int = 1032;
pub const F_ADD_SEALS: ::c_int = 1033;
pub const F_GET_SEALS: ::c_int = 1034;

pub const F_SEAL_SEAL: ::c_int = 0x0001;
pub const F_SEAL_SHRINK: ::c_int = 0x0002;
pub const F_SEAL_GROW: ::c_int = 0x0004;
pub const F_SEAL_WRITE: ::c_int = 0x0008;

// TODO(#235): Include file sealing fcntls once we have a way to verify them.

pub const SIGTRAP: ::c_int = 5;

pub const PTHREAD_CREATE_JOINABLE: ::c_int = 0;
pub const PTHREAD_CREATE_DETACHED: ::c_int = 1;

pub const CLOCK_REALTIME: ::clockid_t = 0;
pub const CLOCK_MONOTONIC: ::clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: ::clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: ::clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW: ::clockid_t = 4;
pub const CLOCK_REALTIME_COARSE: ::clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: ::clockid_t = 6;
pub const CLOCK_BOOTTIME: ::clockid_t = 7;
pub const CLOCK_REALTIME_ALARM: ::clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM: ::clockid_t = 9;
// TODO(#247) Someday our Travis shall have glibc 2.21 (released in Sep
// 2014.) See also musl/mod.rs
// pub const CLOCK_SGI_CYCLE: ::clockid_t = 10;
// pub const CLOCK_TAI: ::clockid_t = 11;
pub const TIMER_ABSTIME: ::c_int = 1;

pub const RLIMIT_CPU: ::c_int = 0;
pub const RLIMIT_FSIZE: ::c_int = 1;
pub const RLIMIT_DATA: ::c_int = 2;
pub const RLIMIT_STACK: ::c_int = 3;
pub const RLIMIT_CORE: ::c_int = 4;
pub const RLIMIT_LOCKS: ::c_int = 10;
pub const RLIMIT_SIGPENDING: ::c_int = 11;
pub const RLIMIT_MSGQUEUE: ::c_int = 12;
pub const RLIMIT_NICE: ::c_int = 13;
pub const RLIMIT_RTPRIO: ::c_int = 14;

pub const RUSAGE_SELF: ::c_int = 0;

pub const O_RDONLY: ::c_int = 0;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDWR: ::c_int = 2;

pub const SOCK_CLOEXEC: ::c_int = O_CLOEXEC;

pub const S_IFIFO: ::mode_t = 4096;
pub const S_IFCHR: ::mode_t = 8192;
pub const S_IFBLK: ::mode_t = 24576;
pub const S_IFDIR: ::mode_t = 16384;
pub const S_IFREG: ::mode_t = 32768;
pub const S_IFLNK: ::mode_t = 40960;
pub const S_IFSOCK: ::mode_t = 49152;
pub const S_IFMT: ::mode_t = 61440;
pub const S_IRWXU: ::mode_t = 448;
pub const S_IXUSR: ::mode_t = 64;
pub const S_IWUSR: ::mode_t = 128;
pub const S_IRUSR: ::mode_t = 256;
pub const S_IRWXG: ::mode_t = 56;
pub const S_IXGRP: ::mode_t = 8;
pub const S_IWGRP: ::mode_t = 16;
pub const S_IRGRP: ::mode_t = 32;
pub const S_IRWXO: ::mode_t = 7;
pub const S_IXOTH: ::mode_t = 1;
pub const S_IWOTH: ::mode_t = 2;
pub const S_IROTH: ::mode_t = 4;
pub const F_OK: ::c_int = 0;
pub const R_OK: ::c_int = 4;
pub const W_OK: ::c_int = 2;
pub const X_OK: ::c_int = 1;
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;
pub const SIGHUP: ::c_int = 1;
pub const SIGINT: ::c_int = 2;
pub const SIGQUIT: ::c_int = 3;
pub const SIGILL: ::c_int = 4;
pub const SIGABRT: ::c_int = 6;
pub const SIGFPE: ::c_int = 8;
pub const SIGKILL: ::c_int = 9;
pub const SIGSEGV: ::c_int = 11;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGTERM: ::c_int = 15;

pub const PROT_NONE: ::c_int = 0;
pub const PROT_READ: ::c_int = 1;
pub const PROT_WRITE: ::c_int = 2;
pub const PROT_EXEC: ::c_int = 4;

pub const LC_CTYPE: ::c_int = 0;
pub const LC_NUMERIC: ::c_int = 1;
pub const LC_TIME: ::c_int = 2;
pub const LC_COLLATE: ::c_int = 3;
pub const LC_MONETARY: ::c_int = 4;
pub const LC_MESSAGES: ::c_int = 5;
pub const LC_ALL: ::c_int = 6;
pub const LC_CTYPE_MASK: ::c_int = (1 << LC_CTYPE);
pub const LC_NUMERIC_MASK: ::c_int = (1 << LC_NUMERIC);
pub const LC_TIME_MASK: ::c_int = (1 << LC_TIME);
pub const LC_COLLATE_MASK: ::c_int = (1 << LC_COLLATE);
pub const LC_MONETARY_MASK: ::c_int = (1 << LC_MONETARY);
pub const LC_MESSAGES_MASK: ::c_int = (1 << LC_MESSAGES);
// LC_ALL_MASK defined per platform

pub const MAP_FILE: ::c_int = 0x0000;
pub const MAP_SHARED: ::c_int = 0x0001;
pub const MAP_PRIVATE: ::c_int = 0x0002;
pub const MAP_FIXED: ::c_int = 0x0010;

pub const MAP_FAILED: *mut ::c_void = !0 as *mut ::c_void;

// MS_ flags for msync(2)
pub const MS_ASYNC: ::c_int = 0x0001;
pub const MS_INVALIDATE: ::c_int = 0x0002;
pub const MS_SYNC: ::c_int = 0x0004;

// MS_ flags for mount(2)
pub const MS_RDONLY: ::c_ulong = 0x01;
pub const MS_NOSUID: ::c_ulong = 0x02;
pub const MS_NODEV: ::c_ulong = 0x04;
pub const MS_NOEXEC: ::c_ulong = 0x08;
pub const MS_SYNCHRONOUS: ::c_ulong = 0x10;
pub const MS_REMOUNT: ::c_ulong = 0x20;
pub const MS_MANDLOCK: ::c_ulong = 0x40;
pub const MS_DIRSYNC: ::c_ulong = 0x80;
pub const MS_NOATIME: ::c_ulong = 0x0400;
pub const MS_NODIRATIME: ::c_ulong = 0x0800;
pub const MS_BIND: ::c_ulong = 0x1000;
pub const MS_MOVE: ::c_ulong = 0x2000;
pub const MS_REC: ::c_ulong = 0x4000;
pub const MS_SILENT: ::c_ulong = 0x8000;
pub const MS_POSIXACL: ::c_ulong = 0x010000;
pub const MS_UNBINDABLE: ::c_ulong = 0x020000;
pub const MS_PRIVATE: ::c_ulong = 0x040000;
pub const MS_SLAVE: ::c_ulong = 0x080000;
pub const MS_SHARED: ::c_ulong = 0x100000;
pub const MS_RELATIME: ::c_ulong = 0x200000;
pub const MS_KERNMOUNT: ::c_ulong = 0x400000;
pub const MS_I_VERSION: ::c_ulong = 0x800000;
pub const MS_STRICTATIME: ::c_ulong = 0x1000000;
pub const MS_ACTIVE: ::c_ulong = 0x40000000;
pub const MS_NOUSER: ::c_ulong = 0x80000000;
pub const MS_MGC_VAL: ::c_ulong = 0xc0ed0000;
pub const MS_MGC_MSK: ::c_ulong = 0xffff0000;
pub const MS_RMT_MASK: ::c_ulong = 0x800051;

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
pub const EWOULDBLOCK: ::c_int = EAGAIN;

pub const SCM_RIGHTS: ::c_int = 0x01;
pub const SCM_CREDENTIALS: ::c_int = 0x02;

pub const PROT_GROWSDOWN: ::c_int = 0x1000000;
pub const PROT_GROWSUP: ::c_int = 0x2000000;

pub const MAP_TYPE: ::c_int = 0x000f;

pub const MADV_NORMAL: ::c_int = 0;
pub const MADV_RANDOM: ::c_int = 1;
pub const MADV_SEQUENTIAL: ::c_int = 2;
pub const MADV_WILLNEED: ::c_int = 3;
pub const MADV_DONTNEED: ::c_int = 4;
pub const MADV_FREE: ::c_int = 8;
pub const MADV_REMOVE: ::c_int = 9;
pub const MADV_DONTFORK: ::c_int = 10;
pub const MADV_DOFORK: ::c_int = 11;
pub const MADV_MERGEABLE: ::c_int = 12;
pub const MADV_UNMERGEABLE: ::c_int = 13;
pub const MADV_HUGEPAGE: ::c_int = 14;
pub const MADV_NOHUGEPAGE: ::c_int = 15;
pub const MADV_DONTDUMP: ::c_int = 16;
pub const MADV_DODUMP: ::c_int = 17;
pub const MADV_HWPOISON: ::c_int = 100;
pub const MADV_SOFT_OFFLINE: ::c_int = 101;

pub const IFF_UP: ::c_int = 0x1;
pub const IFF_BROADCAST: ::c_int = 0x2;
pub const IFF_DEBUG: ::c_int = 0x4;
pub const IFF_LOOPBACK: ::c_int = 0x8;
pub const IFF_POINTOPOINT: ::c_int = 0x10;
pub const IFF_NOTRAILERS: ::c_int = 0x20;
pub const IFF_RUNNING: ::c_int = 0x40;
pub const IFF_NOARP: ::c_int = 0x80;
pub const IFF_PROMISC: ::c_int = 0x100;
pub const IFF_ALLMULTI: ::c_int = 0x200;
pub const IFF_MASTER: ::c_int = 0x400;
pub const IFF_SLAVE: ::c_int = 0x800;
pub const IFF_MULTICAST: ::c_int = 0x1000;
pub const IFF_PORTSEL: ::c_int = 0x2000;
pub const IFF_AUTOMEDIA: ::c_int = 0x4000;
pub const IFF_DYNAMIC: ::c_int = 0x8000;

pub const SOL_IP: ::c_int = 0;
pub const SOL_TCP: ::c_int = 6;
pub const SOL_UDP: ::c_int = 17;
pub const SOL_IPV6: ::c_int = 41;
pub const SOL_ICMPV6: ::c_int = 58;
pub const SOL_RAW: ::c_int = 255;
pub const SOL_DECNET: ::c_int = 261;
pub const SOL_X25: ::c_int = 262;
pub const SOL_PACKET: ::c_int = 263;
pub const SOL_ATM: ::c_int = 264;
pub const SOL_AAL: ::c_int = 265;
pub const SOL_IRDA: ::c_int = 266;
pub const SOL_NETBEUI: ::c_int = 267;
pub const SOL_LLC: ::c_int = 268;
pub const SOL_DCCP: ::c_int = 269;
pub const SOL_NETLINK: ::c_int = 270;
pub const SOL_TIPC: ::c_int = 271;

pub const AF_UNSPEC: ::c_int = 0;
pub const AF_UNIX: ::c_int = 1;
pub const AF_LOCAL: ::c_int = 1;
pub const AF_INET: ::c_int = 2;
pub const AF_AX25: ::c_int = 3;
pub const AF_IPX: ::c_int = 4;
pub const AF_APPLETALK: ::c_int = 5;
pub const AF_NETROM: ::c_int = 6;
pub const AF_BRIDGE: ::c_int = 7;
pub const AF_ATMPVC: ::c_int = 8;
pub const AF_X25: ::c_int = 9;
pub const AF_INET6: ::c_int = 10;
pub const AF_ROSE: ::c_int = 11;
pub const AF_DECnet: ::c_int = 12;
pub const AF_NETBEUI: ::c_int = 13;
pub const AF_SECURITY: ::c_int = 14;
pub const AF_KEY: ::c_int = 15;
pub const AF_NETLINK: ::c_int = 16;
pub const AF_ROUTE: ::c_int = AF_NETLINK;
pub const AF_PACKET: ::c_int = 17;
pub const AF_ASH: ::c_int = 18;
pub const AF_ECONET: ::c_int = 19;
pub const AF_ATMSVC: ::c_int = 20;
pub const AF_RDS: ::c_int = 21;
pub const AF_SNA: ::c_int = 22;
pub const AF_IRDA: ::c_int = 23;
pub const AF_PPPOX: ::c_int = 24;
pub const AF_WANPIPE: ::c_int = 25;
pub const AF_LLC: ::c_int = 26;
pub const AF_CAN: ::c_int = 29;
pub const AF_TIPC: ::c_int = 30;
pub const AF_BLUETOOTH: ::c_int = 31;
pub const AF_IUCV: ::c_int = 32;
pub const AF_RXRPC: ::c_int = 33;
pub const AF_ISDN: ::c_int = 34;
pub const AF_PHONET: ::c_int = 35;
pub const AF_IEEE802154: ::c_int = 36;
pub const AF_CAIF: ::c_int = 37;
pub const AF_ALG: ::c_int = 38;

pub const PF_UNSPEC: ::c_int = AF_UNSPEC;
pub const PF_UNIX: ::c_int = AF_UNIX;
pub const PF_LOCAL: ::c_int = AF_LOCAL;
pub const PF_INET: ::c_int = AF_INET;
pub const PF_AX25: ::c_int = AF_AX25;
pub const PF_IPX: ::c_int = AF_IPX;
pub const PF_APPLETALK: ::c_int = AF_APPLETALK;
pub const PF_NETROM: ::c_int = AF_NETROM;
pub const PF_BRIDGE: ::c_int = AF_BRIDGE;
pub const PF_ATMPVC: ::c_int = AF_ATMPVC;
pub const PF_X25: ::c_int = AF_X25;
pub const PF_INET6: ::c_int = AF_INET6;
pub const PF_ROSE: ::c_int = AF_ROSE;
pub const PF_DECnet: ::c_int = AF_DECnet;
pub const PF_NETBEUI: ::c_int = AF_NETBEUI;
pub const PF_SECURITY: ::c_int = AF_SECURITY;
pub const PF_KEY: ::c_int = AF_KEY;
pub const PF_NETLINK: ::c_int = AF_NETLINK;
pub const PF_ROUTE: ::c_int = AF_ROUTE;
pub const PF_PACKET: ::c_int = AF_PACKET;
pub const PF_ASH: ::c_int = AF_ASH;
pub const PF_ECONET: ::c_int = AF_ECONET;
pub const PF_ATMSVC: ::c_int = AF_ATMSVC;
pub const PF_RDS: ::c_int = AF_RDS;
pub const PF_SNA: ::c_int = AF_SNA;
pub const PF_IRDA: ::c_int = AF_IRDA;
pub const PF_PPPOX: ::c_int = AF_PPPOX;
pub const PF_WANPIPE: ::c_int = AF_WANPIPE;
pub const PF_LLC: ::c_int = AF_LLC;
pub const PF_CAN: ::c_int = AF_CAN;
pub const PF_TIPC: ::c_int = AF_TIPC;
pub const PF_BLUETOOTH: ::c_int = AF_BLUETOOTH;
pub const PF_IUCV: ::c_int = AF_IUCV;
pub const PF_RXRPC: ::c_int = AF_RXRPC;
pub const PF_ISDN: ::c_int = AF_ISDN;
pub const PF_PHONET: ::c_int = AF_PHONET;
pub const PF_IEEE802154: ::c_int = AF_IEEE802154;
pub const PF_CAIF: ::c_int = AF_CAIF;
pub const PF_ALG: ::c_int = AF_ALG;

pub const SOMAXCONN: ::c_int = 128;

pub const MSG_OOB: ::c_int = 1;
pub const MSG_PEEK: ::c_int = 2;
pub const MSG_DONTROUTE: ::c_int = 4;
pub const MSG_CTRUNC: ::c_int = 8;
pub const MSG_TRUNC: ::c_int = 0x20;
pub const MSG_DONTWAIT: ::c_int = 0x40;
pub const MSG_EOR: ::c_int = 0x80;
pub const MSG_WAITALL: ::c_int = 0x100;
pub const MSG_FIN: ::c_int = 0x200;
pub const MSG_SYN: ::c_int = 0x400;
pub const MSG_CONFIRM: ::c_int = 0x800;
pub const MSG_RST: ::c_int = 0x1000;
pub const MSG_ERRQUEUE: ::c_int = 0x2000;
pub const MSG_NOSIGNAL: ::c_int = 0x4000;
pub const MSG_MORE: ::c_int = 0x8000;
pub const MSG_WAITFORONE: ::c_int = 0x10000;
pub const MSG_FASTOPEN: ::c_int = 0x20000000;
pub const MSG_CMSG_CLOEXEC: ::c_int = 0x40000000;

pub const SCM_TIMESTAMP: ::c_int = SO_TIMESTAMP;

pub const SOCK_RAW: ::c_int = 3;
pub const SOCK_RDM: ::c_int = 4;
pub const IP_MULTICAST_IF: ::c_int = 32;
pub const IP_MULTICAST_TTL: ::c_int = 33;
pub const IP_MULTICAST_LOOP: ::c_int = 34;
pub const IP_TTL: ::c_int = 2;
pub const IP_HDRINCL: ::c_int = 3;
pub const IP_PKTINFO: ::c_int = 8;
pub const IP_ADD_MEMBERSHIP: ::c_int = 35;
pub const IP_DROP_MEMBERSHIP: ::c_int = 36;
pub const IP_TRANSPARENT: ::c_int = 19;
pub const IPV6_UNICAST_HOPS: ::c_int = 16;
pub const IPV6_MULTICAST_IF: ::c_int = 17;
pub const IPV6_MULTICAST_HOPS: ::c_int = 18;
pub const IPV6_MULTICAST_LOOP: ::c_int = 19;
pub const IPV6_ADD_MEMBERSHIP: ::c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: ::c_int = 21;
pub const IPV6_V6ONLY: ::c_int = 26;
pub const IPV6_RECVPKTINFO: ::c_int = 49;
pub const IPV6_PKTINFO: ::c_int = 50;

pub const TCP_NODELAY: ::c_int = 1;
pub const TCP_MAXSEG: ::c_int = 2;
pub const TCP_CORK: ::c_int = 3;
pub const TCP_KEEPIDLE: ::c_int = 4;
pub const TCP_KEEPINTVL: ::c_int = 5;
pub const TCP_KEEPCNT: ::c_int = 6;
pub const TCP_SYNCNT: ::c_int = 7;
pub const TCP_LINGER2: ::c_int = 8;
pub const TCP_DEFER_ACCEPT: ::c_int = 9;
pub const TCP_WINDOW_CLAMP: ::c_int = 10;
pub const TCP_INFO: ::c_int = 11;
pub const TCP_QUICKACK: ::c_int = 12;
pub const TCP_CONGESTION: ::c_int = 13;

pub const SO_DEBUG: ::c_int = 1;

pub const SHUT_RD: ::c_int = 0;
pub const SHUT_WR: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;

pub const LOCK_SH: ::c_int = 1;
pub const LOCK_EX: ::c_int = 2;
pub const LOCK_NB: ::c_int = 4;
pub const LOCK_UN: ::c_int = 8;

pub const SS_ONSTACK: ::c_int = 1;
pub const SS_DISABLE: ::c_int = 2;

pub const PATH_MAX: ::c_int = 4096;

pub const FD_SETSIZE: usize = 1024;

pub const EPOLLIN: ::c_int = 0x1;
pub const EPOLLPRI: ::c_int = 0x2;
pub const EPOLLOUT: ::c_int = 0x4;
pub const EPOLLRDNORM: ::c_int = 0x40;
pub const EPOLLRDBAND: ::c_int = 0x80;
pub const EPOLLWRNORM: ::c_int = 0x100;
pub const EPOLLWRBAND: ::c_int = 0x200;
pub const EPOLLMSG: ::c_int = 0x400;
pub const EPOLLERR: ::c_int = 0x8;
pub const EPOLLHUP: ::c_int = 0x10;
pub const EPOLLET: ::c_int = 0x80000000;

pub const EPOLL_CTL_ADD: ::c_int = 1;
pub const EPOLL_CTL_MOD: ::c_int = 3;
pub const EPOLL_CTL_DEL: ::c_int = 2;

pub const MNT_DETACH: ::c_int = 0x2;
pub const MNT_EXPIRE: ::c_int = 0x4;

pub const Q_GETFMT: ::c_int = 0x800004;
pub const Q_GETINFO: ::c_int = 0x800005;
pub const Q_SETINFO: ::c_int = 0x800006;
pub const QIF_BLIMITS: ::uint32_t = 1;
pub const QIF_SPACE: ::uint32_t = 2;
pub const QIF_ILIMITS: ::uint32_t = 4;
pub const QIF_INODES: ::uint32_t = 8;
pub const QIF_BTIME: ::uint32_t = 16;
pub const QIF_ITIME: ::uint32_t = 32;
pub const QIF_LIMITS: ::uint32_t = 5;
pub const QIF_USAGE: ::uint32_t = 10;
pub const QIF_TIMES: ::uint32_t = 48;
pub const QIF_ALL: ::uint32_t = 63;

pub const MNT_FORCE: ::c_int = 0x1;

pub const Q_SYNC: ::c_int = 0x800001;
pub const Q_QUOTAON: ::c_int = 0x800002;
pub const Q_QUOTAOFF: ::c_int = 0x800003;
pub const Q_GETQUOTA: ::c_int = 0x800007;
pub const Q_SETQUOTA: ::c_int = 0x800008;

pub const TCIOFF: ::c_int = 2;
pub const TCION: ::c_int = 3;
pub const TCOOFF: ::c_int = 0;
pub const TCOON: ::c_int = 1;
pub const TCIFLUSH: ::c_int = 0;
pub const TCOFLUSH: ::c_int = 1;
pub const TCIOFLUSH: ::c_int = 2;
pub const NL0: ::c_int  = 0x00000000;
pub const NL1: ::c_int  = 0x00000100;
pub const TAB0: ::c_int = 0x00000000;
pub const CR0: ::c_int  = 0x00000000;
pub const FF0: ::c_int  = 0x00000000;
pub const BS0: ::c_int  = 0x00000000;
pub const VT0: ::c_int  = 0x00000000;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VINTR: usize = 0;
pub const VQUIT: usize = 1;
pub const VLNEXT: usize = 15;
pub const IGNBRK: ::tcflag_t = 0x00000001;
pub const BRKINT: ::tcflag_t = 0x00000002;
pub const IGNPAR: ::tcflag_t = 0x00000004;
pub const PARMRK: ::tcflag_t = 0x00000008;
pub const INPCK: ::tcflag_t = 0x00000010;
pub const ISTRIP: ::tcflag_t = 0x00000020;
pub const INLCR: ::tcflag_t = 0x00000040;
pub const IGNCR: ::tcflag_t = 0x00000080;
pub const ICRNL: ::tcflag_t = 0x00000100;
pub const IXANY: ::tcflag_t = 0x00000800;
pub const IMAXBEL: ::tcflag_t = 0x00002000;
pub const OPOST: ::tcflag_t = 0x1;
pub const CS5: ::tcflag_t = 0x00000000;
pub const CRTSCTS: ::tcflag_t = 0x80000000;
pub const ECHO: ::tcflag_t = 0x00000008;
pub const OCRNL:  ::tcflag_t = 0o000010;
pub const ONOCR:  ::tcflag_t = 0o000020;
pub const ONLRET: ::tcflag_t = 0o000040;
pub const OFILL:  ::tcflag_t = 0o000100;
pub const OFDEL:  ::tcflag_t = 0o000200;

pub const CLONE_VM: ::c_int = 0x100;
pub const CLONE_FS: ::c_int = 0x200;
pub const CLONE_FILES: ::c_int = 0x400;
pub const CLONE_SIGHAND: ::c_int = 0x800;
pub const CLONE_PTRACE: ::c_int = 0x2000;
pub const CLONE_VFORK: ::c_int = 0x4000;
pub const CLONE_PARENT: ::c_int = 0x8000;
pub const CLONE_THREAD: ::c_int = 0x10000;
pub const CLONE_NEWNS: ::c_int = 0x20000;
pub const CLONE_SYSVSEM: ::c_int = 0x40000;
pub const CLONE_SETTLS: ::c_int = 0x80000;
pub const CLONE_PARENT_SETTID: ::c_int = 0x100000;
pub const CLONE_CHILD_CLEARTID: ::c_int = 0x200000;
pub const CLONE_DETACHED: ::c_int = 0x400000;
pub const CLONE_UNTRACED: ::c_int = 0x800000;
pub const CLONE_CHILD_SETTID: ::c_int = 0x01000000;
pub const CLONE_NEWUTS: ::c_int = 0x04000000;
pub const CLONE_NEWIPC: ::c_int = 0x08000000;
pub const CLONE_NEWUSER: ::c_int = 0x10000000;
pub const CLONE_NEWPID: ::c_int = 0x20000000;
pub const CLONE_NEWNET: ::c_int = 0x40000000;
pub const CLONE_IO: ::c_int = 0x80000000;
pub const CLONE_NEWCGROUP: ::c_int = 0x02000000;

pub const WNOHANG: ::c_int = 0x00000001;
pub const WUNTRACED: ::c_int = 0x00000002;
pub const WSTOPPED: ::c_int = WUNTRACED;
pub const WEXITED: ::c_int = 0x00000004;
pub const WCONTINUED: ::c_int = 0x00000008;
pub const WNOWAIT: ::c_int = 0x01000000;

// Options set using PTRACE_SETOPTIONS.
pub const PTRACE_O_TRACESYSGOOD: ::c_int = 0x00000001;
pub const PTRACE_O_TRACEFORK: ::c_int = 0x00000002;
pub const PTRACE_O_TRACEVFORK: ::c_int = 0x00000004;
pub const PTRACE_O_TRACECLONE: ::c_int = 0x00000008;
pub const PTRACE_O_TRACEEXEC: ::c_int = 0x00000010;
pub const PTRACE_O_TRACEVFORKDONE: ::c_int = 0x00000020;
pub const PTRACE_O_TRACEEXIT: ::c_int = 0x00000040;
pub const PTRACE_O_TRACESECCOMP: ::c_int = 0x00000080;
pub const PTRACE_O_EXITKILL: ::c_int = 0x00100000;
pub const PTRACE_O_SUSPEND_SECCOMP: ::c_int = 0x00200000;
pub const PTRACE_O_MASK: ::c_int = 0x003000ff;

// Wait extended result codes for the above trace options.
pub const PTRACE_EVENT_FORK: ::c_int = 1;
pub const PTRACE_EVENT_VFORK: ::c_int = 2;
pub const PTRACE_EVENT_CLONE: ::c_int = 3;
pub const PTRACE_EVENT_EXEC: ::c_int = 4;
pub const PTRACE_EVENT_VFORK_DONE: ::c_int = 5;
pub const PTRACE_EVENT_EXIT: ::c_int = 6;
pub const PTRACE_EVENT_SECCOMP: ::c_int = 7;
// PTRACE_EVENT_STOP was added to glibc in 2.26
// pub const PTRACE_EVENT_STOP: ::c_int = 128;

pub const __WNOTHREAD: ::c_int = 0x20000000;
pub const __WALL: ::c_int = 0x40000000;
pub const __WCLONE: ::c_int = 0x80000000;

pub const SPLICE_F_MOVE: ::c_uint = 0x01;
pub const SPLICE_F_NONBLOCK: ::c_uint = 0x02;
pub const SPLICE_F_MORE: ::c_uint = 0x04;
pub const SPLICE_F_GIFT: ::c_uint = 0x08;

pub const RTLD_LOCAL: ::c_int = 0;
pub const RTLD_LAZY: ::c_int = 1;

pub const POSIX_FADV_NORMAL: ::c_int = 0;
pub const POSIX_FADV_RANDOM: ::c_int = 1;
pub const POSIX_FADV_SEQUENTIAL: ::c_int = 2;
pub const POSIX_FADV_WILLNEED: ::c_int = 3;

pub const AT_FDCWD: ::c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 0x100;
pub const AT_REMOVEDIR: ::c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: ::c_int = 0x400;
pub const AT_NO_AUTOMOUNT: ::c_int = 0x800;
pub const AT_EMPTY_PATH: ::c_int = 0x1000;

pub const LOG_CRON: ::c_int = 9 << 3;
pub const LOG_AUTHPRIV: ::c_int = 10 << 3;
pub const LOG_FTP: ::c_int = 11 << 3;
pub const LOG_PERROR: ::c_int = 0x20;

pub const PIPE_BUF: usize = 4096;

pub const SI_LOAD_SHIFT: ::c_uint = 16;

pub const SIGEV_SIGNAL: ::c_int = 0;
pub const SIGEV_NONE: ::c_int = 1;
pub const SIGEV_THREAD: ::c_int = 2;

pub const P_ALL: idtype_t = 0;
pub const P_PID: idtype_t = 1;
pub const P_PGID: idtype_t = 2;

pub const UTIME_OMIT: c_long = 1073741822;
pub const UTIME_NOW: c_long = 1073741823;

pub const POLLIN: ::c_short = 0x1;
pub const POLLPRI: ::c_short = 0x2;
pub const POLLOUT: ::c_short = 0x4;
pub const POLLERR: ::c_short = 0x8;
pub const POLLHUP: ::c_short = 0x10;
pub const POLLNVAL: ::c_short = 0x20;
pub const POLLRDNORM: ::c_short = 0x040;
pub const POLLRDBAND: ::c_short = 0x080;

pub const IPTOS_LOWDELAY: u8 = 0x10;
pub const IPTOS_THROUGHPUT: u8 = 0x08;
pub const IPTOS_RELIABILITY: u8 = 0x04;
pub const IPTOS_MINCOST: u8 = 0x02;

pub const IPTOS_PREC_NETCONTROL: u8 = 0xe0;
pub const IPTOS_PREC_INTERNETCONTROL: u8 = 0xc0;
pub const IPTOS_PREC_CRITIC_ECP: u8 = 0xa0;
pub const IPTOS_PREC_FLASHOVERRIDE: u8 = 0x80;
pub const IPTOS_PREC_FLASH: u8 = 0x60;
pub const IPTOS_PREC_IMMEDIATE: u8 = 0x40;
pub const IPTOS_PREC_PRIORITY: u8 = 0x20;
pub const IPTOS_PREC_ROUTINE: u8 = 0x00;

pub const IPOPT_COPY: u8 = 0x80;
pub const IPOPT_CLASS_MASK: u8 = 0x60;
pub const IPOPT_NUMBER_MASK: u8 = 0x1f;

pub const IPOPT_CONTROL: u8 = 0x00;
pub const IPOPT_RESERVED1: u8 = 0x20;
pub const IPOPT_MEASUREMENT: u8 = 0x40;
pub const IPOPT_RESERVED2: u8 = 0x60;
pub const IPOPT_END: u8 = (0 |IPOPT_CONTROL);
pub const IPOPT_NOOP: u8 = (1 |IPOPT_CONTROL);
pub const IPOPT_SEC: u8 = (2 |IPOPT_CONTROL|IPOPT_COPY);
pub const IPOPT_LSRR: u8 = (3 |IPOPT_CONTROL|IPOPT_COPY);
pub const IPOPT_TIMESTAMP: u8 = (4 |IPOPT_MEASUREMENT);
pub const IPOPT_RR: u8 = (7 |IPOPT_CONTROL);
pub const IPOPT_SID: u8 = (8 |IPOPT_CONTROL|IPOPT_COPY);
pub const IPOPT_SSRR: u8 = (9 |IPOPT_CONTROL|IPOPT_COPY);
pub const IPOPT_RA: u8 = (20|IPOPT_CONTROL|IPOPT_COPY);
pub const IPVERSION: u8 = 4;
pub const MAXTTL: u8 = 255;
pub const IPDEFTTL: u8 = 64;
pub const IPOPT_OPTVAL: u8 = 0;
pub const IPOPT_OLEN: u8 = 1;
pub const IPOPT_OFFSET: u8 = 2;
pub const IPOPT_MINOFF: u8 = 4;
pub const MAX_IPOPTLEN: u8 = 40;
pub const IPOPT_NOP: u8 = IPOPT_NOOP;
pub const IPOPT_EOL: u8 = IPOPT_END;
pub const IPOPT_TS: u8 = IPOPT_TIMESTAMP;
pub const IPOPT_TS_TSONLY: u8 = 0;
pub const IPOPT_TS_TSANDADDR: u8 = 1;
pub const IPOPT_TS_PRESPEC: u8 = 3;

pub const RTNL_FAMILY_IPMR: u8 = 128;
pub const RTNL_FAMILY_IP6MR: u8 = 129;
pub const RTNL_FAMILY_MAX: u8 = 129;

pub const RTM_BASE: u16 = 16;
pub const RTM_DELLINK: u16 = 17;
pub const RTM_GETLINK: u16 = 18;
pub const RTM_SETLINK: u16 = 19;
pub const RTM_NEWADDR: u16 = 20;
pub const RTM_DELADDR: u16 = 21;
pub const RTM_GETADDR: u16 = 22;
pub const RTM_NEWROUTE: u16 = 24;
pub const RTM_DELROUTE: u16 = 25;
pub const RTM_GETROUTE: u16 = 26;
pub const RTM_NEWNEIGH: u16 = 28;
pub const RTM_DELNEIGH: u16 = 29;
pub const RTM_GETNEIGH: u16 = 30;
pub const RTM_NEWRULE: u16 = 32;
pub const RTM_DELRULE: u16 = 33;
pub const RTM_GETRULE: u16 = 34;
pub const RTM_NEWQDISC: u16 = 36;
pub const RTM_DELQDISC: u16 = 37;
pub const RTM_GETQDISC: u16 = 38;
pub const RTM_NEWTCLASS: u16 = 40;
pub const RTM_DELTCLASS: u16 = 41;
pub const RTM_GETTCLASS: u16 = 42;
pub const RTM_NEWTFILTER: u16 = 44;
pub const RTM_DELTFILTER: u16 = 45;
pub const RTM_GETTFILTER: u16 = 46;
pub const RTM_NEWACTION: u16 = 48;
pub const RTM_DELACTION: u16 = 49;
pub const RTM_GETACTION: u16 = 50;
pub const RTM_NEWPREFIX: u16 = 52;
pub const RTM_GETMULTICAST: u16 = 58;
pub const RTM_GETANYCAST: u16 = 62;
pub const RTM_NEWNEIGHTBL: u16 = 64;
pub const RTM_GETNEIGHTBL: u16 = 66;
pub const RTM_SETNEIGHTBL: u16 = 67;
pub const RTM_NEWNDUSEROPT: u16 = 68;
pub const RTM_NEWADDRLABEL: u16 = 72;
pub const RTM_DELADDRLABEL: u16 = 73;
pub const RTM_GETADDRLABEL: u16 = 74;
pub const RTM_GETDCB: u16 = 78;
pub const RTM_SETDCB: u16 = 79;
pub const RTM_NEWNETCONF: u16 = 80;
pub const RTM_GETNETCONF: u16 = 82;
pub const RTM_NEWMDB: u16 = 84;
pub const RTM_DELMDB: u16 = 85;
pub const RTM_GETMDB: u16 = 86;
pub const RTM_NEWNSID: u16 = 88;
pub const RTM_DELNSID: u16 = 89;
pub const RTM_GETNSID: u16 = 90;
pub const __RTM_MAX: u16 = 91;
pub const RTM_MAX: u16 = (((__RTM_MAX + 3) & !3) - 1);

pub const RTM_NR_MSGTYPES: u16 = RTM_MAX + 1 - RTM_BASE;
pub const RTM_NR_FAMILIES: u16 = RTM_NR_MSGTYPES >> 2;

pub const RTA_ALIGNTO: usize = 4;

pub const RTN_UNSPEC: u8 = 0;
pub const RTN_UNICAST: u8 = 1;
pub const RTN_LOCAL: u8 = 2;
pub const RTN_BROADCAST: u8 = 3;
pub const RTN_ANYCAST: u8 = 4;
pub const RTN_MULTICAST: u8 = 5;
pub const RTN_BLACKHOLE: u8 = 6;
pub const RTN_UNREACHABLE: u8 = 7;
pub const RTN_PROHIBIT: u8 = 8;
pub const RTN_THROW: u8 = 9;
pub const RTN_NAT: u8 = 10;
pub const RTN_XRESOLVE: u8 = 11;
pub const __RTN_MAX: u8 = 12;
pub const RTN_MAX: u8 = __RTN_MAX - 1;

pub const RTPROT_UNSPEC: u8 = 0;
pub const RTPROT_REDIRECT: u8 = 1;
pub const RTPROT_KERNEL: u8 = 2;
pub const RTPROT_BOOT: u8 = 3;
pub const RTPROT_STATIC: u8 = 4;
pub const RTPROT_GATED: u8 = 8;
pub const RTPROT_RA: u8 = 9;
pub const RTPROT_MRT: u8 = 10;
pub const RTPROT_ZEBRA: u8 = 11;
pub const RTPROT_BIRD: u8 = 12;
pub const RTPROT_DNROUTED: u8 = 13;
pub const RTPROT_XORP: u8 = 14;
pub const RTPROT_NTK: u8 = 15;
pub const RTPROT_DHCP: u8 = 16;
pub const RTPROT_MROUTED: u8 = 17;
pub const RTPROT_BABEL: u8 = 42;

pub const RT_SCOPE_UNIVERSE: u8 = 0;
pub const RT_SCOPE_SITE: u8 = 200;
pub const RT_SCOPE_LINK: u8 = 253;
pub const RT_SCOPE_HOST: u8 = 254;
pub const RT_SCOPE_NOWHERE: u8 = 25;

pub const RTM_F_NOTIFY: ::c_uint = 0x100;
pub const RTM_F_CLONED: ::c_uint = 0x200;
pub const RTM_F_EQUALIZE: ::c_uint = 0x400;
pub const RTM_F_PREFIX: ::c_uint = 0x800;
pub const RTM_F_LOOKUP_TABLE: ::c_uint = 0x1000;
pub const RTM_F_FIB_MATCH: ::c_uint = 0x2000;

pub const RT_TABLE_UNSPEC: u32 = 0;
pub const RT_TABLE_COMPAT: u32 = 252;
pub const RT_TABLE_DEFAULT: u32 = 253;
pub const RT_TABLE_MAIN: u32 = 254;
pub const RT_TABLE_LOCAL: u32 = 255;
pub const RT_TABLE_MAX: u32 = 0xFFFFFFF;

pub const RTA_UNSPEC: ::c_ushort = 0;
pub const RTA_DST: ::c_ushort = 1;
pub const RTA_SRC: ::c_ushort = 2;
pub const RTA_IIF: ::c_ushort = 3;
pub const RTA_OIF: ::c_ushort = 4;
pub const RTA_GATEWAY: ::c_ushort = 5;
pub const RTA_PRIORITY: ::c_ushort = 6;
pub const RTA_PREFSRC: ::c_ushort = 7;
pub const RTA_METRICS: ::c_ushort = 8;
pub const RTA_MULTIPATH: ::c_ushort = 9;
pub const RTA_PROTOINFO: ::c_ushort = 10;
pub const RTA_FLOW: ::c_ushort = 11;
pub const RTA_CACHEINFO: ::c_ushort = 12;
pub const RTA_SESSION: ::c_ushort = 13;
pub const RTA_MP_ALGO: ::c_ushort = 14;
pub const RTA_TABLE: ::c_ushort = 15;
pub const RTA_MARK: ::c_ushort = 16;
pub const RTA_MFC_STATS: ::c_ushort = 17;
pub const RTA_VIA: ::c_ushort = 18;
pub const RTA_NEWDST: ::c_ushort = 19;
pub const RTA_PREF: ::c_ushort = 20;
pub const RTA_ENCAP_TYPE: ::c_ushort = 21;
pub const RTA_ENCAP: ::c_ushort = 22;
pub const RTA_EXPIRES: ::c_ushort = 23;
pub const RTA_PAD: ::c_ushort = 24;
pub const RTA_UID: ::c_ushort = 25;
pub const RTA_TTL_PROPAGATE: ::c_ushort = 26;
pub const __RTA_MAX: ::c_ushort = 27;
pub const RTA_MAX: ::c_ushort = __RTA_MAX - 1;

pub const RTNH_F_DEAD: ::c_uchar = 1;
pub const RTNH_F_PERVASIVE: ::c_uchar = 2;
pub const RTNH_F_ONLINK: ::c_uchar = 4;
pub const RTNH_F_OFFLOAD: ::c_uchar = 8;
pub const RTNH_F_LINKDOWN: ::c_uchar = 16;
pub const RTNH_F_UNRESOLVED: ::c_uchar = 32;
pub const RTNH_COMPARE_MASK: ::c_uchar = (RTNH_F_DEAD | RTNH_F_LINKDOWN | RTNH_F_OFFLOAD);
pub const RTNH_ALIGNTO: usize = 4;
pub const RTNETLINK_HAVE_PEERINFO: bool = true;

pub const RTAX_UNSPEC: ::c_ushort = 0;
pub const RTAX_LOCK: ::c_ushort = 1;
pub const RTAX_MTU: ::c_ushort = 2;
pub const RTAX_WINDOW: ::c_ushort = 3;
pub const RTAX_RTT: ::c_ushort = 4;
pub const RTAX_RTTVAR: ::c_ushort = 5;
pub const RTAX_SSTHRESH: ::c_ushort = 6;
pub const RTAX_CWND: ::c_ushort = 7;
pub const RTAX_ADVMSS: ::c_ushort = 8;
pub const RTAX_REORDERING: ::c_ushort = 9;
pub const RTAX_HOPLIMIT: ::c_ushort = 10;
pub const RTAX_INITCWND: ::c_ushort = 11;
pub const RTAX_FEATURES: ::c_ushort = 12;
pub const RTAX_RTO_MIN: ::c_ushort = 13;
pub const RTAX_INITRWND: ::c_ushort = 14;
pub const RTAX_QUICKACK: ::c_ushort = 15;
pub const RTAX_CC_ALGO: ::c_ushort = 16;
pub const RTAX_FASTOPEN_NO_COOKIE: ::c_ushort = 17;
pub const __RTAX_MAX: ::c_ushort = 18;
pub const RTAX_MAX: ::c_ushort = __RTAX_MAX - 1;

pub const RTAX_FEATURE_ECN: u32 = 1 << 0;
pub const RTAX_FEATURE_SACK: u32 = 1 << 1;
pub const RTAX_FEATURE_TIMESTAMP: u32 = 1 << 2;
pub const RTAX_FEATURE_ALLFRAG: u32 = 1 << 3;
pub const RTAX_FEATURE_MASK: u32 =
    RTAX_FEATURE_ECN | RTAX_FEATURE_SACK | RTAX_FEATURE_TIMESTAMP | RTAX_FEATURE_ALLFRAG;

pub const PREFIX_UNSPEC: ::c_uchar = 0;
pub const PREFIX_ADDRESS: ::c_uchar = 1;
pub const PREFIX_CACHEINFO: ::c_uchar = 2;
pub const __PREFIX_MAX: ::c_uchar = 3;
pub const PREFIX_MAX: ::c_uchar = __PREFIX_MAX - 1;

pub const TCA_UNSPEC: ::c_uchar = 0;
pub const TCA_KIND: ::c_uchar = 1;
pub const TCA_OPTIONS: ::c_uchar = 2;
pub const TCA_STATS: ::c_uchar = 3;
pub const TCA_XSTATS: ::c_uchar = 4;
pub const TCA_RATE: ::c_uchar = 5;
pub const TCA_FCNT: ::c_uchar = 6;
pub const TCA_STATS2: ::c_uchar = 7;
pub const TCA_STAB: ::c_uchar = 8;
pub const TCA_PAD: ::c_uchar = 9;
pub const TCA_DUMP_INVISIBLE: ::c_uchar = 10;
pub const TCA_CHAIN: ::c_uchar = 11;
pub const TCA_HW_OFFLOAD: ::c_uchar = 12;
pub const __TCA_MAX: ::c_uchar = 13;
pub const TCA_MAX: ::c_uchar = __TCA_MAX - 1;

pub const NDUSEROPT_UNSPEC: ::c_uchar = 0;
pub const NDUSEROPT_SRCADDR: ::c_uchar = 1;
pub const __NDUSEROPT_MAX: ::c_uchar = 2;
pub const NDUSEROPT_MAX: ::c_uchar = __NDUSEROPT_MAX - 1;

pub const RTMGRP_LINK: u32 = 1;
pub const RTMGRP_NOTIFY: u32 = 2;
pub const RTMGRP_NEIGH: u32 = 4;
pub const RTMGRP_TC: u32 = 8;
pub const RTMGRP_IPV4_IFADDR: u32 = 0x10;
pub const RTMGRP_IPV4_MROUTE: u32 = 0x20;
pub const RTMGRP_IPV4_ROUTE: u32 = 0x40;
pub const RTMGRP_IPV4_RULE: u32 = 0x80;
pub const RTMGRP_IPV6_IFADDR: u32 = 0x100;
pub const RTMGRP_IPV6_MROUTE: u32 = 0x200;
pub const RTMGRP_IPV6_ROUTE: u32 = 0x400;
pub const RTMGRP_IPV6_IFINFO: u32 = 0x800;
pub const RTMGRP_DECnet_IFADDR: u32 = 0x1000;
pub const RTMGRP_DECnet_ROUTE: u32 = 0x4000;
pub const RTMGRP_IPV6_PREFIX: u32 = 0x20000;

pub const RTNLGRP_NONE: ::c_int = 0;
pub const RTNLGRP_LINK: ::c_int = 1;
pub const RTNLGRP_NOTIFY: ::c_int = 2;
pub const RTNLGRP_NEIGH: ::c_int = 3;
pub const RTNLGRP_TC: ::c_int = 4;
pub const RTNLGRP_IPV4_IFADDR: ::c_int = 5;
pub const RTNLGRP_IPV4_MROUTE: ::c_int = 6;
pub const RTNLGRP_IPV4_ROUTE: ::c_int = 7;
pub const RTNLGRP_IPV4_RULE: ::c_int = 8;
pub const RTNLGRP_IPV6_IFADDR: ::c_int = 9;
pub const RTNLGRP_IPV6_MROUTE: ::c_int = 10;
pub const RTNLGRP_IPV6_ROUTE: ::c_int = 11;
pub const RTNLGRP_IPV6_IFINFO: ::c_int = 12;
pub const RTNLGRP_DECnet_IFADDR: ::c_int = 13;
pub const RTNLGRP_NOP2: ::c_int = 14;
pub const RTNLGRP_DECnet_ROUTE: ::c_int = 15;
pub const RTNLGRP_DECnet_RULE: ::c_int = 16;
pub const RTNLGRP_NOP4: ::c_int = 17;
pub const RTNLGRP_IPV6_PREFIX: ::c_int = 18;
pub const RTNLGRP_IPV6_RULE: ::c_int = 19;
pub const RTNLGRP_ND_USEROPT: ::c_int = 20;
pub const RTNLGRP_PHONET_IFADDR: ::c_int = 21;
pub const RTNLGRP_PHONET_ROUTE: ::c_int = 22;
pub const RTNLGRP_DCB: ::c_int = 23;
pub const RTNLGRP_IPV4_NETCONF: ::c_int = 24;
pub const RTNLGRP_IPV6_NETCONF: ::c_int = 25;
pub const RTNLGRP_MDB: ::c_int = 26;
pub const RTNLGRP_MPLS_ROUTE: ::c_int = 27;
pub const RTNLGRP_NSID: ::c_int = 28;
pub const RTNLGRP_MPLS_NETCONF: ::c_int = 29;
pub const RTNLGRP_IPV4_MROUTE_R: ::c_int = 30;
pub const RTNLGRP_IPV6_MROUTE_R: ::c_int = 31;
pub const __RTNLGRP_MAX: ::c_int = 32;
pub const RTNLGRP_MAX: ::c_int = __RTNLGRP_MAX - 1;

pub const TCA_ROOT_UNSPEC: ::c_int = 0;
pub const TCA_ROOT_TAB: ::c_int = 1;
pub const TCA_ROOT_FLAGS: ::c_int = 2;
pub const TCA_ROOT_COUNT: ::c_int = 3;
pub const TCA_ROOT_TIME_DELTA: ::c_int = 4;
pub const __TCA_ROOT_MAX: ::c_int = 5;
pub const TCA_ROOT_MAX: ::c_int = __TCA_ROOT_MAX - 1;
pub const TCA_ACT_TAB: ::c_int = TCA_ROOT_TAB;
pub const TCAA_MAX: ::c_int = TCA_ROOT_TAB;

pub const TCA_FLAG_LARGE_DUMP_ON: u32 = 1 << 0;

pub const RTEXT_FILTER_VF: u32 = 1 << 0;
pub const RTEXT_FILTER_BRVLAN: u32 = 1 << 1;
pub const RTEXT_FILTER_BRVLAN_COMPRESSED: u32 = 1 << 2;
pub const RTEXT_FILTER_SKIP_STATS: u32 = 1 << 3;

f! {
    pub fn FD_CLR(fd: ::c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        (*set).fds_bits[fd / size] &= !(1 << (fd % size));
        return
    }

    pub fn FD_ISSET(fd: ::c_int, set: *mut fd_set) -> bool {
        let fd = fd as usize;
        let size = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        return ((*set).fds_bits[fd / size] & (1 << (fd % size))) != 0
    }

    pub fn FD_SET(fd: ::c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        (*set).fds_bits[fd / size] |= 1 << (fd % size);
        return
    }

    pub fn FD_ZERO(set: *mut fd_set) -> () {
        for slot in (*set).fds_bits.iter_mut() {
            *slot = 0;
        }
    }

    pub fn WIFSTOPPED(status: ::c_int) -> bool {
        (status & 0xff) == 0x7f
    }

    pub fn WSTOPSIG(status: ::c_int) -> ::c_int {
        (status >> 8) & 0xff
    }

    pub fn WIFCONTINUED(status: ::c_int) -> bool {
        status == 0xffff
    }

    pub fn WIFSIGNALED(status: ::c_int) -> bool {
        ((status & 0x7f) + 1) as i8 >= 2
    }

    pub fn WTERMSIG(status: ::c_int) -> ::c_int {
        status & 0x7f
    }

    pub fn WIFEXITED(status: ::c_int) -> bool {
        (status & 0x7f) == 0
    }

    pub fn WEXITSTATUS(status: ::c_int) -> ::c_int {
        (status >> 8) & 0xff
    }

    pub fn WCOREDUMP(status: ::c_int) -> bool {
        (status & 0x80) != 0
    }

    pub fn QCMD(cmd: ::c_int, type_: ::c_int) -> ::c_int {
        (cmd << 8) | (type_ & 0x00ff)
    }

    pub fn IPOPT_COPIED(o: u8) -> u8 {
        o & IPOPT_COPY
    }

    pub fn IPOPT_CLASS(o: u8) -> u8 {
        o & IPOPT_CLASS_MASK
    }

    pub fn IPOPT_NUMBER(o: u8) -> u8 {
        o & IPOPT_NUMBER_MASK
    }

    pub fn RTM_FAM(cmd: u16) -> u16 {
        (cmd - RTM_BASE) >> 2
    }

    pub fn RTA_ALIGN(len: usize) -> usize {
        (len + RTA_ALIGNTO - 1) & !(RTA_ALIGNTO - 1)
    }

    pub fn RTA_OK(rta: &rtattr, len: usize) -> bool {
        len >= mem::size_of::<rtattr>() &&
        (rta.rta_len as usize) >= mem::size_of::<rtattr>() &&
        (rta.rta_len as usize) <= len
    }

    pub fn RTA_NEXT(rta: &rtattr, attrlen: &mut usize) -> *const rtattr {
        *attrlen -= RTA_ALIGN(rta.rta_len as usize);
        let rta_ptr = rta as *const rtattr as *const u8;
        let rta_ptr = rta_ptr.offset(RTA_ALIGN(rta.rta_len as usize) as isize);
        rta_ptr as *const rtattr
    }

    pub fn RTA_LENGTH(len: usize) -> usize {
        RTA_ALIGN(mem::size_of::<rtattr>()) + len
    }

    pub fn RTA_SPACE(len: usize) -> usize {
        RTA_ALIGN(RTA_LENGTH(len))
    }

    pub fn RTA_DATA(rta: *const rtattr) -> *const ::c_void {
        (rta as *const u8).offset(RTA_LENGTH(0) as isize) as *const ::c_void
    }

    pub fn RTA_PAYLOAD(rta: &rtattr) -> usize {
        rta.rta_len as usize - RTA_LENGTH(0)
    }

    pub fn RTM_RTA(r: *const rtmsg) -> *const rtattr {
        let r_ptr = r as *const u8;
        let r_ptr = r_ptr.offset(NLMSG_ALIGN(mem::size_of::<rtmsg>()) as isize);
        r_ptr as *const rtattr
    }

    pub fn RTM_PAYLOAD(n: &nlmsghdr) -> usize {
        NLMSG_PAYLOAD(n, mem::size_of::<rtmsg>())
    }

    pub fn RTNH_ALIGN(len: usize) -> usize {
        (len + RTNH_ALIGNTO - 1) & !(RTNH_ALIGNTO - 1)
    }

    pub fn RTNH_OK(rtnh: &rtnexthop, len: usize) -> bool {
        rtnh.rtnh_len as usize >= mem::size_of::<rtnexthop>() &&
        rtnh.rtnh_len as usize <= len
    }

    pub fn RTNH_NEXT(rtnh: &rtnexthop) -> *const rtnexthop {
        let rtnh_ptr = rtnh as *const rtnexthop as *const u8;
        let rtnh_ptr = rtnh_ptr.offset(RTNH_ALIGN(rtnh.rtnh_len as usize) as isize);
        rtnh_ptr as *const rtnexthop
    }

    pub fn RTNH_LENGTH(len: usize) -> usize {
        RTNH_ALIGN(mem::size_of::<rtnexthop>()) + len
    }

    pub fn RTNH_SPACE(len: usize) -> usize {
        RTNH_ALIGN(RTNH_LENGTH(len))
    }

    pub fn RTNH_DATA(rtnh: *const rtnexthop) -> *const rtattr {
        let rtnh_ptr = rtnh as *const u8;
        let rtnh_ptr = rtnh_ptr.offset(RTNH_LENGTH(0) as isize);
        rtnh_ptr as *const rtattr
    }

    pub fn TCA_RTA(r: *const tcmsg) -> *const rtattr {
        let r_ptr = r as *const u8;
        let r_ptr = r_ptr.offset(NLMSG_ALIGN(mem::size_of::<tcmsg>()) as isize);
        r_ptr as *const rtattr
    }

    pub fn TCA_PAYLOAD(n: &nlmsghdr) -> usize {
        NLMSG_PAYLOAD(n, mem::size_of::<tcmsg>())
    }

    pub fn TA_RTA(r: *const tcamsg) -> *const rtattr {
        let r_ptr = r as *const u8;
        let r_ptr = r_ptr.offset(NLMSG_ALIGN(mem::size_of::<tcamsg>()) as isize);
        r_ptr as *const rtattr
    }

    pub fn TA_PAYLOAD(n: &nlmsghdr) -> usize {
        NLMSG_PAYLOAD(n, mem::size_of::<tcamsg>())
    }
}

extern {
    pub fn fdatasync(fd: ::c_int) -> ::c_int;
    pub fn mincore(addr: *mut ::c_void, len: ::size_t,
                   vec: *mut ::c_uchar) -> ::c_int;
    pub fn clock_getres(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_gettime(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_settime(clk_id: ::clockid_t, tp: *const ::timespec) -> ::c_int;
    pub fn dirfd(dirp: *mut ::DIR) -> ::c_int;

    pub fn pthread_getattr_np(native: ::pthread_t,
                              attr: *mut ::pthread_attr_t) -> ::c_int;
    pub fn pthread_attr_getstack(attr: *const ::pthread_attr_t,
                                 stackaddr: *mut *mut ::c_void,
                                 stacksize: *mut ::size_t) -> ::c_int;
    pub fn memalign(align: ::size_t, size: ::size_t) -> *mut ::c_void;
    pub fn setgroups(ngroups: ::size_t,
                     ptr: *const ::gid_t) -> ::c_int;
    pub fn pipe2(fds: *mut ::c_int, flags: ::c_int) -> ::c_int;
    pub fn statfs(path: *const ::c_char, buf: *mut statfs) -> ::c_int;
    pub fn statfs64(path: *const ::c_char, buf: *mut statfs64) -> ::c_int;
    pub fn fstatfs(fd: ::c_int, buf: *mut statfs) -> ::c_int;
    pub fn fstatfs64(fd: ::c_int, buf: *mut statfs64) -> ::c_int;
    pub fn statvfs64(path: *const ::c_char, buf: *mut statvfs64) -> ::c_int;
    pub fn fstatvfs64(fd: ::c_int, buf: *mut statvfs64) -> ::c_int;
    pub fn memrchr(cx: *const ::c_void,
                   c: ::c_int,
                   n: ::size_t) -> *mut ::c_void;

    pub fn posix_fadvise(fd: ::c_int, offset: ::off_t, len: ::off_t,
                         advise: ::c_int) -> ::c_int;
    pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;
    pub fn utimensat(dirfd: ::c_int, path: *const ::c_char,
                     times: *const ::timespec, flag: ::c_int) -> ::c_int;
    pub fn duplocale(base: ::locale_t) -> ::locale_t;
    pub fn freelocale(loc: ::locale_t);
    pub fn newlocale(mask: ::c_int,
                     locale: *const ::c_char,
                     base: ::locale_t) -> ::locale_t;
    pub fn uselocale(loc: ::locale_t) -> ::locale_t;
    pub fn creat64(path: *const c_char, mode: mode_t) -> ::c_int;
    pub fn fstat64(fildes: ::c_int, buf: *mut stat64) -> ::c_int;
    pub fn fstatat64(dirfd: ::c_int, pathname: *const c_char,
                     buf: *mut stat64, flags: ::c_int) -> ::c_int;
    pub fn ftruncate64(fd: ::c_int, length: off64_t) -> ::c_int;
    pub fn getrlimit64(resource: ::c_int, rlim: *mut rlimit64) -> ::c_int;
    pub fn lseek64(fd: ::c_int, offset: off64_t, whence: ::c_int) -> off64_t;
    pub fn lstat64(path: *const c_char, buf: *mut stat64) -> ::c_int;
    pub fn mmap64(addr: *mut ::c_void,
                  len: ::size_t,
                  prot: ::c_int,
                  flags: ::c_int,
                  fd: ::c_int,
                  offset: off64_t)
                  -> *mut ::c_void;
    pub fn open64(path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
    pub fn openat64(fd: ::c_int,
                    path: *const c_char,
                    oflag: ::c_int, ...) -> ::c_int;
    pub fn pread64(fd: ::c_int, buf: *mut ::c_void, count: ::size_t,
                   offset: off64_t) -> ::ssize_t;
    pub fn preadv64(fd: ::c_int,
                    iov: *const ::iovec,
                    iovcnt: ::c_int,
                    offset: ::off64_t) -> ::ssize_t;
    pub fn pwrite64(fd: ::c_int, buf: *const ::c_void, count: ::size_t,
                    offset: off64_t) -> ::ssize_t;
    pub fn pwritev64(fd: ::c_int,
                     iov: *const ::iovec,
                     iovcnt: ::c_int,
                     offset: ::off64_t) -> ::ssize_t;
    pub fn readdir64(dirp: *mut ::DIR) -> *mut ::dirent64;
    pub fn readdir64_r(dirp: *mut ::DIR, entry: *mut ::dirent64,
                       result: *mut *mut ::dirent64) -> ::c_int;
    pub fn setrlimit64(resource: ::c_int, rlim: *const rlimit64) -> ::c_int;
    pub fn stat64(path: *const c_char, buf: *mut stat64) -> ::c_int;
    pub fn truncate64(path: *const c_char, length: off64_t) -> ::c_int;

    pub fn fdopendir(fd: ::c_int) -> *mut ::DIR;

    pub fn mknodat(dirfd: ::c_int, pathname: *const ::c_char,
                   mode: ::mode_t, dev: dev_t) -> ::c_int;
    pub fn pthread_condattr_getclock(attr: *const pthread_condattr_t,
                                     clock_id: *mut clockid_t) -> ::c_int;
    pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t,
                                     clock_id: ::clockid_t) -> ::c_int;
    pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t,
                                       pshared: ::c_int) -> ::c_int;
    pub fn accept4(fd: ::c_int, addr: *mut ::sockaddr, len: *mut ::socklen_t,
                   flg: ::c_int) -> ::c_int;
    pub fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t,
                                        pshared: ::c_int) -> ::c_int;
    pub fn pthread_rwlockattr_getpshared(attr: *const pthread_rwlockattr_t,
                                         val: *mut ::c_int) -> ::c_int;
    pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t,
                                         val: ::c_int) -> ::c_int;
    pub fn ptsname_r(fd: ::c_int,
                     buf: *mut ::c_char,
                     buflen: ::size_t) -> ::c_int;
    pub fn clearenv() -> ::c_int;
    pub fn waitid(idtype: idtype_t, id: id_t, infop: *mut ::siginfo_t,
                  options: ::c_int) -> ::c_int;
    pub fn setreuid(ruid: ::uid_t, euid: ::uid_t) -> ::c_int;
    pub fn setregid(rgid: ::gid_t, egid: ::gid_t) -> ::c_int;
    pub fn getresuid(ruid: *mut ::uid_t, euid: *mut ::uid_t,
                     suid: *mut ::uid_t) -> ::c_int;
    pub fn getresgid(rgid: *mut ::gid_t, egid: *mut ::gid_t,
                     sgid: *mut ::gid_t) -> ::c_int;
    pub fn acct(filename: *const ::c_char) -> ::c_int;
    pub fn brk(addr: *mut ::c_void) -> ::c_int;
    pub fn sbrk(increment: ::intptr_t) -> *mut ::c_void;
    pub fn vfork() -> ::pid_t;
    pub fn setresgid(rgid: ::gid_t, egid: ::gid_t, sgid: ::gid_t) -> ::c_int;
    pub fn setresuid(ruid: ::uid_t, euid: ::uid_t, suid: ::uid_t) -> ::c_int;
    pub fn wait4(pid: ::pid_t, status: *mut ::c_int, options: ::c_int,
                 rusage: *mut ::rusage) -> ::pid_t;
    pub fn openpty(amaster: *mut ::c_int,
                aslave: *mut ::c_int,
                name: *mut ::c_char,
                termp: *const termios,
                winp: *const ::winsize) -> ::c_int;
    pub fn execvpe(file: *const ::c_char, argv: *const *const ::c_char,
                   envp: *const *const ::c_char) -> ::c_int;
    pub fn fexecve(fd: ::c_int, argv: *const *const ::c_char,
                   envp: *const *const ::c_char)
                   -> ::c_int;
    pub fn getifaddrs(ifap: *mut *mut ::ifaddrs) -> ::c_int;
    pub fn freeifaddrs(ifa: *mut ::ifaddrs);
    pub fn bind(socket: ::c_int, address: *const ::sockaddr,
                address_len: ::socklen_t) -> ::c_int;

    pub fn writev(fd: ::c_int,
                  iov: *const ::iovec,
                  iovcnt: ::c_int) -> ::ssize_t;
    pub fn readv(fd: ::c_int,
                 iov: *const ::iovec,
                 iovcnt: ::c_int) -> ::ssize_t;

    pub fn sendmsg(fd: ::c_int,
                   msg: *const ::msghdr,
                   flags: ::c_int) -> ::ssize_t;
    pub fn recvmsg(fd: ::c_int, msg: *mut ::msghdr, flags: ::c_int)
                   -> ::ssize_t;
}

cfg_if! {
    if #[cfg(target_os = "emscripten")] {
        mod emscripten;
        pub use self::emscripten::*;
    } else if #[cfg(any(target_os = "linux", target_os = "fuchsia"))] {
        mod linux;
        pub use self::linux::*;
    } else if #[cfg(target_os = "android")] {
        mod android;
        pub use self::android::*;
    } else {
        // Unknown target_os
    }
}
    // pub fn forkpty(amaster: *mut ::c_int,
    //             name: *mut ::c_char,
    //             termp: *const termios,
    //             winp: *const ::winsize) -> ::pid_t;
