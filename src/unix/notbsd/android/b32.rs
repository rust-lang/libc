s! {
    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: ::sigset_t,
        pub sa_flags: ::c_ulong,
        pub sa_restorer: ::dox::Option<extern fn()>,
    }
}
