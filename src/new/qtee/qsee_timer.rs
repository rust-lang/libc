//! Header: `qsee_timer.h`

use crate::prelude::*;

extern "C" {
    pub fn qsee_get_uptime() -> c_ulonglong;
}
