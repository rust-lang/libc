//! Header: `netinet6/in6_var.h`
//!
//! https://github.com/freebsd/freebsd-src/blob/main/sys/netinet6/in6_var.h

use crate::prelude::*;

pub const IN6_IFF_ANYCAST: c_int = 0x01;
pub const IN6_IFF_TENTATIVE: c_int = 0x02;
pub const IN6_IFF_DUPLICATED: c_int = 0x04;
pub const IN6_IFF_DETACHED: c_int = 0x08;
pub const IN6_IFF_DEPRECATED: c_int = 0x10;
pub const IN6_IFF_NODAD: c_int = 0x20;
pub const IN6_IFF_AUTOCONF: c_int = 0x40;
pub const IN6_IFF_TEMPORARY: c_int = 0x80;
pub const IN6_IFF_PREFER_SOURCE: c_int = 0x0100;

pub const SIOCGIFAFLAG_IN6: c_ulong = 0xc1206949; // _IOWR('i', 73, in6_ifreq)
