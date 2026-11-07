//! SerenityOS libc.
//!
//! * Headers: <https://github.com/SerenityOS/serenity/tree/master/Userland/Libraries/LibC>
//! * Manual pages: <https://man.serenityos.org/>

use crate::prelude::*;

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        pub type wchar_t = c_uint;
    } else {
        pub type wchar_t = c_int;
    }
}

// Most structs and constants shared b/w Kernel and LibC
// are defined in Kernel/API headers.
mod Kernel {
    pub(crate) mod API {
        pub(crate) mod Ioctl;
        pub(crate) mod POSIX {
            pub(crate) mod dirent;
            pub(crate) mod errno;
            pub(crate) mod fcntl;
            pub(crate) mod netinet {
                pub(crate) mod r#in;
                pub(crate) mod tcp;
            }
            pub(crate) mod poll;
            pub(crate) mod sched;
            pub(crate) mod select;
            pub(crate) mod signal;
            pub(crate) mod signal_numbers;
            pub(crate) mod termios;
            pub(crate) mod time;
            pub(crate) mod unistd;
            pub(crate) mod sys {
                pub(crate) mod limits;
                pub(crate) mod mman;
                pub(crate) mod socket;
                pub(crate) mod stat;
                pub(crate) mod statvfs;
                pub(crate) mod types;
                pub(crate) mod uio;
                pub(crate) mod un;
                pub(crate) mod wait;
            }
        }
    }
}

// LibC Headers
pub(crate) mod dirent_mod;
pub(crate) mod dlfcn;
pub(crate) mod errno;
pub(crate) mod fcntl;
pub(crate) mod fd_set_mod;
pub(crate) mod locale;
pub(crate) mod netdb;
pub(crate) mod netinet {
    pub(crate) mod r#in;
    pub(crate) mod tcp;
}
pub(crate) mod poll;
pub(crate) mod pthread;
pub(crate) mod pwd;
pub(crate) mod sched;
pub(crate) mod select;
pub(crate) mod semaphore;
pub(crate) mod signal;
pub(crate) mod stdlib;
pub(crate) mod sys {
    pub(crate) mod file;
    pub(crate) mod ioctl;
    pub(crate) mod limits;
    pub(crate) mod mman;
    pub(crate) mod resource;
    pub(crate) mod socket;
    pub(crate) mod stat;
    pub(crate) mod statvfs;
    pub(crate) mod types;
    pub(crate) mod uio;
    pub(crate) mod un;
    pub(crate) mod wait;
}
pub(crate) mod termios_mod;
pub(crate) mod time;
pub(crate) mod unistd;
