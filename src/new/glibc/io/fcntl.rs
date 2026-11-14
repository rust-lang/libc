//! Header: `fcntl.h`
//!
//! <https://github.com/sailfishos-mirror/glibc/blob/master/io/fcntl.h>

#[cfg(target_os = "linux")]
pub use crate::new::glibc::bits::fcntl_linux::*;
