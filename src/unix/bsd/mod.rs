pub type c_char = i8;
pub type wchar_t = i32;
pub type off_t = i64;
pub type useconds_t = u32;
pub type blkcnt_t = i64;
pub type socklen_t = u32;
pub type sa_family_t = u8;
pub type pthread_t = ::uintptr_t;

s! {
    pub struct sockaddr {
        pub sa_len: u8,
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [::c_char; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_len: u8,
        pub sin6_family: sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_un {
        pub sun_len: u8,
        pub sun_family: sa_family_t,
        pub sun_path: [c_char; 104]
    }

    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_change: ::time_t,
        pub pw_class: *mut ::c_char,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
        pub pw_expire: ::time_t,

        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        pub pw_fields: ::c_int,
    }

    pub struct ifaddrs {
        pub ifa_next: *mut ifaddrs,
        pub ifa_name: *mut ::c_char,
        pub ifa_flags: ::c_uint,
        pub ifa_addr: *mut ::sockaddr,
        pub ifa_netmask: *mut ::sockaddr,
        pub ifa_dstaddr: *mut ::sockaddr,
        pub ifa_data: *mut ::c_void
    }
}

pub const FIOCLEX: c_ulong = 0x20006601;

pub const SA_ONSTACK: ::c_int = 0x0001;
pub const SA_SIGINFO: ::c_int = 0x0040;

pub const SIGBUS: ::c_int = 10;
pub const SIG_SETMASK: ::c_int = 3;

pub const IPV6_MULTICAST_LOOP: ::c_int = 11;
pub const IPV6_V6ONLY: ::c_int = 27;

extern {
    pub fn mincore(addr: *const ::c_void, len: ::size_t,
                   vec: *mut c_char) -> ::c_int;
    pub fn sysctlnametomib(name: *const c_char,
                           mibp: *mut ::c_int,
                           sizep: *mut ::size_t)
                           -> ::c_int;
    pub fn setgroups(ngroups: ::c_int,
                     ptr: *const ::gid_t) -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
}

cfg_if! {
    if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        mod apple;
        pub use self::apple::*;
    } else if #[cfg(any(target_os = "openbsd", target_os = "netbsd",
                        target_os = "dragonfly"))] {
        mod openbsdlike;
        pub use self::openbsdlike::*;
    } else if #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))] {
        mod freebsdlike;
        pub use self::freebsdlike::*;
    } else {
        // ...
    }
}
