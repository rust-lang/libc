s! {
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
}

pub const MAP_STACK : ::c_int = 0x4000;

// https://github.com/openbsd/src/blob/master/sys/net/if.h#L187
pub const IFF_UP: ::c_int = 0x1; // interface is up
pub const IFF_BROADCAST: ::c_int = 0x2; // broadcast address valid
pub const IFF_DEBUG: ::c_int = 0x4; // turn on debugging
pub const IFF_LOOPBACK: ::c_int = 0x8; // is a loopback net
pub const IFF_POINTOPOINT: ::c_int = 0x10; // interface is point-to-point link
pub const IFF_STATICARP: ::c_int = 0x20; // only static ARP
pub const IFF_RUNNING: ::c_int = 0x40; // resources allocated
pub const IFF_NOARP: ::c_int = 0x80; // no address resolution protocol
pub const IFF_PROMISC: ::c_int = 0x100; // receive all packets
pub const IFF_ALLMULTI: ::c_int = 0x200; // receive all multicast packets
pub const IFF_OACTIVE: ::c_int = 0x400; // transmission in progress
pub const IFF_SIMPLEX: ::c_int = 0x800; // can't hear own transmissions
pub const IFF_LINK0: ::c_int = 0x1000; // per link layer defined bit
pub const IFF_LINK1: ::c_int = 0x2000; // per link layer defined bit
pub const IFF_LINK2: ::c_int = 0x4000; // per link layer defined bit
pub const IFF_MULTICAST: ::c_int = 0x8000; // supports multicast

pub const SIGSTKSZ : ::size_t = 28672;

extern {
    pub fn accept4(s: ::c_int, addr: *mut ::sockaddr,
                   addrlen: *mut ::socklen_t, flags: ::c_int) -> ::c_int;
    pub fn execvpe(file: *const ::c_char, argv: *const *const ::c_char,
                   envp: *const *const ::c_char) -> ::c_int;
    pub fn pledge(promises: *const ::c_char,
                  execpromises: *const ::c_char) -> ::c_int;
    pub fn strtonum(nptr: *const ::c_char, minval: ::c_longlong,
                    maxval: ::c_longlong,
                    errstr: *mut *const ::c_char) -> ::c_longlong;
}

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        mod x86;
        pub use self::x86::*;
    } else if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    } else if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else {
        // Unknown target_arch
    }
}
