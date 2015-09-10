extern crate libc;
extern crate libc_test;

use std::mem;

use libc::*;
use libc::types::os::common::bsd43::*;

fn same(rust: u64, c: u64, attr: &str) {
    if rust != c {
        panic!("bad {}: rust: {} != c {}", attr, rust, c);
    }
}

macro_rules! offset_of {
    ($ty:ident, $field:ident) => (
        (&((*(0 as *const $ty)).$field)) as *const _ as u64
    )
}

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/all.rs"));
