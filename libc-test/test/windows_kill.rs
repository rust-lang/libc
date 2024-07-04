#![allow(bad_style, improper_ctypes, unused, deprecated)]

extern crate libc;
use libc::*;

#[cfg(any(target_os = "windows"))]
include!(concat!(env!("OUT_DIR"), "/windows_kill.rs"));

#[cfg(not(any(target_os = "windows")))]
fn main() {
    println!("PASSED 0 tests");
}
