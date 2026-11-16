//! Header: `qsee_time.h`

use crate::prelude::*;

pub type tztimespec_t = tztimespec;

s2! {
    #[exhaustive]
    #[repr(packed)]
    pub struct tztimespec {
        pub tv_sec: u32,
        pub tv_nsec: u32,
    }
}

extern "C" {
    pub fn time_getutcsec(tz_time_spec: *mut tztimespec_t) -> c_int;
}
