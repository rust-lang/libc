//! File `Kernel/API/POSIX/sys/un.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/sys/un.h>
use crate::prelude::*;

pub const UNIX_PATH_MAX: c_int = 108;

s! {
    pub struct sockaddr_un {
        pub sun_family: u16,
        pub sun_path: [c_char; UNIX_PATH_MAX as usize],
    }
}
