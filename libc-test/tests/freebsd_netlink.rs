#![allow(deprecated)]
#![allow(unused)]

#[cfg(target_os = "freebsd")]
#[allow(unused_imports)]
use libc::netlink::*;

#[cfg(target_os = "freebsd")]
include!(concat!(env!("OUT_DIR"), "/netlink_ctest_output.rs"));

#[cfg(not(target_os = "freebsd"))]
fn main() {
    println!("PASSED 0 tests");
}
