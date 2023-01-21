
pub const AF_NETLINK: ::c_int = 38;
pub const PF_NETLINK: ::c_int = AF_NETLINK;
pub const SOL_NETLINK: ::c_int = 270;

/// nlmsg_flags: Indicateds request to kernel
pub const NLM_F_REQUEST: ::c_int = 1;
/// nlmsg_flags: Message is part of a group terminated by NLMSG_DONE msg
pub const NLM_F_MULTI: ::c_int = 2;
/// nlmsg_flags: Reply with ack message containing resulting error code
pub const NLM_F_ACK: ::c_int = 4;
/// nlmsg_flags: (not supported) Echo this request back
pub const NLM_F_ECHO: ::c_int = 8;
/// nlmsg_flags: Dump was inconsistent due to sequence change
pub const NLM_F_DUMP_INTR: ::c_int = 16;
/// nlmsg_flags: Dump was filtered as requested
pub const NLM_F_DUMP_FILTERED: ::c_int = 32;

/// GET message flag: Return the complete table
pub const NLM_F_ROOT: ::c_int = 0x100;
/// GET message flag: Return all entries matching criteria
pub const NLM_F_MATCH: ::c_int = 0x200;
/// GET message flag: Return an atomic snapshot (ignored)
pub const NLM_F_ATOMIC: ::c_int = 0x400;
pub const NLM_F_DUMP: ::c_int = NLM_F_ROOT | NLM_F_MATCH;

/// NEW request flag: Replace existing matching config object
pub const NLM_F_REPLACE: ::c_int = 0x100;
/// NEW request flag: Don't replace the object if exists
pub const NLM_F_EXCL: ::c_int = 0x200;
/// NEW request flag: Create if it does not exist
pub const NLM_F_CREATE: ::c_int = 0x400;
/// NEW request flag: Add to end of list
pub const NLM_F_APPEND: ::c_int = 0x800;


/// DELETE message flag: Do not delete recursively
pub const NLM_F_NONREC: ::c_int = 0x100;

/// ACK Message flag: request was capped
pub const NLM_F_CAPPED: ::c_int = 0x100;
/// ACK Message flag: extended ACK TVLs were included
pub const NLM_F_ACK_TLVS: ::c_int = 0x200;


/// standard message type (nlmsg_type): Message is ignored
pub const NLMSG_NOOP: ::c_int = 0x1;
/// reply error code reporting
pub const NLMSG_ERROR: ::c_int = 0x2;
/// Message terminates a multipart message
pub const NLMSG_DONE: ::c_int = 0x3;
/// overrun detected, data is lost
pub const NLMSG_OVERRUN: ::c_int = 0x4;
/// < 0x10: reserved control messages
pub const NLMSG_MIN_TYPE: ::c_int = 0x10;

// Defition of numbers assigned to the netlink subsystems.

/// Routing/device hook
pub const NETLINK_ROUTE: ::c_int = 0;
/// not supported
pub const NETLINK_UNUSED: ::c_int = 1;
/// not supported
pub const NETLINK_USERSOCK: ::c_int = 2;
/// not supported
pub const NETLINK_FIREWALL: ::c_int = 3;
/// not supported
pub const NETLINK_SOCK_DIAG: ::c_int = 4;
/// not supported
pub const NETLINK_NFLOG: ::c_int = 5;
/// not supported
pub const NETLINK_XFRM: ::c_int = 6;
/// not supported
pub const NETLINK_SELINUX: ::c_int = 7;
/// not supported
pub const NETLINK_ISCSI: ::c_int = 8;
/// not supported
pub const NETLINK_AUDIT: ::c_int = 9;
/// not supported
pub const NETLINK_FIB_LOOKUP: ::c_int = 10;
/// not supported
pub const NETLINK_CONNECTOR: ::c_int = 11;
/// not supported
pub const NETLINK_NETFILTER: ::c_int = 12;
/// not supported
pub const NETLINK_IP6_FW: ::c_int = 13;
/// not supported
pub const NETLINK_DNRTMSG: ::c_int = 14;
/// not supported
pub const NETLINK_KOBJECT_UEVENT: ::c_int = 15;
/// Generic netlink (dynamic families)
pub const NETLINK_GENERIC: ::c_int = 16;

// Netlink socket operations

/// Subscribe for the specified group notifications
pub const NETLINK_ADD_MEMBERSHIP: ::c_int = 1;
/// Unsubscribe from the specified group
pub const NETLINK_DROP_MEMBERSHIP: ::c_int = 2;
/// XXX: not supported
pub const NETLINK_PKTINFO: ::c_int = 3;
/// XXX: not supported
pub const NETLINK_BROADCAST_ERROR: ::c_int = 4;
/// XXX: not supported
pub const NETLINK_NO_ENOBUFS: ::c_int = 5;
/// XXX: not supported
pub const NETLINK_RX_RING: ::c_int = 6;
/// XXX: not supported
pub const NETLINK_TX_RING: ::c_int = 7;
/// XXX: not supported
pub const NETLINK_LISTEN_ALL_NSID: ::c_int = 8;

pub const NETLINK_LIST_MEMBERSHIPS: ::c_int = 9;
/// Send only original message header in the reply
pub const NETLINK_CAP_ACK: ::c_int = 10;
/// Ack support for receiving additional TLVs in ack
pub const NETLINK_EXT_ACK: ::c_int = 11;
/// Strick header checking
pub const NETLINK_GET_STRICT_CHK: ::c_int = 12;

/// NETLINK_ROUTE message: creates new interface
pub const NL_RTM_NEWLINK: u16      = 16;
/// NETLINK_ROUTE message: deletes matching interface
pub const NL_RTM_DELLINK: u16      = 17;
/// NETLINK_ROUTE message: lists matching interfaces
pub const NL_RTM_GETLINK: u16      = 18;
/// not supported
pub const NL_RTM_SETLINK: u16      = 19;
/// not supported
pub const NL_RTM_NEWADDR: u16      = 20;
/// not supported
pub const NL_RTM_DELADDR: u16      = 21;
/// NETLINK_ROUTE message: lists matching ifaddrs
pub const NL_RTM_GETADDR: u16      = 22;
/// NETLINK_ROUTE message: adds or changes a route
pub const NL_RTM_NEWROUTE: u16     = 24;
/// NETLINK_ROUTE message: deletes matching route
pub const NL_RTM_DELROUTE: u16     = 25;
/// NETLINK_ROUTE message: lists matching routes
pub const NL_RTM_GETROUTE: u16     = 26;
/// NETLINK_ROUTE message: creates new arp/ndp entry
pub const NL_RTM_NEWNEIGH: u16     = 28;
/// NETLINK_ROUTE message: deletes matching arp/ndp entry
pub const NL_RTM_DELNEIGH: u16     = 29;
/// NETLINK_ROUTE message: lists matching arp/ndp entry
pub const NL_RTM_GETNEIGH: u16     = 30;
/// not supported
pub const NL_RTM_NEWRULE: u16      = 32;
/// not supported
pub const NL_RTM_DELRULE: u16      = 33;
/// not supported
pub const NL_RTM_GETRULE: u16      = 34;
/// not supported
pub const NL_RTM_NEWQDISC: u16     = 36;
/// not supported
pub const NL_RTM_DELQDISC: u16     = 37;
/// not supported
pub const NL_RTM_GETQDISC: u16     = 38;
/// not supported
pub const NL_RTM_NEWTCLASS: u16    = 40;
/// not supported
pub const NL_RTM_DELTCLASS: u16    = 41;
/// not supported
pub const NL_RTM_GETTCLASS: u16    = 42;
/// not supported
pub const NL_RTM_NEWTFILTER: u16   = 44;
/// not supported
pub const NL_RTM_DELTFILTER: u16   = 45;
/// not supported
pub const NL_RTM_GETTFILTER: u16   = 46;
/// not supported
pub const NL_RTM_NEWACTION: u16    = 48;
/// not supported
pub const NL_RTM_DELACTION: u16    = 49;
/// not supported
pub const NL_RTM_GETACTION: u16    = 50;
/// not supported
pub const NL_RTM_NEWPREFIX: u16    = 52;
/// not supported
pub const NL_RTM_GETMULTICAST: u16 = 58;
/// not supported
pub const NL_RTM_GETANYCAST: u16   = 62;
/// not supported
pub const NL_RTM_NEWNEIGHTBL: u16  = 64;
/// not supported
pub const NL_RTM_GETNEIGHTBL: u16  = 66;
/// not supported
pub const NL_RTM_SETNEIGHTBL: u16  = 67;
/// not supported
pub const NL_RTM_NEWNDUSEROPT: u16 = 68;
/// not supported
pub const NL_RTM_NEWADDRLABEL: u16 = 72;
/// not supported
pub const NL_RTM_DELADDRLABEL: u16 = 73;
/// not supported
pub const NL_RTM_GETADDRLABEL: u16 = 74;
/// not supported
pub const NL_RTM_GETDCB: u16       = 78;
/// not supported
pub const NL_RTM_SETDCB: u16       = 79;
/// not supported
pub const NL_RTM_NEWNETCONF: u16   = 80;
/// not supported
pub const NL_RTM_GETNETCONF: u16   = 82;
/// not supported
pub const NL_RTM_NEWMDB: u16       = 84;
/// not supported
pub const NL_RTM_DELMDB: u16       = 85;
/// not supported
pub const NL_RTM_GETMDB: u16       = 86;
/// not supported
pub const NL_RTM_NEWNSID: u16      = 88;
/// not supported
pub const NL_RTM_DELNSID: u16      = 89;
/// not supported
pub const NL_RTM_GETNSID: u16      = 90;
/// not supported
pub const NL_RTM_NEWSTATS: u16	   = 92;
/// not supported
pub const NL_RTM_GETSTATS: u16	   = 94;
/// NETLINK_ROUTE message: creates new user nexhtop
pub const NL_RTM_NEWNEXTHOP: u16   = 104;
/// NETLINK_ROUTE message: deletes matching nexthop
pub const NL_RTM_DELNEXTHOP: u16   = 105;
/// NETLINK_ROUTE message: lists created user nexthops
pub const NL_RTM_GETNEXTHOP: u16   = 106;

// userspace compat definitions for RTNLGRP_*
pub const RTMGRP_LINK: ::c_int = 0x00001;
pub const RTMGRP_NOTIFY: ::c_int = 0x00002;
pub const RTMGRP_NEIGH: ::c_int = 0x00004;
pub const RTMGRP_TC: ::c_int = 0x00008;
pub const RTMGRP_IPV4_IFADDR: ::c_int = 0x00010;
pub const RTMGRP_IPV4_MROUTE: ::c_int = 0x00020;
pub const RTMGRP_IPV4_ROUTE: ::c_int = 0x00040;
pub const RTMGRP_IPV4_RULE: ::c_int = 0x00080;
pub const RTMGRP_IPV6_IFADDR: ::c_int = 0x00100;
pub const RTMGRP_IPV6_MROUTE: ::c_int = 0x00200;
pub const RTMGRP_IPV6_ROUTE: ::c_int = 0x00400;
pub const RTMGRP_IPV6_IFINFO: ::c_int = 0x00800;
pub const RTMGRP_DECnet_IFADDR: ::c_int = 0x01000;
pub const RTMGRP_DECnet_ROUTE: ::c_int = 0x04000;
pub const RTMGRP_IPV6_PREFIX: ::c_int = 0x20000;

// enum rtnetlink_groups
pub const RTNLGRP_NONE: ::c_uint = 0x00;
pub const RTNLGRP_LINK: ::c_uint = 0x01;
pub const RTNLGRP_NOTIFY: ::c_uint = 0x02;
pub const RTNLGRP_NEIGH: ::c_uint = 0x03;
pub const RTNLGRP_TC: ::c_uint = 0x04;
pub const RTNLGRP_IPV4_IFADDR: ::c_uint = 0x05;
pub const RTNLGRP_IPV4_MROUTE: ::c_uint = 0x06;
pub const RTNLGRP_IPV4_ROUTE: ::c_uint = 0x07;
pub const RTNLGRP_IPV4_RULE: ::c_uint = 0x08;
pub const RTNLGRP_IPV6_IFADDR: ::c_uint = 0x09;
pub const RTNLGRP_IPV6_MROUTE: ::c_uint = 0x0a;
pub const RTNLGRP_IPV6_ROUTE: ::c_uint = 0x0b;
pub const RTNLGRP_IPV6_IFINFO: ::c_uint = 0x0c;
pub const RTNLGRP_DECnet_IFADDR: ::c_uint = 0x0d;
pub const RTNLGRP_NOP2: ::c_uint = 0x0e;
pub const RTNLGRP_DECnet_ROUTE: ::c_uint = 0x0f;
pub const RTNLGRP_DECnet_RULE: ::c_uint = 0x10;
pub const RTNLGRP_NOP4: ::c_uint = 0x11;
pub const RTNLGRP_IPV6_PREFIX: ::c_uint = 0x12;
pub const RTNLGRP_IPV6_RULE: ::c_uint = 0x13;
pub const RTNLGRP_ND_USEROPT: ::c_uint = 0x14;
pub const RTNLGRP_PHONET_IFADDR: ::c_uint = 0x15;
pub const RTNLGRP_PHONET_ROUTE: ::c_uint = 0x16;
pub const RTNLGRP_DCB: ::c_uint = 0x17;
pub const RTNLGRP_IPV4_NETCONF: ::c_uint = 0x18;
pub const RTNLGRP_IPV6_NETCONF: ::c_uint = 0x19;
pub const RTNLGRP_MDB: ::c_uint = 0x1a;
pub const RTNLGRP_MPLS_ROUTE: ::c_uint = 0x1b;
pub const RTNLGRP_NSID: ::c_uint = 0x1c;
pub const RTNLGRP_MPLS_NETCONF: ::c_uint = 0x1d;
pub const RTNLGRP_IPV4_MROUTE_R: ::c_uint = 0x1e;
pub const RTNLGRP_IPV6_MROUTE_R: ::c_uint = 0x1f;
pub const RTNLGRP_NEXTHOP: ::c_uint = 0x20;
pub const RTNLGRP_BRVLAN: ::c_uint = 0x21;


e! {
    #[repr(u32)]
    pub enum nlmsgerr_attrs {
        /// string, error message
        NLMSGERR_ATTR_MSG = 1,
        /// u32, offset of the invalid attr from nl header
        NLMSGERR_ATTR_OFFS	= 2,
        /// binary, data to pass to userland
        NLMSGERR_ATTR_COOKIE = 3,
        // not supported
        NLMSGERR_ATTR_POLICY = 4,
    }
}

s_no_extra_traits! {
    pub struct sockaddr_nl {
        /// sizeof (sockaddr_nl)
        pub nl_len: u8,
        /// netlink family
        pub nl_family: ::sa_family_t,
        /// reserved, set to 0
        nl_pad: ::c_ushort,
        /// desired port ID, 0 for auto-select
        pub nl_pid: u32,
        /// multicast groups mask to bind to
        pub nl_groups: u32
    }
}

s! {
    /// Netlink Message Header
    pub struct nlmsghdr {
	    /// Length of message including header
        pub nlmsg_len: u32,
    	/// Message type identifier
        pub nlmsg_type: u16,
	    /// Flags (NLM_F_)
        pub nlmsg_flags: u16,
    	/// Sequence number
        pub nlmsg_seq: u32,
	    /// Sending process port ID
        pub nlmsg_pid: u32,
    }

    /// Netlink ACK Message
    pub struct nlmsgerr {
        pub error: ::c_int,
        pub msg: nlmsghdr,
    }

    /// Base netlink attribute TLV header
    pub struct nlattr {
        /// Total attribute length
        pub nla_len: u16,
        /// Attribute type
        pub nla_type: u16,
    }
}
