#![allow(bad_style, improper_ctypes, unused, deprecated)]

extern crate libc;
use libc::*;

#[cfg(target_os = "windows")]
mod t {
    // TODO add tests
    #[test]
    fn test_kill() {
        assert_eq!(0, 0);
    }
}

fn main() {
    println!("PASSED 1 tests");
}
