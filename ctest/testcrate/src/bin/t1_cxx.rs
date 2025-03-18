#![cfg(not(test))]
#![deny(warnings)]

use libc::*;
use testcrate::t1::*;

include!(concat!(env!("OUT_DIR"), "/t1gen_cxx.rs"));
