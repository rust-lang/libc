#![allow(bad_style)]
extern crate libc;

use libc::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
