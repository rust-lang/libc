#![cfg(not(test))]
#![deny(warnings)]

use ctest_test::t2::*;

include!(concat!(env!("OUT_DIR"), "/t2gen_cxx.rs"));
