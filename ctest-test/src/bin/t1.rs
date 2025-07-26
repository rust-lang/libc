#![cfg(not(test))]
#![deny(warnings)]

use std::ffi::c_uint;

use ctest_test::t1::*;

include!(concat!(env!("OUT_DIR"), "/t1gen.rs"));
