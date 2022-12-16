pub type c_char = i8;
pub type c_long = i32;
pub type c_ulong = u32;

pub type wchar_t = ::c_uint;

// pub type u_char = ::c_uchar;
// pub type u_short = ::c_ushort;
// pub type u_int = ::c_uint;
// pub type u_long = c_ulong;
// pub type ushort = ::c_ushort;
// pub type uint = ::c_uint;
// pub type ulong = c_ulong;
pub type clock_t = c_ulong;
// pub type daddr_t = c_long;
// pub type caddr_t = *mut c_char;
// pub type sbintime_t = ::c_longlong;
// pub type sigset_t = ::c_ulong;

s! {
    pub struct cmsghdr {
        pub cmsg_len: ::socklen_t,
        pub cmsg_level: ::c_int,
        pub cmsg_type: ::c_int,
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::c_int,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::size_t,
        pub msg_flags: ::c_int,
    }

    pub struct sockaddr {
        pub sa_family: ::sa_family_t,
        pub data: [::c_char; NET_SOCKADDR_MAX_SIZE], // previously [:14]
    }

    pub struct sockaddr_in6 {
        pub sin6_family: ::sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u8,
    }

    pub struct sockaddr_in {
        pub sin_family: ::sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
    }

    pub struct sockaddr_storage {
        pub ss_family: ::sa_family_t,
        pub data: [::c_char; NET_SOCKADDR_MAX_SIZE]
    }

    pub struct sockaddr_un {
        pub sun_family: ::sa_family_t,
        pub sun_path: [::c_char; NET_SOCKADDR_MAX_SIZE]
    }
}

pub const NET_SOCKADDR_MAX_SIZE: ::size_t = crate::mem::size_of::<sockaddr_in6>();

pub const PF_UNSPEC: c_long = 0;
/**< Unspecified protocol family.  */
pub const PF_INET: c_long = 1;
/**< IP protocol family version 4. */
pub const PF_INET6: c_long = 2;
/**< IP protocol family version 6. */
pub const PF_PACKET: c_long = 3;
/**< Packet family.                */
pub const PF_CAN: c_long = 4;
/**< Controller Area Network.      */
pub const PF_NET_MGMT: c_long = 5;
/**< Network management info.      */
pub const PF_LOCAL: c_long = 6;
/**< Inter-process communication   */
pub const PF_UNIX: c_long = PF_LOCAL;
/**< Inter-process communication   */

pub const AF_UNSPEC: c_long = PF_UNSPEC;
/**< Unspecified address family.   */
pub const AF_INET: c_long = PF_INET;
/**< IP protocol family version 4. */
pub const AF_INET6: c_long = PF_INET6;
/**< IP protocol family version 6. */
pub const AF_PACKET: c_long = PF_PACKET;
/**< Packet family.                */
pub const AF_CAN: c_long = PF_CAN;
/**< Controller Area Network.      */
pub const AF_NET_MGMT: c_long = PF_NET_MGMT;
/**< Network management info.      */
pub const AF_LOCAL: c_long = PF_LOCAL;
/**< Inter-process communication   */
pub const AF_UNIX: c_long = PF_UNIX;
/**< Inter-process communication   */

pub const FIONBIO: ::c_ulong = 2147772030;

pub const ZSOCK_POLLIN: ::c_short = 0x1;
pub const ZSOCK_POLLPRI: ::c_short = 0x2;
pub const ZSOCK_POLLHUP: ::c_short = 0x4;
pub const ZSOCK_POLLERR: ::c_short = 0x8;
pub const ZSOCK_POLLOUT: ::c_short = 0x10;
pub const ZSOCK_POLLNVAL: ::c_short = 0x20;

pub const POLLIN: ::c_short = ZSOCK_POLLIN;
pub const POLLPRI: ::c_short = ZSOCK_POLLPRI;
pub const POLLHUP: ::c_short = ZSOCK_POLLHUP;
pub const POLLERR: ::c_short = ZSOCK_POLLERR;
pub const POLLOUT: ::c_short = ZSOCK_POLLOUT;
pub const POLLNVAL: ::c_short = ZSOCK_POLLNVAL;

pub const _SC_PAGESIZE: ::c_int = 8;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 51;

pub const SIGHUP: ::c_int = 1; /* hangup */
pub const SIGINT: ::c_int = 2; /* interrupt */
pub const SIGQUIT: ::c_int = 3; /* quit */
pub const SIGILL: ::c_int = 4; /* illegal instruction (not reset when caught) */
pub const SIGTRAP: ::c_int = 5; /* trace trap (not reset when caught) */
pub const SIGIOT: ::c_int = 6; /* IOT instruction */
pub const SIGABRT: ::c_int = 6; /* used by abort, replace SIGIOT in the future */
pub const SIGEMT: ::c_int = 7; /* EMT instruction */
pub const SIGFPE: ::c_int = 8; /* floating point exception */
pub const SIGKILL: ::c_int = 9; /* kill (cannot be caught or ignored) */
pub const SIGBUS: ::c_int = 10; /* bus error */
pub const SIGSEGV: ::c_int = 11; /* segmentation violation */
pub const SIGSYS: ::c_int = 12; /* bad argument to system call */
pub const SIGPIPE: ::c_int = 13; /* write on a pipe with no one to read it */
pub const SIGALRM: ::c_int = 14; /* alarm clock */
pub const SIGTERM: ::c_int = 15; /* software termination signal from kill */
pub const SOL_SOCKET: ::c_int = 0xfff;

pub const ZSOCK_MSG_PEEK: c_long = 0x02;
pub const ZSOCK_MSG_TRUNC: c_long = 0x20;
pub const ZSOCK_MSG_DONTWAIT: c_long = 0x40;
pub const ZSOCK_MSG_WAITALL: c_long = 0x100;

pub const MSG_PEEK: c_long = ZSOCK_MSG_PEEK;
pub const MSG_TRUNC: c_long = ZSOCK_MSG_TRUNC;
pub const MSG_DONTWAIT: c_long = ZSOCK_MSG_DONTWAIT;
pub const MSG_WAITALL: c_long = ZSOCK_MSG_WAITALL;

pub const DNS_EAI_BADFLAGS: ::c_int = -1;
pub const DNS_EAI_NONAME: ::c_int = -2;
pub const DNS_EAI_AGAIN: ::c_int = -3;
pub const DNS_EAI_FAIL: ::c_int = -4;
pub const DNS_EAI_NODATA: ::c_int = -5;
pub const DNS_EAI_FAMILY: ::c_int = -6;
pub const DNS_EAI_SOCKTYPE: ::c_int = -7;
pub const DNS_EAI_SERVICE: ::c_int = -8;
pub const DNS_EAI_ADDRFAMILY: ::c_int = -9;
pub const DNS_EAI_MEMORY: ::c_int = -10;
pub const DNS_EAI_SYSTEM: ::c_int = -11;
pub const DNS_EAI_OVERFLOW: ::c_int = -12;
pub const DNS_EAI_INPROGRESS: ::c_int = -100;
pub const DNS_EAI_CANCELED: ::c_int = -101;
pub const DNS_EAI_NOTCANCELED: ::c_int = -102;
pub const DNS_EAI_ALLDONE: ::c_int = -103;
pub const DNS_EAI_IDN_ENCODE: ::c_int = -105;

/** POSIX wrapper for @ref DNS_EAI_BADFLAGS */
pub const EAI_BADFLAGS: ::c_int = DNS_EAI_BADFLAGS;
/** POSIX wrapper for @ref DNS_EAI_NONAME */
pub const EAI_NONAME: ::c_int = DNS_EAI_NONAME;
/** POSIX wrapper for @ref DNS_EAI_AGAIN */
pub const EAI_AGAIN: ::c_int = DNS_EAI_AGAIN;
/** POSIX wrapper for @ref DNS_EAI_FAIL */
pub const EAI_FAIL: ::c_int = DNS_EAI_FAIL;
/** POSIX wrapper for @ref DNS_EAI_NODATA */
pub const EAI_NODATA: ::c_int = DNS_EAI_NODATA;
/** POSIX wrapper for @ref DNS_EAI_MEMORY */
pub const EAI_MEMORY: ::c_int = DNS_EAI_MEMORY;
/** POSIX wrapper for @ref DNS_EAI_SYSTEM */
pub const EAI_SYSTEM: ::c_int = DNS_EAI_SYSTEM;
/** POSIX wrapper for @ref DNS_EAI_SERVICE */
pub const EAI_SERVICE: ::c_int = DNS_EAI_SERVICE;
/** POSIX wrapper for @ref DNS_EAI_SOCKTYPE */
pub const EAI_SOCKTYPE: ::c_int = DNS_EAI_SOCKTYPE;
/** POSIX wrapper for @ref DNS_EAI_FAMILY */
pub const EAI_FAMILY: ::c_int = DNS_EAI_FAMILY;

pub const PTHREAD_STACK_MIN: ::size_t = 768;

pub const RTLD_DEFAULT: *mut ::c_void = 0 as *mut ::c_void;

extern "C" {
    pub fn pthread_create(
        native: *mut ::pthread_t,
        attr: *const ::pthread_attr_t,
        f: extern "C" fn(_: *mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;

    // pub fn getrandom(buf: *mut ::c_void, buflen: ::size_t, flags: ::c_uint) -> ::ssize_t;
    //
    // #[link_name = "lwip_sendmsg"]
    // pub fn sendmsg(s: ::c_int, msg: *const ::msghdr, flags: ::c_int) -> ::ssize_t;
    // #[link_name = "lwip_recvmsg"]
    // pub fn recvmsg(s: ::c_int, msg: *mut ::msghdr, flags: ::c_int) -> ::ssize_t;
    //
    // pub fn eventfd(initval: ::c_uint, flags: ::c_int) -> ::c_int;
}

pub use crate::unix::newlib::generic::{sigset_t, stat};
