// sys/socket.h
pub const AF_NETLINK: ::c_int = 38;
pub const PF_NETLINK: ::c_int = AF_NETLINK;

// netlink/netlink.h
pub const SOL_NETLINK: ::c_int = 270;
pub const NETLINK_ADD_MEMBERSHIP: ::c_int = 1;
pub const NETLINK_DROP_MEMBERSHIP: ::c_int = 2;
pub const NETLINK_PKTINFO: ::c_int = 3;
pub const NETLINK_BROADCAST_ERROR: ::c_int = 4;
pub const NETLINK_NO_ENOBUFS: ::c_int = 5;
pub const NETLINK_RX_RING: ::c_int = 6;
pub const NETLINK_TX_RING: ::c_int = 7;
pub const NETLINK_LISTEN_ALL_NSID: ::c_int = 8;
pub const NETLINK_LIST_MEMBERSHIPS: ::c_int = 9;
pub const NETLINK_CAP_ACK: ::c_int = 10;
pub const NETLINK_EXT_ACK: ::c_int = 11;
pub const NETLINK_GET_STRICT_CHK: ::c_int = 12;
//
pub const NLM_F_REQUEST: ::c_int = 0x01;
pub const NLM_F_MULTI: ::c_int = 0x02;
pub const NLM_F_ACK: ::c_int = 0x04;
pub const NLM_F_ECHO: ::c_int = 0x08;
pub const NLM_F_DUMP_INTR: ::c_int = 0x10;
pub const NLM_F_DUMP_FILTERED: ::c_int = 0x20;
//
pub const NLM_F_ROOT: ::c_int = 0x100;
pub const NLM_F_MATCH: ::c_int = 0x200;
pub const NLM_F_ATOMIC: ::c_int = 0x400;
pub const NLM_F_DUMP: ::c_int = NLM_F_ROOT | NLM_F_MATCH;
//
pub const NLM_F_REPLACE: ::c_int = 0x100;
pub const NLM_F_EXCL: ::c_int = 0x200;
pub const NLM_F_CREATE: ::c_int = 0x400;
pub const NLM_F_APPEND: ::c_int = 0x800;
//
pub const NLM_F_NONREC: ::c_int = 0x100;
//
pub const NLM_F_CAPPED: ::c_int = 0x100;
pub const NLM_F_ACK_TLVS: ::c_int = 0x200;
//
pub const NLMSG_NOOP: ::c_int = 0x1;
pub const NLMSG_ERROR: ::c_int = 0x2;
pub const NLMSG_DONE: ::c_int = 0x3;
pub const NLMSG_OVERRUN: ::c_int = 0x4;
//
pub const NETLINK_ROUTE: ::c_int = 0;
pub const NETLINK_UNUSED: ::c_int = 1;
pub const NETLINK_USERSOCK: ::c_int = 2;
pub const NETLINK_FIREWALL: ::c_int = 3;
pub const NETLINK_SOCK_DIAG: ::c_int = 4;
pub const NETLINK_NFLOG: ::c_int = 5;
pub const NETLINK_XFRM: ::c_int = 6;
pub const NETLINK_SELINUX: ::c_int = 7;
pub const NETLINK_ISCSI: ::c_int = 8;
pub const NETLINK_AUDIT: ::c_int = 9;
pub const NETLINK_FIB_LOOKUP: ::c_int = 10;
pub const NETLINK_CONNECTOR: ::c_int = 11;
pub const NETLINK_NETFILTER: ::c_int = 12;
pub const NETLINK_IP6_FW: ::c_int = 13;
pub const NETLINK_DNRTMSG: ::c_int = 14;
pub const NETLINK_KOBJECT_UEVENT: ::c_int = 15;
pub const NETLINK_GENERIC: ::c_int = 16;
//
const NL_ITEM_ALIGN_SIZE: ::c_int = 4; // mem::size_of::<u32>(); FIXME accept new dep?
pub const NLMSG_ALIGNTO: ::c_int = NL_ITEM_ALIGN_SIZE;

// netlink/netlink_generic.h
pub const CTRL_CMD_UNSPEC: ::c_int = 0;
pub const CTRL_CMD_NEWFAMILY: ::c_int = 1;
pub const CTRL_CMD_DELFAMILY: ::c_int = 2;
pub const CTRL_CMD_GETFAMILY: ::c_int = 3;
pub const CTRL_CMD_NEWOPS: ::c_int = 4;
pub const CTRL_CMD_DELOPS: ::c_int = 5;
pub const CTRL_CMD_GETOPS: ::c_int = 6;
pub const CTRL_CMD_NEWMCAST_GRP: ::c_int = 7;
pub const CTRL_CMD_DELMCAST_GRP: ::c_int = 8;
pub const CTRL_CMD_GETMCAST_GRP: ::c_int = 9;
pub const CTRL_CMD_GETPOLICY: ::c_int = 10;
//
pub const CTRL_ATTR_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_FAMILY_ID: ::c_int = 1;
pub const CTRL_ATTR_FAMILY_NAME: ::c_int = 2;
pub const CTRL_ATTR_VERSION: ::c_int = 3;
pub const CTRL_ATTR_HDRSIZE: ::c_int = 4;
pub const CTRL_ATTR_MAXATTR: ::c_int = 5;
pub const CTRL_ATTR_OPS: ::c_int = 6;
pub const CTRL_ATTR_MCAST_GROUPS: ::c_int = 7;
pub const CTRL_ATTR_POLICY: ::c_int = 8;
pub const CTRL_ATTR_OP_POLICY: ::c_int = 9;
pub const CTRL_ATTR_OP: ::c_int = 10;
//
pub const CTRL_ATTR_MCAST_GRP_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_MCAST_GRP_NAME: ::c_int = 1;
pub const CTRL_ATTR_MCAST_GRP_ID: ::c_int = 2;
