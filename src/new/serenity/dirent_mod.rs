//! Header: `dirent.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/dirent.h>
pub use super::Kernel::API::POSIX::dirent::*;
use crate::prelude::*;

s! {
    pub struct dirent {
        pub d_ino: crate::ino_t,
        pub d_off: crate::off_t,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }
}

extern "C" {
    pub fn dirfd(dirp: *mut crate::DIR) -> c_int;
}
