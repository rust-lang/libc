//! Header: `sys/socket.h`
//!
//! https://github.com/freebsd/freebsd-src/blob/main/sys/sys/socket.h

use crate::prelude::*;

pub const SO_RERROR: c_int = 0x0002_0000;

pub const AF_NETLINK: c_int = 38;
pub const PF_NETLINK: c_int = AF_NETLINK;
