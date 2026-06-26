use crate::prelude::*;

pub type clock_t = c_ulong;
pub type wchar_t = c_int;

// the newlib shipped with devkitPPC does not support the following components:
// - sockaddr
// - AF_INET6
// - FIONBIO
// - POLL*
// - SOL_SOCKET
// - MSG_*
