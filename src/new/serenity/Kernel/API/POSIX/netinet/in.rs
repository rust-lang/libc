//! File `Kernel/API/POSIX/netinet/in.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/netinet/in.h>
use crate::prelude::*;

pub const IN_CLASSA_NET: u32 = 0xff000000;
pub const IN_CLASSB_NET: u32 = 0xffff0000;
pub const IN_CLASSC_NET: u32 = 0xffffff00;

pub const IN_LOOPBACKNET: u32 = 127;

pub const IP_TOS: c_int = 1;
pub const IP_TTL: c_int = 2;
pub const IP_MULTICAST_LOOP: c_int = 3;
pub const IP_ADD_MEMBERSHIP: c_int = 4;
pub const IP_DROP_MEMBERSHIP: c_int = 5;
pub const IP_MULTICAST_IF: c_int = 6;
pub const IP_MULTICAST_TTL: c_int = 7;
pub const IP_BLOCK_SOURCE: c_int = 8;
pub const IP_UNBLOCK_SOURCE: c_int = 9;
pub const IP_OPTIONS: c_int = 10;

pub const IPTOS_DELAY: c_int = 16;
pub const IPTOS_THROUGHPUT: c_int = 8;
pub const IPTOS_RELIABILITY: c_int = 4;

pub const MCAST_JOIN_SOURCE_GROUP: c_int = 100;
pub const MCAST_LEAVE_SOURCE_GROUP: c_int = 101;

pub const IPPORT_RESERVED: c_int = 1024;
pub const IPPORT_USERRESERVED: c_int = 5000;

s! {
    pub struct in_addr {
        pub s_addr: u32,
    }

    pub struct sockaddr_in {
        pub sin_family: crate::sa_family_t,
        pub sin_port: crate::in_port_t,
        pub sin_addr: crate::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct ip_mreq {
        pub imr_multiaddr: crate::in_addr,
        pub imr_interface: crate::in_addr,
    }

    pub struct group_source_req {
        pub gsr_interface: u32,
        pub gsr_group: crate::sockaddr_storage,
        pub gsr_source: crate::sockaddr_storage,
    }

    pub struct ip_mreq_source {
        pub imr_multiaddr: crate::in_addr,
        pub imr_sourceaddr: crate::in_addr,
        pub imr_interface: crate::in_addr,
    }
}

pub const IPV6_UNICAST_HOPS: c_int = 1;
pub const IPV6_MULTICAST_HOPS: c_int = 2;
pub const IPV6_MULTICAST_LOOP: c_int = 3;
pub const IPV6_MULTICAST_IF: c_int = 4;
pub const IPV6_ADD_MEMBERSHIP: c_int = 5;
pub const IPV6_DROP_MEMBERSHIP: c_int = 6;
pub const IP_ADD_SOURCE_MEMBERSHIP: c_int = 7;
pub const IP_DROP_SOURCE_MEMBERSHIP: c_int = 8;
pub const IPV6_V6ONLY: c_int = 9;

pub const IPV6_JOIN_GROUP: c_int = 5;
pub const IPV6_LEAVE_GROUP: c_int = 6;

pub const IPV6_RECVPKTINFO: c_int = 10;
pub const IPV6_PKTINFO: c_int = 11;
pub const IPV6_RECVHOPLIMIT: c_int = 12;
pub const IPV6_HOPLIMIT: c_int = 13;

s! {
    pub struct in6_pktinfo {
        pub ipi6_addr: crate::in6_addr,
        pub ipi6_ifindex: u32,
    }

    pub struct sockaddr_in6 {
        pub sin6_family: crate::sa_family_t,
        pub sin6_port: crate::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: crate::in6_addr,
        pub sin6_scope_id: u32,
    }
}
