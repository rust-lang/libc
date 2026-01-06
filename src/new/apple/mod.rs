//! Apple interfaces.
//!
//! The Xcode SDK includes interfaces that are split across a couple of different libraries. Most
//! of these are available at <https://github.com/apple-oss-distributions>.

mod libc;
pub(crate) use libc::*;

mod libpthread;
pub(crate) use libpthread::pthread_;
pub(crate) use pthread_::pthread;

mod xnu;
pub(crate) use xnu::*;
