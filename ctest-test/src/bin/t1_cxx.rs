#![cfg(not(test))]

cfg_if::cfg_if! {
    if #[cfg(has_cxx)] {
        use ctest_test::t1::*;
        use std::ffi::c_uint;

        include!(concat!(env!("OUT_DIR"), "/t1gen_cxx.rs"));
    } else {
        fn main() {}
    }
}
