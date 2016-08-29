// Native C types
pub type c_char = u8;

s! {
    /* Header <fcntl.h> */
    cfg_if! {
        if #[cfg(feature = "file_offset64")] {
            type flock = ::flock64;
        } else {
            pub struct flock {
                pub l_type: ::c_short,
                pub l_whence: ::c_short,
                pub l_start: ::off_t,
                pub l_len: ::off_t,
                pub l_pid: ::pid_t,
            }
        }
    }
}

