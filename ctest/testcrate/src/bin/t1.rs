#![cfg(not(test))]
#![deny(warnings)]

extern crate libc;
extern crate testcrate;
use libc::*;
use testcrate::t1::*;

include!(concat!(env!("OUT_DIR"), "/t1gen.rs"));
