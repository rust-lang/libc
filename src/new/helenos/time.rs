//! HelenOS time handling
//!
//! * Header file: <https://github.com/HelenOS/helenos/tree/master/uspace/lib/c/include/time.h>

pub use crate::{
    c_long,
    c_longlong,
};

pub type time_t = c_longlong;
pub type usec_t = c_longlong;

s! {
    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }
}

extern "C" {
    pub fn getuptime(tp: *mut timespec);
}
