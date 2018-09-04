#![cfg(not(test))]
#![allow(bad_style)]

extern crate testcrate;
extern crate libc;
use testcrate::t1::*;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/t1gen.rs"));
