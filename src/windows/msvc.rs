pub type _crt_signal_t = ::size_t;

pub const L_tmpnam: ::c_uint = 260;
pub const TMP_MAX: ::c_uint = 0x7fff_ffff;
pub const SIGINT: ::c_int = 2;
pub const SIGILL: ::c_int = 4;
pub const SIGABRT: ::c_int = 22;
pub const SIGFPE: ::c_int = 8;
pub const SIGSEGV: ::c_int = 11;
pub const SIGTERM: ::c_int = 15;
pub const SIG_ERR: ::c_int = -1;

extern {
    pub fn signal(signum: ::c_int, handler: _crt_signal_t) -> _crt_signal_t;
    pub fn raise(signum: ::c_int) -> ::c_int;
    #[link_name = "_stricmp"]
    pub fn stricmp(s1: *const ::c_char, s2: *const ::c_char) -> ::c_int;
    #[link_name = "_strnicmp"]
    pub fn strnicmp(s1: *const ::c_char, s2: *const ::c_char,
                    n: ::size_t) -> ::c_int;
}
