#![cfg(not(test))]
#![deny(warnings)]

extern crate testcrate;
use testcrate::t2::*;

include!(concat!(env!("OUT_DIR"), "/t2gen_cxx.rs"));
