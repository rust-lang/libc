pub type mode_t = u32;

s! {
    pub struct sigaction {
        pub sa_flags: ::c_uint,
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: ::sigset_t,
        _restorer: *mut ::c_void,
    }
}

pub const SYS_gettid: ::c_long = 178;
