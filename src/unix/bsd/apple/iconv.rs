extern "C" {
    pub fn iconv_open(tocode: *const ::c_char, fromcode: *const ::c_char) -> iconv_t;
    pub fn iconv(
        cd: iconv_t,
        inbuf: *mut *mut ::c_char,
        inbytesleft: *mut ::size_t,
        outbuf: *mut *mut ::c_char,
        outbytesleft: *mut ::size_t,
    ) -> ::size_t;
    pub fn iconv_close(cd: iconv_t) -> ::c_int;
}
