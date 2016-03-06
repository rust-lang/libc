pub type mode_t = u16;

s! {
    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: ::sigset_t,
        pub sa_flags: ::c_ulong,
        pub sa_restorer: ::dox::Option<extern fn()>,
    }
}

pub const SYS_gettid: ::c_long = 224;
