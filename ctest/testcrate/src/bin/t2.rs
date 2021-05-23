#![cfg(not(test))]
#![deny(warnings)]
#![allow(unaligned_references)]

use testcrate::t2::*;

include!(concat!(env!("OUT_DIR"), "/t2gen.rs"));
