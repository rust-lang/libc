//! Header: `unistd.h`

pub use crate::new::common::posix::unistd::{
    STDERR_FILENO,
    STDIN_FILENO,
    STDOUT_FILENO,
};
extern "C" {
    pub fn crypt(key: *const crate::c_char, salt: *const crate::c_char) -> *mut crate::c_char;
}
