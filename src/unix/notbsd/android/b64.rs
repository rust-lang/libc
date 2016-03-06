pub type mode_t = u32;

pub const SYS_gettid: ::c_int = 178;

s! {
    pub struct sigaction {
        pub sa_flags: ::c_uint,
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: ::sigset_t,
        _restorer: *mut ::c_void,
    }
}
