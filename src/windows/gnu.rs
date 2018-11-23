pub const L_tmpnam: ::c_uint = 14;
pub const TMP_MAX: ::c_uint = 0x7fff;

extern {
    pub fn strcasecmp(s1: *const ::c_char, s2: *const ::c_char) -> ::c_int;
    pub fn strncasecmp(s1: *const ::c_char, s2: *const ::c_char,
                    n: ::size_t) -> ::c_int;
}
