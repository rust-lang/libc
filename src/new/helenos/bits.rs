//! HelenOS ABI bits
//!
//! * Headers: <https://github.com/HelenOS/helenos/tree/master/abi/include/_bits>

// `errno.h`
pub type errno_t = crate::c_int;

// `native.h`
pub type sysarg_t = crate::uintptr_t;
