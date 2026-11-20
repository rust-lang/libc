//! File `Kernel/API/POSIX/termios.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/termios.h>

pub const NCCS: usize = 32;

pub type tcflag_t = u32;
pub type speed_t = u32;

s! {
    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_cc: [crate::cc_t; NCCS],
        pub c_ispeed: speed_t,
        pub c_ospeed: speed_t,
    }
}
