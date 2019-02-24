#![allow(bad_style, improper_ctypes, deprecated, unused_macros)]
extern crate cty;

use cty::*;

include!(concat!(env!("OUT_DIR"), "/main.rs"));
