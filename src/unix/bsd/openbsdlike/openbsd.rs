s! {
    pub struct glob_t {
        pub gl_pathc:   ::c_int,
        __unused1:      ::c_int,
        pub gl_offs:    ::c_int,
        __unused2:      ::c_int,
        pub gl_pathv:   *mut *mut ::c_char,

        __unused3: *mut ::c_void,

        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
        __unused6: *mut ::c_void,
        __unused7: *mut ::c_void,
        __unused8: *mut ::c_void,
        __unused9: *mut ::c_void,
    }
}
