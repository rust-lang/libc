//! Header: `qsee_log.h`

use crate::prelude::*;

extern "C" {
    pub fn qsee_printf(fmt: *const c_char, ...);
}
