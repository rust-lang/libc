//! Header: `netlink/netlink.h`
//!
//! <https://github.com/freebsd/freebsd-src/blob/df9d6403caa6426e92f5e100602f4d2be474bbae/sys/netlink/netlink.h>

use crate::prelude::*;

s! {
    pub struct sockaddr_nl {
        pub nl_len: u8,
        pub nl_family: crate::sa_family_t,
        nl_pad: Padding<u16>,
        pub nl_pid: u32,
        pub nl_groups: u32,
    }
}

pub const SOL_NETLINK: c_int = 270;
pub const NETLINK_ADD_MEMBERSHIP: c_int = 1;
pub const NETLINK_DROP_MEMBERSHIP: c_int = 2;
pub const NETLINK_PKTINFO: c_int = 3;
pub const NETLINK_BROADCAST_ERROR: c_int = 4;
pub const NETLINK_NO_ENOBUFS: c_int = 5;
pub const NETLINK_RX_RING: c_int = 6;
pub const NETLINK_TX_RING: c_int = 7;
pub const NETLINK_LISTEN_ALL_NSID: c_int = 8;
pub const NETLINK_LIST_MEMBERSHIPS: c_int = 9;
pub const NETLINK_CAP_ACK: c_int = 10;
pub const NETLINK_EXT_ACK: c_int = 11;
pub const NETLINK_GET_STRICT_CHK: c_int = 12;

pub const NLM_F_REQUEST: c_int = 0x01;
pub const NLM_F_MULTI: c_int = 0x02;
pub const NLM_F_ACK: c_int = 0x04;
pub const NLM_F_ECHO: c_int = 0x08;
pub const NLM_F_DUMP_INTR: c_int = 0x10;
pub const NLM_F_DUMP_FILTERED: c_int = 0x20;

pub const NLM_F_ROOT: c_int = 0x100;
pub const NLM_F_MATCH: c_int = 0x200;
pub const NLM_F_ATOMIC: c_int = 0x400;
pub const NLM_F_DUMP: c_int = NLM_F_ROOT | NLM_F_MATCH;

pub const NLM_F_REPLACE: c_int = 0x100;
pub const NLM_F_EXCL: c_int = 0x200;
pub const NLM_F_CREATE: c_int = 0x400;
pub const NLM_F_APPEND: c_int = 0x800;

pub const NLM_F_NONREC: c_int = 0x100;

pub const NLM_F_CAPPED: c_int = 0x100;
pub const NLM_F_ACK_TLVS: c_int = 0x200;

pub const NLMSG_NOOP: c_int = 0x1;
pub const NLMSG_ERROR: c_int = 0x2;
pub const NLMSG_DONE: c_int = 0x3;
pub const NLMSG_OVERRUN: c_int = 0x4;

pub const NETLINK_ROUTE: c_int = 0;
pub const NETLINK_UNUSED: c_int = 1;
pub const NETLINK_USERSOCK: c_int = 2;
pub const NETLINK_FIREWALL: c_int = 3;
pub const NETLINK_SOCK_DIAG: c_int = 4;
pub const NETLINK_NFLOG: c_int = 5;
pub const NETLINK_XFRM: c_int = 6;
pub const NETLINK_SELINUX: c_int = 7;
pub const NETLINK_ISCSI: c_int = 8;
pub const NETLINK_AUDIT: c_int = 9;
pub const NETLINK_FIB_LOOKUP: c_int = 10;
pub const NETLINK_CONNECTOR: c_int = 11;
pub const NETLINK_NETFILTER: c_int = 12;
pub const NETLINK_IP6_FW: c_int = 13;
pub const NETLINK_DNRTMSG: c_int = 14;
pub const NETLINK_KOBJECT_UEVENT: c_int = 15;
pub const NETLINK_GENERIC: c_int = 16;

pub const NL_ITEM_ALIGN_SIZE: c_int = size_of::<u32>() as c_int;
pub const NLMSG_ALIGNTO: c_int = NL_ITEM_ALIGN_SIZE;
