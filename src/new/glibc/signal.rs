//! Header: `signal/signal.h`

pub use super::bits::signum_generic::*;
pub use super::sysdeps::unix::linux::bits::sigaction::*;
pub use super::sysdeps::unix::linux::bits::types::siginfo_t::*;
use crate::prelude::*;

extern "C" {
    pub fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
}
