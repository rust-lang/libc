pub type __p_sig_fn_t = ::size_t;

pub const L_tmpnam: ::c_uint = 14;
pub const TMP_MAX: ::c_uint = 0x7fff;
pub const SIGINT: ::c_int = 2;
pub const SIGILL: ::c_int = 4;
pub const SIGFPE: ::c_int = 8;
pub const SIGSEGV: ::c_int = 11;
pub const SIGTERM: ::c_int = 15;
pub const SIGABRT: ::c_int = 22;
pub const NSIG: ::c_int = 23;
pub const SIG_ERR: ::c_int = -1;

extern {
    pub fn signal(signum: ::c_int, handler: __p_sig_fn_t) -> __p_sig_fn_t;
    pub fn raise(signum: ::c_int) -> ::c_int;
    pub fn strcasecmp(s1: *const ::c_char, s2: *const ::c_char) -> ::c_int;
    pub fn strncasecmp(s1: *const ::c_char, s2: *const ::c_char,
                       n: ::size_t) -> ::c_int;
}
