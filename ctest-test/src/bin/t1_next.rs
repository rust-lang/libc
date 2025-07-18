#![cfg(not(test))]
#![deny(warnings)]

use ctest_test::t1::*;

include!(concat!(env!("OUT_DIR"), "/t1nextgen.rs"));
