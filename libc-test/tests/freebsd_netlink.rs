#![allow(deprecated)]

#[allow(unused_imports)]
use libc::netlink::{netlink::*, netlink_generic::*};

include!(concat!(env!("OUT_DIR"), "/netlink_ctest_output.rs"));
