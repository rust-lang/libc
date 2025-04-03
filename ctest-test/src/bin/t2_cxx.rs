#![cfg(not(test))]

cfg_if::cfg_if! {
    if #[cfg(has_cxx)] {
        use ctest_test::t2::*;

        include!(concat!(env!("OUT_DIR"), "/t2gen_cxx.rs"));
    } else {
        fn main() {}
    }
}
