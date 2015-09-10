extern crate libc;
extern crate libc_test;

use std::mem;

use libc::*;
use libc::types::os::common::bsd43::*;

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/all.rs"));
