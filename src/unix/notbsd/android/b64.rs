s! {
    pub struct sigaction {
        pub sa_flags: ::c_uint,
        pub sa_sigaction: sighandler_t,
        pub sa_mask: sigset_t,
        _restorer: *mut ::c_void,
    }
}
