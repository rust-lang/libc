#![allow(unused, deprecated)]

use libc::*;

#[cfg(all(target_os = "linux", target_env = "gnu"))]
include!(concat!(env!("OUT_DIR"), "/linux_itimer.rs"));

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
fn main() {
    println!("PASSED 0 tests");
}
