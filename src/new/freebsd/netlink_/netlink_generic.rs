//! Header: `netlink/netlink_generic.h`
//!
//! <https://github.com/freebsd/freebsd-src/blob/main/sys/netlink/netlink_generic.h>

c_enum! {
    pub enum #anon {
        pub CTRL_CMD_UNSPEC,
        pub CTRL_CMD_NEWFAMILY,
        pub CTRL_CMD_DELFAMILY,
        pub CTRL_CMD_GETFAMILY,
        pub CTRL_CMD_NEWOPS,
        pub CTRL_CMD_DELOPS,
        pub CTRL_CMD_GETOPS,
        pub CTRL_CMD_NEWMCAST_GRP,
        pub CTRL_CMD_DELMCAST_GRP,
        pub CTRL_CMD_GETMCAST_GRP,
        pub CTRL_CMD_GETPOLICY,
    }

    pub enum #anon {
        pub CTRL_ATTR_UNSPEC,
        pub CTRL_ATTR_FAMILY_ID,
        pub CTRL_ATTR_FAMILY_NAME,
        pub CTRL_ATTR_VERSION,
        pub CTRL_ATTR_HDRSIZE,
        pub CTRL_ATTR_MAXATTR,
        pub CTRL_ATTR_OPS,
        pub CTRL_ATTR_MCAST_GROUPS,
        pub CTRL_ATTR_POLICY,
        pub CTRL_ATTR_OP_POLICY,
        pub CTRL_ATTR_OP,
    }

    pub enum #anon {
        pub CTRL_ATTR_MCAST_GRP_UNSPEC,
        pub CTRL_ATTR_MCAST_GRP_NAME,
        pub CTRL_ATTR_MCAST_GRP_ID,
    }
}
