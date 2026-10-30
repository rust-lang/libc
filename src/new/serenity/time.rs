//! Header: `time.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/time.h>

pub use super::Kernel::API::POSIX::time::*;
use crate::prelude::*;

s! {
    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
    }
}

extern_ty! {
    pub type timezone;
}

pub const TIME_UTC: c_int = 0;

extern "C" {
    pub fn ctime(timep: *const crate::time_t) -> *mut c_char;
    pub fn ctime_r(timep: *const crate::time_t, buf: *mut c_char) -> *mut c_char;
    pub fn tzset();
    pub fn asctime(tm: *const tm) -> *mut c_char;
    pub fn asctime_r(tm: *const tm, buf: *mut c_char) -> *mut c_char;

    pub fn clock() -> crate::clock_t;

    pub fn clock_gettime(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int;
    pub fn clock_settime(clk_id: crate::clockid_t, tp: *const crate::timespec) -> c_int;
    pub fn clock_nanosleep(
        clk_id: crate::clockid_t,
        flags: c_int,
        req: *const crate::timespec,
        rem: *mut crate::timespec,
    ) -> c_int;
    pub fn clock_getres(clk_id: crate::clockid_t, res: *mut crate::timespec) -> c_int;

    pub fn timespec_get(tp: *mut crate::timespec, base: c_int) -> c_int;
}
