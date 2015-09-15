s! {
    pub struct sockaddr {
        pub sa_len: u8,
        pub sa_family: sa_family_t,
        pub sa_data: [u8; 14],
    }

    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [u8; 8],
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
}

extern {
    pub fn shm_open(name: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn sysctl(name: *mut c_int,
                  namelen: c_uint,
                  oldp: *mut ::c_void,
                  oldlenp: *mut size_t,
                  newp: *mut ::c_void,
                  newlen: size_t)
                  -> c_int;
    pub fn mincore(addr: *const ::c_void, len: size_t,
                   vec: *mut c_char) -> c_int;
    pub fn sysctlbyname(name: *const c_char,
                        oldp: *mut ::c_void,
                        oldlenp: *mut size_t,
                        newp: *mut ::c_void,
                        newlen: size_t)
                        -> c_int;
    pub fn sysctlnametomib(name: *const c_char,
                           mibp: *mut c_int,
                           sizep: *mut size_t)
                           -> c_int;
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
