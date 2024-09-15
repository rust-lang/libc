#![allow(unused_imports)]
#![allow(deprecated)]

// Generated in `build.rs`.
include!(concat!(env!("OUT_DIR"), "/semver.rs"));

fn main() {
    // The test is about the imports created in `semver.rs`.
    println!("PASSED 1 tests");
}
