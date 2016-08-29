// Native C types
pub type c_char = i8;

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
                pub l_sysid: ::c_long,
                pub l_pid: ::pid_t,
                pad: [::c_long; 4],
            }
        }
    }
}

