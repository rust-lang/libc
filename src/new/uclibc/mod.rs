//! uClibc.
//!
//! * About: <https://uclibc.org/>
//! * Headers: <https://github.com/kraj/uClibc> (mirror)

#[cfg(target_os = "linux")]
pub(crate) mod pthread;
pub(crate) mod unistd;
