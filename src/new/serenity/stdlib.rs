//! Header: `stdlib.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/stdlib.h>

use crate::prelude::*;

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;

extern "C" {
    pub fn _abort() -> !;
    pub fn malloc_size(ptr: *mut c_void) -> usize;
    pub fn malloc_good_size(s: usize) -> usize;
    pub fn serenity_dump_malloc_stats() -> c_int;
    pub fn secure_getenv(name: *const c_char) -> *mut c_char;
    pub fn serenity_putenv(new_var: *const c_char, length: usize) -> c_int;
    pub fn clearenv() -> c_int;
    pub fn getprogname() -> *const c_char;
    pub fn setprogname(name: *const c_char);
    pub fn qsort_r(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: Option<unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void) -> c_int>,
        arg: *mut c_void,
    );
    pub fn ptsname_r(fd: c_int, buffer: *mut c_char, buflen: usize) -> c_int;
    pub fn abs(n: c_int) -> c_int;
    pub fn labs(n: c_long) -> c_long;
    pub fn llabs(n: c_longlong) -> c_longlong;
    // mktemp omitted intentionally
    pub fn mkstemps(template: *mut c_char, suffixlen: c_int) -> c_int;
    // multi-byte functions omitted intentionally
}

pub const RAND_MAX: c_int = 32767;

extern "C" {
    pub fn rand() -> c_int;
    pub fn srand(seed: c_uint);
    pub fn drand48() -> c_double;
    pub fn srand48(seed: c_long);
    pub fn random() -> c_long;
    pub fn srandom(seed: c_uint);

    pub fn arc4random() -> u32;
    pub fn arc4random_buf(buf: *mut c_void, nbytes: usize);
    pub fn arc4random_uniform(upper_bound: u32) -> u32;
}

s! {
    pub struct div_t {
        pub quot: c_int,
        pub rem: c_int,
    }

    pub struct ldiv_t {
        pub quot: c_long,
        pub rem: c_long,
    }

    pub struct lldiv_t {
        pub quot: c_longlong,
        pub rem: c_longlong,
    }
}

extern "C" {
    pub fn div(numerator: c_int, denominator: c_int) -> div_t;
    pub fn ldiv(numerator: c_long, denominator: c_long) -> ldiv_t;
    pub fn lldiv(numerator: c_longlong, denominator: c_longlong) -> lldiv_t;
}
