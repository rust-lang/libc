//! File `Kernel/API/POSIX/sys/wait.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/sys/wait.h>
use crate::prelude::*;

f! {
    pub const safe fn WIFSTOPPED(status: c_int) -> bool {
        (status & 0xff) == 0x7f
    }

    pub const safe fn WSTOPSIG(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    pub const safe fn WIFCONTINUED(status: c_int) -> bool {
        status == 0xffff
    }

    pub const safe fn WIFSIGNALED(status: c_int) -> bool {
        ((status & 0x7f) + 1) as i8 >= 2
    }

    pub const safe fn WTERMSIG(status: c_int) -> c_int {
        status & 0x7f
    }

    pub const safe fn WIFEXITED(status: c_int) -> bool {
        (status & 0x7f) == 0
    }

    pub const safe fn WEXITSTATUS(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    // Serenity doesn't have native WCOREDUMP.
    pub const safe fn WCOREDUMP(_status: c_int) -> bool {
        false
    }
}

pub const WNOHANG: c_int = 1;
pub const WUNTRACED: c_int = 2;
pub const WSTOPPED: c_int = WUNTRACED;
pub const WEXITED: c_int = 4;
pub const WCONTINUED: c_int = 8;
pub const WNOWAIT: c_int = 0x1000000;

pub type idtype_t = c_uint;
pub const P_ALL: idtype_t = 1;
pub const P_PID: idtype_t = 2;
pub const P_PGID: idtype_t = 3;
