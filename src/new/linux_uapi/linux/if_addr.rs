//! Header: `uapi/linux/if_addr.h`

s! {
    pub struct ifaddrmsg {
        pub ifa_family: u8,
        pub ifa_prefixlen: u8,
        pub ifa_flags: u8,
        pub ifa_scope: u8,
        pub ifa_index: u32,
    }
}
