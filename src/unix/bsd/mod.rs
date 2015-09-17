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
    }

    pub struct sigaltstack {
        pub ss_sp: *mut ::c_void,
        pub ss_size: ::size_t,
        pub ss_flags: ::c_int,
    }
}

pub const FIOCLEX: c_ulong = 0x20006601;

pub const SA_ONSTACK: ::c_int = 0x0001;
pub const SA_SIGINFO: ::c_int = 0x0040;

pub const SIGSTKSZ: ::size_t = 131072;
pub const SIGBUS: ::c_int = 10;
pub const SIG_SETMASK: ::c_int = 3;

extern {
    pub fn mincore(addr: *const ::c_void, len: size_t,
                   vec: *mut c_char) -> c_int;
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
