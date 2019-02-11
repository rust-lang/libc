#![cfg(not(test))]
#![allow(bad_style)]

extern crate libc;
extern crate testcrate;
use libc::*;
use testcrate::t1::*;

include!(concat!(env!("OUT_DIR"), "/t1gen_cxx.rs"));
