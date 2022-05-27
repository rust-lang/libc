use super::{Send, Sync};

pub use ffi::c_void;

pub type c_char = i8;
pub type c_uchar = u8;
pub type c_schar = i8;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_short = i16;
pub type c_ushort = u16;
#[cfg(target_arch = "wasm32")]
pub type c_long = i32;
#[cfg(target_arch = "wasm32")]
pub type c_ulong = u32;
#[cfg(target_arch = "wasm64")]
pub type c_long = i64;
#[cfg(target_arch = "wasm64")]
pub type c_ulong = u64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type off_t = i64;
pub type pid_t = i32;
pub type clock_t = c_longlong;
pub type time_t = c_longlong;
pub type c_double = f64;
pub type c_float = f32;
pub type ino_t = u64;
pub type sigset_t = c_uchar;
pub type suseconds_t = c_longlong;
pub type mode_t = u32;
pub type dev_t = u64;
pub type uid_t = u32;
pub type gid_t = u32;
pub type nlink_t = u64;
pub type blksize_t = c_long;
pub type blkcnt_t = i64;
pub type nfds_t = c_ulong;
pub type wchar_t = i32;
pub type nl_item = c_int;

pub type socklen_t = u32;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sa_family_t = u16;

s! {
    pub struct in_addr {
        pub s_addr: ::in_addr_t,
    }
    
    #[repr(align(4))]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct ip_mreqn {
        pub imr_multiaddr: in_addr,
        pub imr_address: in_addr,
        pub imr_ifindex: ::c_int,
    }

    pub struct ip_mreq_source {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
        pub imr_sourceaddr: in_addr,
    }

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

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut ::sockaddr,
        pub ai_canonname: *mut c_char,
        pub ai_next: *mut addrinfo,
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
        pub ifa_ifu: *mut ::sockaddr,
        pub ifa_data: *mut ::c_void
    }

    pub struct in6_rtmsg {
        rtmsg_dst: ::in6_addr,
        rtmsg_src: ::in6_addr,
        rtmsg_gateway: ::in6_addr,
        rtmsg_type: u32,
        rtmsg_dst_len: u16,
        rtmsg_src_len: u16,
        rtmsg_metric: u32,
        rtmsg_info: ::c_ulong,
        rtmsg_flags: u32,
        rtmsg_ifindex: ::c_int,
    }

    pub struct arpreq {
        pub arp_pa: ::sockaddr,
        pub arp_ha: ::sockaddr,
        pub arp_flags: ::c_int,
        pub arp_netmask: ::sockaddr,
        pub arp_dev: [::c_char; 16],
    }

    pub struct arpreq_old {
        pub arp_pa: ::sockaddr,
        pub arp_ha: ::sockaddr,
        pub arp_flags: ::c_int,
        pub arp_netmask: ::sockaddr,
    }

    pub struct arphdr {
        pub ar_hrd: u16,
        pub ar_pro: u16,
        pub ar_hln: u8,
        pub ar_pln: u8,
        pub ar_op: u16,
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::c_int,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::socklen_t,
        pub msg_flags: ::c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: ::socklen_t,
        pub cmsg_level: ::c_int,
        pub cmsg_type: ::c_int,
    }

    pub struct mmsghdr {
        pub msg_hdr: ::msghdr,
        pub msg_len: ::c_uint,
    }
}

s_no_extra_traits! {
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
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for sockaddr_un {
            fn eq(&self, other: &sockaddr_un) -> bool {
                self.sun_family == other.sun_family
                    && self
                    .sun_path
                    .iter()
                    .zip(other.sun_path.iter())
                    .all(|(a, b)| a == b)
            }
        }
        impl Eq for sockaddr_un {}
        impl ::fmt::Debug for sockaddr_un {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("sockaddr_un")
                    .field("sun_family", &self.sun_family)
                    .finish()
            }
        }
        impl ::hash::Hash for sockaddr_un {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.sun_family.hash(state);
                self.sun_path.hash(state);
            }
        }

        impl PartialEq for sockaddr_storage {
            fn eq(&self, other: &sockaddr_storage) -> bool {
                self.ss_family == other.ss_family
                    && self
                    .__ss_pad2
                    .iter()
                    .zip(other.__ss_pad2.iter())
                    .all(|(a, b)| a == b)
            }
        }

        impl Eq for sockaddr_storage {}

        impl ::fmt::Debug for sockaddr_storage {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("sockaddr_storage")
                    .field("ss_family", &self.ss_family)
                    .field("__ss_align", &self.__ss_align)
                    .finish()
            }
        }

        impl ::hash::Hash for sockaddr_storage {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.ss_family.hash(state);
                self.__ss_pad2.hash(state);
            }
        }
    }
}

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
pub const SOL_BLUETOOTH: ::c_int = 274;
pub const SOL_ALG: ::c_int = 279;

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
pub const IP_TOS: ::c_int = 1;
pub const IP_TTL: ::c_int = 2;
pub const IP_HDRINCL: ::c_int = 3;
pub const IP_OPTIONS: ::c_int = 4;
pub const IP_ROUTER_ALERT: ::c_int = 5;
pub const IP_RECVOPTS: ::c_int = 6;
pub const IP_RETOPTS: ::c_int = 7;
pub const IP_PKTINFO: ::c_int = 8;
pub const IP_PKTOPTIONS: ::c_int = 9;
pub const IP_MTU_DISCOVER: ::c_int = 10;
pub const IP_RECVERR: ::c_int = 11;
pub const IP_RECVTTL: ::c_int = 12;
pub const IP_RECVTOS: ::c_int = 13;
pub const IP_MTU: ::c_int = 14;
pub const IP_FREEBIND: ::c_int = 15;
pub const IP_IPSEC_POLICY: ::c_int = 16;
pub const IP_XFRM_POLICY: ::c_int = 17;
pub const IP_PASSSEC: ::c_int = 18;
pub const IP_TRANSPARENT: ::c_int = 19;
pub const IP_ORIGDSTADDR: ::c_int = 20;
pub const IP_RECVORIGDSTADDR: ::c_int = IP_ORIGDSTADDR;
pub const IP_MINTTL: ::c_int = 21;
pub const IP_NODEFRAG: ::c_int = 22;
pub const IP_CHECKSUM: ::c_int = 23;
pub const IP_BIND_ADDRESS_NO_PORT: ::c_int = 24;
pub const IP_MULTICAST_IF: ::c_int = 32;
pub const IP_MULTICAST_TTL: ::c_int = 33;
pub const IP_MULTICAST_LOOP: ::c_int = 34;
pub const IP_ADD_MEMBERSHIP: ::c_int = 35;
pub const IP_DROP_MEMBERSHIP: ::c_int = 36;
pub const IP_UNBLOCK_SOURCE: ::c_int = 37;
pub const IP_BLOCK_SOURCE: ::c_int = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: ::c_int = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: ::c_int = 40;
pub const IP_MSFILTER: ::c_int = 41;
pub const IP_MULTICAST_ALL: ::c_int = 49;
pub const IP_UNICAST_IF: ::c_int = 50;

pub const IP_DEFAULT_MULTICAST_TTL: ::c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: ::c_int = 1;

pub const IP_PMTUDISC_DONT: ::c_int = 0;
pub const IP_PMTUDISC_WANT: ::c_int = 1;
pub const IP_PMTUDISC_DO: ::c_int = 2;
pub const IP_PMTUDISC_PROBE: ::c_int = 3;
pub const IP_PMTUDISC_INTERFACE: ::c_int = 4;
pub const IP_PMTUDISC_OMIT: ::c_int = 5;

// IPPROTO_IP defined in src/unix/mod.rs
/// Hop-by-hop option header
pub const IPPROTO_HOPOPTS: ::c_int = 0;
// IPPROTO_ICMP defined in src/unix/mod.rs
/// group mgmt protocol
pub const IPPROTO_IGMP: ::c_int = 2;
/// for compatibility
pub const IPPROTO_IPIP: ::c_int = 4;
// IPPROTO_TCP defined in src/unix/mod.rs
/// exterior gateway protocol
pub const IPPROTO_EGP: ::c_int = 8;
/// pup
pub const IPPROTO_PUP: ::c_int = 12;
// IPPROTO_UDP defined in src/unix/mod.rs
/// xns idp
pub const IPPROTO_IDP: ::c_int = 22;
/// tp-4 w/ class negotiation
pub const IPPROTO_TP: ::c_int = 29;
/// DCCP
pub const IPPROTO_DCCP: ::c_int = 33;
// IPPROTO_IPV6 defined in src/unix/mod.rs
/// IP6 routing header
pub const IPPROTO_ROUTING: ::c_int = 43;
/// IP6 fragmentation header
pub const IPPROTO_FRAGMENT: ::c_int = 44;
/// resource reservation
pub const IPPROTO_RSVP: ::c_int = 46;
/// General Routing Encap.
pub const IPPROTO_GRE: ::c_int = 47;
/// IP6 Encap Sec. Payload
pub const IPPROTO_ESP: ::c_int = 50;
/// IP6 Auth Header
pub const IPPROTO_AH: ::c_int = 51;
// IPPROTO_ICMPV6 defined in src/unix/mod.rs
/// IP6 no next header
pub const IPPROTO_NONE: ::c_int = 59;
/// IP6 destination option
pub const IPPROTO_DSTOPTS: ::c_int = 60;
pub const IPPROTO_MTP: ::c_int = 92;
/// encapsulation header
pub const IPPROTO_ENCAP: ::c_int = 98;
/// Protocol indep. multicast
pub const IPPROTO_PIM: ::c_int = 103;
/// IP Payload Comp. Protocol
pub const IPPROTO_COMP: ::c_int = 108;
/// SCTP
pub const IPPROTO_SCTP: ::c_int = 132;
pub const IPPROTO_MH: ::c_int = 135;
pub const IPPROTO_UDPLITE: ::c_int = 136;
/// raw IP packet
pub const IPPROTO_RAW: ::c_int = 255;
pub const IPPROTO_BEETPH: ::c_int = 94;
pub const IPPROTO_MPLS: ::c_int = 137;

pub const MCAST_EXCLUDE: ::c_int = 0;
pub const MCAST_INCLUDE: ::c_int = 1;
pub const MCAST_JOIN_GROUP: ::c_int = 42;
pub const MCAST_BLOCK_SOURCE: ::c_int = 43;
pub const MCAST_UNBLOCK_SOURCE: ::c_int = 44;
pub const MCAST_LEAVE_GROUP: ::c_int = 45;
pub const MCAST_JOIN_SOURCE_GROUP: ::c_int = 46;
pub const MCAST_LEAVE_SOURCE_GROUP: ::c_int = 47;
pub const MCAST_MSFILTER: ::c_int = 48;

pub const IPV6_ADDRFORM: ::c_int = 1;
pub const IPV6_2292PKTINFO: ::c_int = 2;
pub const IPV6_2292HOPOPTS: ::c_int = 3;
pub const IPV6_2292DSTOPTS: ::c_int = 4;
pub const IPV6_2292RTHDR: ::c_int = 5;
pub const IPV6_2292PKTOPTIONS: ::c_int = 6;
pub const IPV6_CHECKSUM: ::c_int = 7;
pub const IPV6_2292HOPLIMIT: ::c_int = 8;
pub const IPV6_NEXTHOP: ::c_int = 9;
pub const IPV6_AUTHHDR: ::c_int = 10;
pub const IPV6_UNICAST_HOPS: ::c_int = 16;
pub const IPV6_MULTICAST_IF: ::c_int = 17;
pub const IPV6_MULTICAST_HOPS: ::c_int = 18;
pub const IPV6_MULTICAST_LOOP: ::c_int = 19;
pub const IPV6_ADD_MEMBERSHIP: ::c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: ::c_int = 21;
pub const IPV6_ROUTER_ALERT: ::c_int = 22;
pub const IPV6_MTU_DISCOVER: ::c_int = 23;
pub const IPV6_MTU: ::c_int = 24;
pub const IPV6_RECVERR: ::c_int = 25;
pub const IPV6_V6ONLY: ::c_int = 26;
pub const IPV6_JOIN_ANYCAST: ::c_int = 27;
pub const IPV6_LEAVE_ANYCAST: ::c_int = 28;
pub const IPV6_IPSEC_POLICY: ::c_int = 34;
pub const IPV6_XFRM_POLICY: ::c_int = 35;
pub const IPV6_HDRINCL: ::c_int = 36;
pub const IPV6_RECVPKTINFO: ::c_int = 49;
pub const IPV6_PKTINFO: ::c_int = 50;
pub const IPV6_RECVHOPLIMIT: ::c_int = 51;
pub const IPV6_HOPLIMIT: ::c_int = 52;
pub const IPV6_RECVHOPOPTS: ::c_int = 53;
pub const IPV6_HOPOPTS: ::c_int = 54;
pub const IPV6_RTHDRDSTOPTS: ::c_int = 55;
pub const IPV6_RECVRTHDR: ::c_int = 56;
pub const IPV6_RTHDR: ::c_int = 57;
pub const IPV6_RECVDSTOPTS: ::c_int = 58;
pub const IPV6_DSTOPTS: ::c_int = 59;
pub const IPV6_RECVPATHMTU: ::c_int = 60;
pub const IPV6_PATHMTU: ::c_int = 61;
pub const IPV6_DONTFRAG: ::c_int = 62;
pub const IPV6_RECVTCLASS: ::c_int = 66;
pub const IPV6_TCLASS: ::c_int = 67;
pub const IPV6_AUTOFLOWLABEL: ::c_int = 70;
pub const IPV6_ADDR_PREFERENCES: ::c_int = 72;
pub const IPV6_MINHOPCOUNT: ::c_int = 73;
pub const IPV6_ORIGDSTADDR: ::c_int = 74;
pub const IPV6_RECVORIGDSTADDR: ::c_int = IPV6_ORIGDSTADDR;
pub const IPV6_TRANSPARENT: ::c_int = 75;
pub const IPV6_UNICAST_IF: ::c_int = 76;
pub const IPV6_PREFER_SRC_TMP: ::c_int = 0x0001;
pub const IPV6_PREFER_SRC_PUBLIC: ::c_int = 0x0002;
pub const IPV6_PREFER_SRC_PUBTMP_DEFAULT: ::c_int = 0x0100;
pub const IPV6_PREFER_SRC_COA: ::c_int = 0x0004;
pub const IPV6_PREFER_SRC_HOME: ::c_int = 0x0400;
pub const IPV6_PREFER_SRC_CGA: ::c_int = 0x0008;
pub const IPV6_PREFER_SRC_NONCGA: ::c_int = 0x0800;

pub const IPV6_PMTUDISC_DONT: ::c_int = 0;
pub const IPV6_PMTUDISC_WANT: ::c_int = 1;
pub const IPV6_PMTUDISC_DO: ::c_int = 2;
pub const IPV6_PMTUDISC_PROBE: ::c_int = 3;
pub const IPV6_PMTUDISC_INTERFACE: ::c_int = 4;
pub const IPV6_PMTUDISC_OMIT: ::c_int = 5;

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
pub const TCP_MD5SIG: ::c_int = 14;
pub const TCP_COOKIE_TRANSACTIONS: ::c_int = 15;
pub const TCP_THIN_LINEAR_TIMEOUTS: ::c_int = 16;
pub const TCP_THIN_DUPACK: ::c_int = 17;
pub const TCP_USER_TIMEOUT: ::c_int = 18;
pub const TCP_REPAIR: ::c_int = 19;
pub const TCP_REPAIR_QUEUE: ::c_int = 20;
pub const TCP_QUEUE_SEQ: ::c_int = 21;
pub const TCP_REPAIR_OPTIONS: ::c_int = 22;
pub const TCP_FASTOPEN: ::c_int = 23;
pub const TCP_TIMESTAMP: ::c_int = 24;
pub const TCP_NOTSENT_LOWAT: ::c_int = 25;
pub const TCP_CC_INFO: ::c_int = 26;
pub const TCP_SAVE_SYN: ::c_int = 27;
pub const TCP_SAVED_SYN: ::c_int = 28;
pub const TCP_REPAIR_WINDOW: ::c_int = 29;
pub const TCP_FASTOPEN_CONNECT: ::c_int = 30;
pub const TCP_ULP: ::c_int = 31;
pub const TCP_MD5SIG_EXT: ::c_int = 32;
pub const TCP_FASTOPEN_KEY: ::c_int = 33;
pub const TCP_FASTOPEN_NO_COOKIE: ::c_int = 34;
pub const TCP_ZEROCOPY_RECEIVE: ::c_int = 35;
pub const TCP_INQ: ::c_int = 36;
pub const TCP_CM_INQ: ::c_int = TCP_INQ;

pub const SO_DEBUG: ::c_int = 1;
pub const SO_REUSEADDR: ::c_int = 2;
pub const SO_TYPE: ::c_int = 3;
pub const SO_ERROR: ::c_int = 4;
pub const SO_DONTROUTE: ::c_int = 5;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_SNDBUF: ::c_int = 7;
pub const SO_RCVBUF: ::c_int = 8;
pub const SO_KEEPALIVE: ::c_int = 9;
pub const SO_OOBINLINE: ::c_int = 10;
pub const SO_PRIORITY: ::c_int = 12;
pub const SO_LINGER: ::c_int = 13;
pub const SO_BSDCOMPAT: ::c_int = 14;
pub const SO_REUSEPORT: ::c_int = 15;
pub const SO_PASSCRED: ::c_int = 16;
pub const SO_PEERCRED: ::c_int = 17;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_SNDTIMEO: ::c_int = 21;
pub const SO_BINDTODEVICE: ::c_int = 25;
pub const SO_ATTACH_FILTER: ::c_int = 26;
pub const SO_DETACH_FILTER: ::c_int = 27;
pub const SO_GET_FILTER: ::c_int = SO_ATTACH_FILTER;
pub const SO_TIMESTAMP: ::c_int = 29;
pub const SO_ACCEPTCONN: ::c_int = 30;
pub const SO_PEERSEC: ::c_int = 31;
pub const SO_SNDBUFFORCE: ::c_int = 32;
pub const SO_RCVBUFFORCE: ::c_int = 33;
pub const SO_PASSSEC: ::c_int = 34;
pub const SO_MARK: ::c_int = 36;
pub const SO_PROTOCOL: ::c_int = 38;
pub const SO_DOMAIN: ::c_int = 39;
pub const SO_RXQ_OVFL: ::c_int = 40;
pub const SO_PEEK_OFF: ::c_int = 42;
pub const SO_BUSY_POLL: ::c_int = 46;

pub const SHUT_RD: ::c_int = 0;
pub const SHUT_WR: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;

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

pub const IPTOS_ECN_MASK: u8 = 0x03;
pub const IPTOS_ECN_ECT1: u8 = 0x01;
pub const IPTOS_ECN_ECT0: u8 = 0x02;
pub const IPTOS_ECN_CE: u8 = 0x03;

pub const IPOPT_COPY: u8 = 0x80;
pub const IPOPT_CLASS_MASK: u8 = 0x60;
pub const IPOPT_NUMBER_MASK: u8 = 0x1f;

pub const IPOPT_CONTROL: u8 = 0x00;
pub const IPOPT_RESERVED1: u8 = 0x20;
pub const IPOPT_MEASUREMENT: u8 = 0x40;
pub const IPOPT_RESERVED2: u8 = 0x60;
pub const IPOPT_END: u8 = 0 | IPOPT_CONTROL;
pub const IPOPT_NOOP: u8 = 1 | IPOPT_CONTROL;
pub const IPOPT_SEC: u8 = 2 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_LSRR: u8 = 3 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_TIMESTAMP: u8 = 4 | IPOPT_MEASUREMENT;
pub const IPOPT_RR: u8 = 7 | IPOPT_CONTROL;
pub const IPOPT_SID: u8 = 8 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_SSRR: u8 = 9 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_RA: u8 = 20 | IPOPT_CONTROL | IPOPT_COPY;
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

pub const ARPOP_RREQUEST: u16 = 3;
pub const ARPOP_RREPLY: u16 = 4;
pub const ARPOP_InREQUEST: u16 = 8;
pub const ARPOP_InREPLY: u16 = 9;
pub const ARPOP_NAK: u16 = 10;

pub const ATF_NETMASK: ::c_int = 0x20;
pub const ATF_DONTPUB: ::c_int = 0x40;

pub const ARPHRD_NETROM: u16 = 0;
pub const ARPHRD_ETHER: u16 = 1;
pub const ARPHRD_EETHER: u16 = 2;
pub const ARPHRD_AX25: u16 = 3;
pub const ARPHRD_PRONET: u16 = 4;
pub const ARPHRD_CHAOS: u16 = 5;
pub const ARPHRD_IEEE802: u16 = 6;
pub const ARPHRD_ARCNET: u16 = 7;
pub const ARPHRD_APPLETLK: u16 = 8;
pub const ARPHRD_DLCI: u16 = 15;
pub const ARPHRD_ATM: u16 = 19;
pub const ARPHRD_METRICOM: u16 = 23;
pub const ARPHRD_IEEE1394: u16 = 24;
pub const ARPHRD_EUI64: u16 = 27;
pub const ARPHRD_INFINIBAND: u16 = 32;

pub const ARPHRD_SLIP: u16 = 256;
pub const ARPHRD_CSLIP: u16 = 257;
pub const ARPHRD_SLIP6: u16 = 258;
pub const ARPHRD_CSLIP6: u16 = 259;
pub const ARPHRD_RSRVD: u16 = 260;
pub const ARPHRD_ADAPT: u16 = 264;
pub const ARPHRD_ROSE: u16 = 270;
pub const ARPHRD_X25: u16 = 271;
pub const ARPHRD_HWX25: u16 = 272;
pub const ARPHRD_CAN: u16 = 280;
pub const ARPHRD_PPP: u16 = 512;
pub const ARPHRD_CISCO: u16 = 513;
pub const ARPHRD_HDLC: u16 = ARPHRD_CISCO;
pub const ARPHRD_LAPB: u16 = 516;
pub const ARPHRD_DDCMP: u16 = 517;
pub const ARPHRD_RAWHDLC: u16 = 518;

pub const ARPHRD_TUNNEL: u16 = 768;
pub const ARPHRD_TUNNEL6: u16 = 769;
pub const ARPHRD_FRAD: u16 = 770;
pub const ARPHRD_SKIP: u16 = 771;
pub const ARPHRD_LOOPBACK: u16 = 772;
pub const ARPHRD_LOCALTLK: u16 = 773;
pub const ARPHRD_FDDI: u16 = 774;
pub const ARPHRD_BIF: u16 = 775;
pub const ARPHRD_SIT: u16 = 776;
pub const ARPHRD_IPDDP: u16 = 777;
pub const ARPHRD_IPGRE: u16 = 778;
pub const ARPHRD_PIMREG: u16 = 779;
pub const ARPHRD_HIPPI: u16 = 780;
pub const ARPHRD_ASH: u16 = 781;
pub const ARPHRD_ECONET: u16 = 782;
pub const ARPHRD_IRDA: u16 = 783;
pub const ARPHRD_FCPP: u16 = 784;
pub const ARPHRD_FCAL: u16 = 785;
pub const ARPHRD_FCPL: u16 = 786;
pub const ARPHRD_FCFABRIC: u16 = 787;
pub const ARPHRD_IEEE802_TR: u16 = 800;
pub const ARPHRD_IEEE80211: u16 = 801;
pub const ARPHRD_IEEE80211_PRISM: u16 = 802;
pub const ARPHRD_IEEE80211_RADIOTAP: u16 = 803;
pub const ARPHRD_IEEE802154: u16 = 804;

pub const ARPHRD_VOID: u16 = 0xFFFF;
pub const ARPHRD_NONE: u16 = 0xFFFE;

s_no_extra_traits! {
    #[repr(align(16))]
    #[allow(missing_debug_implementations)]
    pub struct max_align_t {
        priv_: [f64; 4]
    }
}

pub type __wasi_rights_t = u64;

#[allow(missing_copy_implementations)]
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum FILE {}
#[allow(missing_copy_implementations)]
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum DIR {}
#[allow(missing_copy_implementations)]
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum __locale_struct {}

pub type locale_t = *mut __locale_struct;

s_paren! {
    // in wasi-libc clockid_t is const struct __clockid* (where __clockid is an opaque struct),
    // but that's an implementation detail that we don't want to have to deal with
    #[repr(transparent)]
    pub struct clockid_t(*const u8);
}

unsafe impl Send for clockid_t {}
unsafe impl Sync for clockid_t {}

s! {
    #[repr(align(8))]
    pub struct fpos_t {
        data: [u8; 16],
    }

    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
        pub __tm_gmtoff: c_int,
        pub __tm_zone: *const c_char,
        pub __tm_nsec: c_int,
    }

    pub struct timeval {
        pub tv_sec: time_t,
        pub tv_usec: suseconds_t,
    }

    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }

    pub struct tms {
        pub tms_utime: clock_t,
        pub tms_stime: clock_t,
        pub tms_cutime: clock_t,
        pub tms_cstime: clock_t,
    }

    pub struct itimerspec {
        pub it_interval: timespec,
        pub it_value: timespec,
    }

    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: in6_addr,
        pub ipv6mr_interface: c_uint,
    }

    pub struct iovec {
        pub iov_base: *mut c_void,
        pub iov_len: size_t,
    }

    pub struct lconv {
        pub decimal_point: *mut c_char,
        pub thousands_sep: *mut c_char,
        pub grouping: *mut c_char,
        pub int_curr_symbol: *mut c_char,
        pub currency_symbol: *mut c_char,
        pub mon_decimal_point: *mut c_char,
        pub mon_thousands_sep: *mut c_char,
        pub mon_grouping: *mut c_char,
        pub positive_sign: *mut c_char,
        pub negative_sign: *mut c_char,
        pub int_frac_digits: c_char,
        pub frac_digits: c_char,
        pub p_cs_precedes: c_char,
        pub p_sep_by_space: c_char,
        pub n_cs_precedes: c_char,
        pub n_sep_by_space: c_char,
        pub p_sign_posn: c_char,
        pub n_sign_posn: c_char,
        pub int_p_cs_precedes: c_char,
        pub int_p_sep_by_space: c_char,
        pub int_n_cs_precedes: c_char,
        pub int_n_sep_by_space: c_char,
        pub int_p_sign_posn: c_char,
        pub int_n_sign_posn: c_char,
    }

    pub struct pollfd {
        pub fd: c_int,
        pub events: c_short,
        pub revents: c_short,
    }

    pub struct linger {
        pub l_onoff: c_int,
        pub l_linger: c_int,
    }

    pub struct itimerval {
        pub it_interval: timeval,
        pub it_value: timeval,
    }

    pub struct rusage {
        pub ru_utime: timeval,
        pub ru_stime: timeval,
    }

    pub struct stat {
        pub st_dev: dev_t,
        pub st_ino: ino_t,
        pub st_nlink: nlink_t,
        pub st_mode: mode_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        __pad0: c_uint,
        pub st_rdev: dev_t,
        pub st_size: off_t,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        __reserved: [c_longlong; 3],
    }
}

// Declare dirent outside of s! so that it doesn't implement Copy, Eq, Hash,
// etc., since it contains a flexible array member with a dynamic size.
#[repr(C)]
#[allow(missing_copy_implementations)]
#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_type: c_uchar,
    /// d_name is declared in WASI libc as a flexible array member, which
    /// can't be directly expressed in Rust. As an imperfect workaround,
    /// declare it as a zero-length array instead.
    pub d_name: [c_char; 0],
}

pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;
pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const _IOFBF: c_int = 0;
pub const _IONBF: c_int = 2;
pub const _IOLBF: c_int = 1;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const FD_CLOEXEC: c_int = 1;
pub const FD_SETSIZE: size_t = 1024;
pub const O_APPEND: c_int = 0x0001;
pub const O_DSYNC: c_int = 0x0002;
pub const O_NONBLOCK: c_int = 0x0004;
pub const O_RSYNC: c_int = 0x0008;
pub const O_SYNC: c_int = 0x0010;
pub const O_CREAT: c_int = 0x0001 << 12;
pub const O_DIRECTORY: c_int = 0x0002 << 12;
pub const O_EXCL: c_int = 0x0004 << 12;
pub const O_TRUNC: c_int = 0x0008 << 12;
pub const O_NOFOLLOW: c_int = 0x01000000;
pub const O_EXEC: c_int = 0x02000000;
pub const O_RDONLY: c_int = 0x04000000;
pub const O_SEARCH: c_int = 0x08000000;
pub const O_WRONLY: c_int = 0x10000000;
pub const O_CLOEXEC: c_int = 0x0;
pub const O_RDWR: c_int = O_WRONLY | O_RDONLY;
pub const O_ACCMODE: c_int = O_EXEC | O_RDWR | O_SEARCH;
pub const O_NOCTTY: c_int = 0x0;
pub const POSIX_FADV_DONTNEED: c_int = 4;
pub const POSIX_FADV_NOREUSE: c_int = 5;
pub const POSIX_FADV_NORMAL: c_int = 0;
pub const POSIX_FADV_RANDOM: c_int = 2;
pub const POSIX_FADV_SEQUENTIAL: c_int = 1;
pub const POSIX_FADV_WILLNEED: c_int = 3;
pub const AT_FDCWD: ::c_int = -2;
pub const AT_EACCESS: c_int = 0x0;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x1;
pub const AT_SYMLINK_FOLLOW: c_int = 0x2;
pub const AT_REMOVEDIR: c_int = 0x4;
pub const UTIME_OMIT: c_long = 0xfffffffe;
pub const UTIME_NOW: c_long = 0xffffffff;
pub const S_IFIFO: mode_t = 49152;
pub const S_IFCHR: mode_t = 8192;
pub const S_IFBLK: mode_t = 24576;
pub const S_IFDIR: mode_t = 16384;
pub const S_IFREG: mode_t = 32768;
pub const S_IFLNK: mode_t = 40960;
pub const S_IFSOCK: mode_t = 49152;
pub const S_IFMT: mode_t = 57344;
pub const S_IXOTH: mode_t = 0x1;
pub const S_IWOTH: mode_t = 0x2;
pub const S_IROTH: mode_t = 0x4;
pub const S_IXGRP: mode_t = 0x8;
pub const S_IWGRP: mode_t = 0x10;
pub const S_IRGRP: mode_t = 0x20;
pub const S_IXUSR: mode_t = 0x40;
pub const S_IWUSR: mode_t = 0x80;
pub const S_IRUSR: mode_t = 0x100;
pub const S_ISVTX: mode_t = 0x200;
pub const S_ISGID: mode_t = 0x400;
pub const S_ISUID: mode_t = 0x800;
pub const DT_UNKNOWN: u8 = 0;
pub const DT_BLK: u8 = 1;
pub const DT_CHR: u8 = 2;
pub const DT_DIR: u8 = 3;
pub const DT_REG: u8 = 4;
pub const DT_LNK: u8 = 7;
pub const FIONREAD: c_int = 1;
pub const FIONBIO: c_int = 2;
pub const F_OK: ::c_int = 0;
pub const R_OK: ::c_int = 4;
pub const W_OK: ::c_int = 2;
pub const X_OK: ::c_int = 1;
pub const POLLIN: ::c_short = 0x1;
pub const POLLOUT: ::c_short = 0x2;
pub const POLLERR: ::c_short = 0x1000;
pub const POLLHUP: ::c_short = 0x2000;
pub const POLLNVAL: ::c_short = 0x4000;
pub const POLLRDNORM: ::c_short = 0x1;
pub const POLLWRNORM: ::c_short = 0x2;

pub const E2BIG: c_int = 1;
pub const EACCES: c_int = 2;
pub const EADDRINUSE: c_int = 3;
pub const EADDRNOTAVAIL: c_int = 4;
pub const EAFNOSUPPORT: c_int = 5;
pub const EAGAIN: c_int = 6;
pub const EALREADY: c_int = 7;
pub const EBADF: c_int = 8;
pub const EBADMSG: c_int = 9;
pub const EBUSY: c_int = 10;
pub const ECANCELED: c_int = 11;
pub const ECHILD: c_int = 12;
pub const ECONNABORTED: c_int = 13;
pub const ECONNREFUSED: c_int = 14;
pub const ECONNRESET: c_int = 15;
pub const EDEADLK: c_int = 16;
pub const EDESTADDRREQ: c_int = 17;
pub const EDOM: c_int = 18;
pub const EDQUOT: c_int = 19;
pub const EEXIST: c_int = 20;
pub const EFAULT: c_int = 21;
pub const EFBIG: c_int = 22;
pub const EHOSTUNREACH: c_int = 23;
pub const EIDRM: c_int = 24;
pub const EILSEQ: c_int = 25;
pub const EINPROGRESS: c_int = 26;
pub const EINTR: c_int = 27;
pub const EINVAL: c_int = 28;
pub const EIO: c_int = 29;
pub const EISCONN: c_int = 30;
pub const EISDIR: c_int = 31;
pub const ELOOP: c_int = 32;
pub const EMFILE: c_int = 33;
pub const EMLINK: c_int = 34;
pub const EMSGSIZE: c_int = 35;
pub const EMULTIHOP: c_int = 36;
pub const ENAMETOOLONG: c_int = 37;
pub const ENETDOWN: c_int = 38;
pub const ENETRESET: c_int = 39;
pub const ENETUNREACH: c_int = 40;
pub const ENFILE: c_int = 41;
pub const ENOBUFS: c_int = 42;
pub const ENODEV: c_int = 43;
pub const ENOENT: c_int = 44;
pub const ENOEXEC: c_int = 45;
pub const ENOLCK: c_int = 46;
pub const ENOLINK: c_int = 47;
pub const ENOMEM: c_int = 48;
pub const ENOMSG: c_int = 49;
pub const ENOPROTOOPT: c_int = 50;
pub const ENOSPC: c_int = 51;
pub const ENOSYS: c_int = 52;
pub const ENOTCONN: c_int = 53;
pub const ENOTDIR: c_int = 54;
pub const ENOTEMPTY: c_int = 55;
pub const ENOTRECOVERABLE: c_int = 56;
pub const ENOTSOCK: c_int = 57;
pub const ENOTSUP: c_int = 58;
pub const ENOTTY: c_int = 59;
pub const ENXIO: c_int = 60;
pub const EOVERFLOW: c_int = 61;
pub const EOWNERDEAD: c_int = 62;
pub const EPERM: c_int = 63;
pub const EPIPE: c_int = 64;
pub const EPROTO: c_int = 65;
pub const EPROTONOSUPPORT: c_int = 66;
pub const EPROTOTYPE: c_int = 67;
pub const ERANGE: c_int = 68;
pub const EROFS: c_int = 69;
pub const ESPIPE: c_int = 70;
pub const ESRCH: c_int = 71;
pub const ESTALE: c_int = 72;
pub const ETIMEDOUT: c_int = 73;
pub const ETXTBSY: c_int = 74;
pub const EXDEV: c_int = 75;
pub const ENOTCAPABLE: c_int = 76;
pub const EOPNOTSUPP: c_int = ENOTSUP;
pub const EWOULDBLOCK: c_int = EAGAIN;

pub const _SC_PAGESIZE: c_int = 30;
pub const _SC_PAGE_SIZE: ::c_int = _SC_PAGESIZE;
pub const _SC_IOV_MAX: c_int = 60;
pub const _SC_SYMLOOP_MAX: c_int = 173;

pub static CLOCK_MONOTONIC: clockid_t = unsafe { clockid_t(ptr_addr_of!(_CLOCK_MONOTONIC)) };
pub static CLOCK_PROCESS_CPUTIME_ID: clockid_t =
    unsafe { clockid_t(ptr_addr_of!(_CLOCK_PROCESS_CPUTIME_ID)) };
pub static CLOCK_REALTIME: clockid_t = unsafe { clockid_t(ptr_addr_of!(_CLOCK_REALTIME)) };
pub static CLOCK_THREAD_CPUTIME_ID: clockid_t =
    unsafe { clockid_t(ptr_addr_of!(_CLOCK_THREAD_CPUTIME_ID)) };

pub const ABDAY_1: ::nl_item = 0x20000;
pub const ABDAY_2: ::nl_item = 0x20001;
pub const ABDAY_3: ::nl_item = 0x20002;
pub const ABDAY_4: ::nl_item = 0x20003;
pub const ABDAY_5: ::nl_item = 0x20004;
pub const ABDAY_6: ::nl_item = 0x20005;
pub const ABDAY_7: ::nl_item = 0x20006;

pub const DAY_1: ::nl_item = 0x20007;
pub const DAY_2: ::nl_item = 0x20008;
pub const DAY_3: ::nl_item = 0x20009;
pub const DAY_4: ::nl_item = 0x2000A;
pub const DAY_5: ::nl_item = 0x2000B;
pub const DAY_6: ::nl_item = 0x2000C;
pub const DAY_7: ::nl_item = 0x2000D;

pub const ABMON_1: ::nl_item = 0x2000E;
pub const ABMON_2: ::nl_item = 0x2000F;
pub const ABMON_3: ::nl_item = 0x20010;
pub const ABMON_4: ::nl_item = 0x20011;
pub const ABMON_5: ::nl_item = 0x20012;
pub const ABMON_6: ::nl_item = 0x20013;
pub const ABMON_7: ::nl_item = 0x20014;
pub const ABMON_8: ::nl_item = 0x20015;
pub const ABMON_9: ::nl_item = 0x20016;
pub const ABMON_10: ::nl_item = 0x20017;
pub const ABMON_11: ::nl_item = 0x20018;
pub const ABMON_12: ::nl_item = 0x20019;

pub const MON_1: ::nl_item = 0x2001A;
pub const MON_2: ::nl_item = 0x2001B;
pub const MON_3: ::nl_item = 0x2001C;
pub const MON_4: ::nl_item = 0x2001D;
pub const MON_5: ::nl_item = 0x2001E;
pub const MON_6: ::nl_item = 0x2001F;
pub const MON_7: ::nl_item = 0x20020;
pub const MON_8: ::nl_item = 0x20021;
pub const MON_9: ::nl_item = 0x20022;
pub const MON_10: ::nl_item = 0x20023;
pub const MON_11: ::nl_item = 0x20024;
pub const MON_12: ::nl_item = 0x20025;

pub const AM_STR: ::nl_item = 0x20026;
pub const PM_STR: ::nl_item = 0x20027;

pub const D_T_FMT: ::nl_item = 0x20028;
pub const D_FMT: ::nl_item = 0x20029;
pub const T_FMT: ::nl_item = 0x2002A;
pub const T_FMT_AMPM: ::nl_item = 0x2002B;

pub const ERA: ::nl_item = 0x2002C;
pub const ERA_D_FMT: ::nl_item = 0x2002E;
pub const ALT_DIGITS: ::nl_item = 0x2002F;
pub const ERA_D_T_FMT: ::nl_item = 0x20030;
pub const ERA_T_FMT: ::nl_item = 0x20031;

pub const CODESET: ::nl_item = 14;
pub const CRNCYSTR: ::nl_item = 0x4000F;
pub const RADIXCHAR: ::nl_item = 0x10000;
pub const THOUSEP: ::nl_item = 0x10001;
pub const YESEXPR: ::nl_item = 0x50000;
pub const NOEXPR: ::nl_item = 0x50001;
pub const YESSTR: ::nl_item = 0x50002;
pub const NOSTR: ::nl_item = 0x50003;

pub const PRIO_MIN: ::c_int = -20;
pub const PRIO_MAX: ::c_int = 20;

pub const IPPROTO_ICMP: ::c_int = 1;
pub const IPPROTO_ICMPV6: ::c_int = 58;
pub const IPPROTO_TCP: ::c_int = 6;
pub const IPPROTO_UDP: ::c_int = 17;
pub const IPPROTO_IP: ::c_int = 0;
pub const IPPROTO_IPV6: ::c_int = 41;

pub const INADDR_LOOPBACK: in_addr_t = 2130706433;
pub const INADDR_ANY: in_addr_t = 0;
pub const INADDR_BROADCAST: in_addr_t = 4294967295;
pub const INADDR_NONE: in_addr_t = 4294967295;

pub const ARPOP_REQUEST: u16 = 1;
pub const ARPOP_REPLY: u16 = 2;

#[cfg_attr(
    feature = "rustc-dep-of-std",
    link(
        name = "c",
        kind = "static",
        modifiers = "-bundle",
        cfg(target_feature = "crt-static")
    )
)]
#[cfg_attr(
    feature = "rustc-dep-of-std",
    link(name = "c", cfg(not(target_feature = "crt-static")))
)]
extern "C" {
    pub fn _Exit(code: c_int) -> !;
    pub fn _exit(code: c_int) -> !;
    pub fn abort() -> !;
    pub fn aligned_alloc(a: size_t, b: size_t) -> *mut c_void;
    pub fn calloc(amt: size_t, amt2: size_t) -> *mut c_void;
    pub fn exit(code: c_int) -> !;
    pub fn free(ptr: *mut c_void);
    pub fn getenv(s: *const c_char) -> *mut c_char;
    pub fn malloc(amt: size_t) -> *mut c_void;
    pub fn malloc_usable_size(ptr: *mut c_void) -> size_t;
    pub fn sbrk(increment: ::intptr_t) -> *mut ::c_void;
    pub fn rand() -> c_int;
    pub fn read(fd: c_int, ptr: *mut c_void, size: size_t) -> ssize_t;
    pub fn realloc(ptr: *mut c_void, amt: size_t) -> *mut c_void;
    pub fn setenv(k: *const c_char, v: *const c_char, a: c_int) -> c_int;
    pub fn unsetenv(k: *const c_char) -> c_int;
    pub fn clearenv() -> ::c_int;
    pub fn write(fd: c_int, ptr: *const c_void, size: size_t) -> ssize_t;
    pub static mut environ: *mut *mut c_char;
    pub fn fopen(a: *const c_char, b: *const c_char) -> *mut FILE;
    pub fn freopen(a: *const c_char, b: *const c_char, f: *mut FILE) -> *mut FILE;
    pub fn fclose(f: *mut FILE) -> c_int;
    pub fn remove(a: *const c_char) -> c_int;
    pub fn rename(a: *const c_char, b: *const c_char) -> c_int;
    pub fn feof(f: *mut FILE) -> c_int;
    pub fn ferror(f: *mut FILE) -> c_int;
    pub fn fflush(f: *mut FILE) -> c_int;
    pub fn clearerr(f: *mut FILE);
    pub fn fseek(f: *mut FILE, b: c_long, c: c_int) -> c_int;
    pub fn ftell(f: *mut FILE) -> c_long;
    pub fn rewind(f: *mut FILE);
    pub fn fgetpos(f: *mut FILE, pos: *mut fpos_t) -> c_int;
    pub fn fsetpos(f: *mut FILE, pos: *const fpos_t) -> c_int;
    pub fn fread(buf: *mut c_void, a: size_t, b: size_t, f: *mut FILE) -> size_t;
    pub fn fwrite(buf: *const c_void, a: size_t, b: size_t, f: *mut FILE) -> size_t;
    pub fn fgetc(f: *mut FILE) -> c_int;
    pub fn getc(f: *mut FILE) -> c_int;
    pub fn getchar() -> c_int;
    pub fn ungetc(a: c_int, f: *mut FILE) -> c_int;
    pub fn fputc(a: c_int, f: *mut FILE) -> c_int;
    pub fn putc(a: c_int, f: *mut FILE) -> c_int;
    pub fn putchar(a: c_int) -> c_int;
    pub fn fputs(a: *const c_char, f: *mut FILE) -> c_int;
    pub fn puts(a: *const c_char) -> c_int;
    pub fn perror(a: *const c_char);
    pub fn srand(a: c_uint);
    pub fn atexit(a: extern "C" fn()) -> c_int;
    pub fn at_quick_exit(a: extern "C" fn()) -> c_int;
    pub fn quick_exit(a: c_int) -> !;
    pub fn posix_memalign(a: *mut *mut c_void, b: size_t, c: size_t) -> c_int;
    pub fn rand_r(a: *mut c_uint) -> c_int;
    pub fn random() -> c_long;
    pub fn srandom(a: c_uint);
    pub fn putenv(a: *mut c_char) -> c_int;
    pub fn clock() -> clock_t;
    pub fn time(a: *mut time_t) -> time_t;
    pub fn difftime(a: time_t, b: time_t) -> c_double;
    pub fn mktime(a: *mut tm) -> time_t;
    pub fn strftime(a: *mut c_char, b: size_t, c: *const c_char, d: *const tm) -> size_t;
    pub fn gmtime(a: *const time_t) -> *mut tm;
    pub fn gmtime_r(a: *const time_t, b: *mut tm) -> *mut tm;
    pub fn localtime(a: *const time_t) -> *mut tm;
    pub fn localtime_r(a: *const time_t, b: *mut tm) -> *mut tm;
    pub fn asctime_r(a: *const tm, b: *mut c_char) -> *mut c_char;
    pub fn ctime_r(a: *const time_t, b: *mut c_char) -> *mut c_char;

    static _CLOCK_MONOTONIC: u8;
    static _CLOCK_PROCESS_CPUTIME_ID: u8;
    static _CLOCK_REALTIME: u8;
    static _CLOCK_THREAD_CPUTIME_ID: u8;
    pub fn nanosleep(a: *const timespec, b: *mut timespec) -> c_int;
    pub fn clock_getres(a: clockid_t, b: *mut timespec) -> c_int;
    pub fn clock_gettime(a: clockid_t, b: *mut timespec) -> c_int;
    pub fn clock_nanosleep(a: clockid_t, a2: c_int, b: *const timespec, c: *mut timespec) -> c_int;

    pub fn isalnum(c: c_int) -> c_int;
    pub fn isalpha(c: c_int) -> c_int;
    pub fn iscntrl(c: c_int) -> c_int;
    pub fn isdigit(c: c_int) -> c_int;
    pub fn isgraph(c: c_int) -> c_int;
    pub fn islower(c: c_int) -> c_int;
    pub fn isprint(c: c_int) -> c_int;
    pub fn ispunct(c: c_int) -> c_int;
    pub fn isspace(c: c_int) -> c_int;
    pub fn isupper(c: c_int) -> c_int;
    pub fn isxdigit(c: c_int) -> c_int;
    pub fn isblank(c: c_int) -> c_int;
    pub fn tolower(c: c_int) -> c_int;
    pub fn toupper(c: c_int) -> c_int;
    pub fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int, size: size_t) -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
    pub fn atoi(s: *const c_char) -> c_int;
    pub fn atof(s: *const c_char) -> c_double;
    pub fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double;
    pub fn strtof(s: *const c_char, endp: *mut *mut c_char) -> c_float;
    pub fn strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_long;
    pub fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong;

    pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    pub fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char;
    pub fn strncat(s: *mut c_char, ct: *const c_char, n: size_t) -> *mut c_char;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int;
    pub fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strdup(cs: *const c_char) -> *mut c_char;
    pub fn strndup(cs: *const c_char, n: size_t) -> *mut c_char;
    pub fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t;
    pub fn strerror(n: c_int) -> *mut c_char;
    pub fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char;
    pub fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t;

    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    pub fn fprintf(stream: *mut ::FILE, format: *const ::c_char, ...) -> ::c_int;
    pub fn printf(format: *const ::c_char, ...) -> ::c_int;
    pub fn snprintf(s: *mut ::c_char, n: ::size_t, format: *const ::c_char, ...) -> ::c_int;
    pub fn sprintf(s: *mut ::c_char, format: *const ::c_char, ...) -> ::c_int;
    pub fn fscanf(stream: *mut ::FILE, format: *const ::c_char, ...) -> ::c_int;
    pub fn scanf(format: *const ::c_char, ...) -> ::c_int;
    pub fn sscanf(s: *const ::c_char, format: *const ::c_char, ...) -> ::c_int;
    pub fn getchar_unlocked() -> ::c_int;
    pub fn putchar_unlocked(c: ::c_int) -> ::c_int;

    pub fn shutdown(socket: ::c_int, how: ::c_int) -> ::c_int;
    pub fn fstat(fildes: ::c_int, buf: *mut stat) -> ::c_int;
    pub fn mkdir(path: *const c_char, mode: mode_t) -> ::c_int;
    pub fn stat(path: *const c_char, buf: *mut stat) -> ::c_int;
    pub fn fdopen(fd: ::c_int, mode: *const c_char) -> *mut ::FILE;
    pub fn fileno(stream: *mut ::FILE) -> ::c_int;
    pub fn open(path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
    pub fn creat(path: *const c_char, mode: mode_t) -> ::c_int;
    pub fn fcntl(fd: ::c_int, cmd: ::c_int, ...) -> ::c_int;
    pub fn opendir(dirname: *const c_char) -> *mut ::DIR;
    pub fn fdopendir(fd: ::c_int) -> *mut ::DIR;
    pub fn readdir(dirp: *mut ::DIR) -> *mut ::dirent;
    pub fn closedir(dirp: *mut ::DIR) -> ::c_int;
    pub fn rewinddir(dirp: *mut ::DIR);
    pub fn dirfd(dirp: *mut ::DIR) -> ::c_int;
    pub fn seekdir(dirp: *mut ::DIR, loc: ::c_long);
    pub fn telldir(dirp: *mut ::DIR) -> ::c_long;

    pub fn openat(dirfd: ::c_int, pathname: *const ::c_char, flags: ::c_int, ...) -> ::c_int;
    pub fn fstatat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        buf: *mut stat,
        flags: ::c_int,
    ) -> ::c_int;
    pub fn linkat(
        olddirfd: ::c_int,
        oldpath: *const ::c_char,
        newdirfd: ::c_int,
        newpath: *const ::c_char,
        flags: ::c_int,
    ) -> ::c_int;
    pub fn mkdirat(dirfd: ::c_int, pathname: *const ::c_char, mode: ::mode_t) -> ::c_int;
    pub fn readlinkat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        buf: *mut ::c_char,
        bufsiz: ::size_t,
    ) -> ::ssize_t;
    pub fn renameat(
        olddirfd: ::c_int,
        oldpath: *const ::c_char,
        newdirfd: ::c_int,
        newpath: *const ::c_char,
    ) -> ::c_int;
    pub fn symlinkat(
        target: *const ::c_char,
        newdirfd: ::c_int,
        linkpath: *const ::c_char,
    ) -> ::c_int;
    pub fn unlinkat(dirfd: ::c_int, pathname: *const ::c_char, flags: ::c_int) -> ::c_int;

    pub fn access(path: *const c_char, amode: ::c_int) -> ::c_int;
    pub fn close(fd: ::c_int) -> ::c_int;
    pub fn fpathconf(filedes: ::c_int, name: ::c_int) -> c_long;
    pub fn getopt(argc: ::c_int, argv: *const *mut c_char, optstr: *const c_char) -> ::c_int;
    pub fn isatty(fd: ::c_int) -> ::c_int;
    pub fn link(src: *const c_char, dst: *const c_char) -> ::c_int;
    pub fn lseek(fd: ::c_int, offset: off_t, whence: ::c_int) -> off_t;
    pub fn pathconf(path: *const c_char, name: ::c_int) -> c_long;
    pub fn rmdir(path: *const c_char) -> ::c_int;
    pub fn sleep(secs: ::c_uint) -> ::c_uint;
    pub fn unlink(c: *const c_char) -> ::c_int;
    pub fn pread(fd: ::c_int, buf: *mut ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t;
    pub fn pwrite(fd: ::c_int, buf: *const ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t;

    pub fn lstat(path: *const c_char, buf: *mut stat) -> ::c_int;

    pub fn fsync(fd: ::c_int) -> ::c_int;
    pub fn fdatasync(fd: ::c_int) -> ::c_int;

    pub fn symlink(path1: *const c_char, path2: *const c_char) -> ::c_int;

    pub fn truncate(path: *const c_char, length: off_t) -> ::c_int;
    pub fn ftruncate(fd: ::c_int, length: off_t) -> ::c_int;

    pub fn getrusage(resource: ::c_int, usage: *mut rusage) -> ::c_int;

    pub fn gettimeofday(tp: *mut ::timeval, tz: *mut ::c_void) -> ::c_int;
    pub fn times(buf: *mut ::tms) -> ::clock_t;

    pub fn strerror_r(errnum: ::c_int, buf: *mut c_char, buflen: ::size_t) -> ::c_int;

    pub fn usleep(secs: ::c_uint) -> ::c_int;
    pub fn send(socket: ::c_int, buf: *const ::c_void, len: ::size_t, flags: ::c_int) -> ::ssize_t;
    pub fn recv(socket: ::c_int, buf: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::ssize_t;
    pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: ::c_int) -> ::c_int;
    pub fn setlocale(category: ::c_int, locale: *const ::c_char) -> *mut ::c_char;
    pub fn localeconv() -> *mut lconv;

    pub fn readlink(path: *const c_char, buf: *mut c_char, bufsz: ::size_t) -> ::ssize_t;

    pub fn timegm(tm: *mut ::tm) -> time_t;

    pub fn sysconf(name: ::c_int) -> ::c_long;

    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;

    pub fn fseeko(stream: *mut ::FILE, offset: ::off_t, whence: ::c_int) -> ::c_int;
    pub fn ftello(stream: *mut ::FILE) -> ::off_t;
    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t, len: ::off_t) -> ::c_int;

    pub fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;

    pub fn faccessat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        mode: ::c_int,
        flags: ::c_int,
    ) -> ::c_int;
    pub fn writev(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;
    pub fn readv(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;
    pub fn pwritev(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int, offset: ::off_t)
        -> ::ssize_t;
    pub fn preadv(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int, offset: ::off_t) -> ::ssize_t;
    pub fn posix_fadvise(fd: ::c_int, offset: ::off_t, len: ::off_t, advise: ::c_int) -> ::c_int;
    pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;
    pub fn utimensat(
        dirfd: ::c_int,
        path: *const ::c_char,
        times: *const ::timespec,
        flag: ::c_int,
    ) -> ::c_int;
    pub fn getentropy(buf: *mut ::c_void, buflen: ::size_t) -> ::c_int;
    pub fn memrchr(cx: *const ::c_void, c: ::c_int, n: ::size_t) -> *mut ::c_void;
    pub fn abs(i: c_int) -> c_int;
    pub fn labs(i: c_long) -> c_long;
    pub fn duplocale(base: ::locale_t) -> ::locale_t;
    pub fn freelocale(loc: ::locale_t);
    pub fn newlocale(mask: ::c_int, locale: *const ::c_char, base: ::locale_t) -> ::locale_t;
    pub fn uselocale(loc: ::locale_t) -> ::locale_t;
    pub fn sched_yield() -> ::c_int;
    pub fn getcwd(buf: *mut c_char, size: ::size_t) -> *mut c_char;
    pub fn chdir(dir: *const c_char) -> ::c_int;

    pub fn nl_langinfo(item: ::nl_item) -> *mut ::c_char;
    pub fn nl_langinfo_l(item: ::nl_item, loc: ::locale_t) -> *mut ::c_char;

    pub fn __wasilibc_register_preopened_fd(fd: c_int, path: *const c_char) -> c_int;
    pub fn __wasilibc_fd_renumber(fd: c_int, newfd: c_int) -> c_int;
    pub fn __wasilibc_unlinkat(fd: c_int, path: *const c_char) -> c_int;
    pub fn __wasilibc_rmdirat(fd: c_int, path: *const c_char) -> c_int;
    pub fn __wasilibc_find_relpath(
        path: *const c_char,
        abs_prefix: *mut *const c_char,
        relative_path: *mut *mut c_char,
        relative_path_len: usize,
    ) -> c_int;
    pub fn __wasilibc_tell(fd: c_int) -> ::off_t;
    pub fn __wasilibc_nocwd___wasilibc_unlinkat(dirfd: c_int, path: *const c_char) -> c_int;
    pub fn __wasilibc_nocwd___wasilibc_rmdirat(dirfd: c_int, path: *const c_char) -> c_int;
    pub fn __wasilibc_nocwd_linkat(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
        flags: c_int,
    ) -> c_int;
    pub fn __wasilibc_nocwd_symlinkat(
        target: *const c_char,
        dirfd: c_int,
        path: *const c_char,
    ) -> c_int;
    pub fn __wasilibc_nocwd_readlinkat(
        dirfd: c_int,
        path: *const c_char,
        buf: *mut c_char,
        bufsize: usize,
    ) -> isize;
    pub fn __wasilibc_nocwd_faccessat(
        dirfd: c_int,
        path: *const c_char,
        mode: c_int,
        flags: c_int,
    ) -> c_int;
    pub fn __wasilibc_nocwd_renameat(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
    ) -> c_int;
    pub fn __wasilibc_nocwd_openat_nomode(dirfd: c_int, path: *const c_char, flags: c_int)
        -> c_int;
    pub fn __wasilibc_nocwd_fstatat(
        dirfd: c_int,
        path: *const c_char,
        buf: *mut stat,
        flags: c_int,
    ) -> c_int;
    pub fn __wasilibc_nocwd_mkdirat_nomode(dirfd: c_int, path: *const c_char) -> c_int;
    pub fn __wasilibc_nocwd_utimensat(
        dirfd: c_int,
        path: *const c_char,
        times: *const ::timespec,
        flags: c_int,
    ) -> c_int;
    pub fn __wasilibc_nocwd_opendirat(dirfd: c_int, path: *const c_char) -> *mut ::DIR;
    pub fn __wasilibc_access(pathname: *const c_char, mode: c_int, flags: c_int) -> c_int;
    pub fn __wasilibc_stat(pathname: *const c_char, buf: *mut stat, flags: c_int) -> c_int;
    pub fn __wasilibc_utimens(
        pathname: *const c_char,
        times: *const ::timespec,
        flags: c_int,
    ) -> c_int;
    pub fn __wasilibc_link(oldpath: *const c_char, newpath: *const c_char, flags: c_int) -> c_int;
    pub fn __wasilibc_link_oldat(
        olddirfd: c_int,
        oldpath: *const c_char,
        newpath: *const c_char,
        flags: c_int,
    ) -> c_int;
    pub fn __wasilibc_link_newat(
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
        flags: c_int,
    ) -> c_int;
    pub fn __wasilibc_rename_oldat(
        olddirfd: c_int,
        oldpath: *const c_char,
        newpath: *const c_char,
    ) -> c_int;
    pub fn __wasilibc_rename_newat(
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
    ) -> c_int;

    pub fn arc4random() -> u32;
    pub fn arc4random_buf(a: *mut c_void, b: size_t);
    pub fn arc4random_uniform(a: u32) -> u32;
}
