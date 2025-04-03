#![cfg(not(test))]
#![deny(warnings)]

use ctest_test::t1::*;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/t1gen_cxx.rs"));
