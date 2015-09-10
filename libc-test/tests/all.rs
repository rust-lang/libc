extern crate libc;
extern crate libc_test;

use std::any::{Any, TypeId};
use std::mem;

use libc::*;
use libc::types::os::common::bsd43::*;

fn same(rust: u64, c: u64, attr: &str) {
    if rust != c {
        panic!("bad {}: rust: {} != c {}", attr, rust, c);
    }
}

fn align<T: Any>() -> u64 {
    // TODO: apparently these three types have less alignment in Rust on x86
    //       than they do in C this difference should.. probably be reconciled.
    //
    //       Perhaps #27195?
    if cfg!(target_pointer_width = "32") {
        if TypeId::of::<T>() == TypeId::of::<f64>() ||
           TypeId::of::<T>() == TypeId::of::<i64>() ||
           TypeId::of::<T>() == TypeId::of::<u64>() {
            return 8
        }
    }
    mem::align_of::<T>() as u64
}

macro_rules! offset_of {
    ($ty:ident, $field:ident) => (
        (&((*(0 as *const $ty)).$field)) as *const _ as u64
    )
}

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/all.rs"));
