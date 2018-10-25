#![cfg(not(test))]
#![allow(bad_style)]

extern crate testcrate;
use testcrate::t2::*;

include!(concat!(env!("OUT_DIR"), "/t2gen.rs"));
