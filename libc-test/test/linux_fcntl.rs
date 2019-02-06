#![allow(bad_style, improper_ctypes, unused, deprecated)]

extern crate libc;

use libc::*;

include!(concat!(env!("OUT_DIR"), "/linux_fcntl.rs"));
