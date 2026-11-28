//! Header: `linux/kcmp.h`

use crate::prelude::*;

pub const KCMP_FILE: c_int = 0;
pub const KCMP_VM: c_int = 1;
pub const KCMP_FILES: c_int = 2;
pub const KCMP_FS: c_int = 3;
pub const KCMP_SIGHAND: c_int = 4;
pub const KCMP_IO: c_int = 5;
pub const KCMP_SYSVSEM: c_int = 6;
pub const KCMP_EPOLL_TFD: c_int = 7;
/// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
/// for details.
pub const KCMP_TYPES: c_int = 8;
