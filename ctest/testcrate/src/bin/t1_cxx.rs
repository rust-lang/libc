#![cfg(not(test))]
#![deny(warnings)]
#![allow(unaligned_references)]

use libc::*;
use testcrate::t1::*;

include!(concat!(env!("OUT_DIR"), "/t1gen_cxx.rs"));
