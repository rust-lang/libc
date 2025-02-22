#![cfg(not(test))]
#![deny(warnings)]

use testcrate::t2::*;

include!(concat!(env!("OUT_DIR"), "/t2gen.rs"));
