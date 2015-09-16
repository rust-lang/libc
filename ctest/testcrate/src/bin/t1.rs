#![cfg(not(test))]
#![allow(bad_style)]

extern crate testcrate;
use testcrate::t1::*;

include!(concat!(env!("OUT_DIR"), "/t1gen.rs"));
