//! NetBSD libc.
//!
//! * Headers: <https://github.com/NetBSD/src/tree/trunk/include>
//! * Sys headers: <https://github.com/NetBSD/src/tree/trunk/sys/sys>
//! * Manual pages: <https://man.netbsd.org/>

pub(crate) mod sys;
pub(crate) mod unistd;
pub(crate) mod utmp_;
pub(crate) mod utmpx_;
