#![cfg(not(test))]
#![deny(warnings)]

use ctest_test::t1::*;
use std::ffi::c_uint;

include!(concat!(env!("OUT_DIR"), "/t1gen.rs"));
