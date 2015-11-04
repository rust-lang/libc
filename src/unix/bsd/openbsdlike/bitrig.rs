s! {
    pub struct glob_t {
        pub gl_pathc:  ::c_int,
        pub gl_matchc: ::c_int,
        pub gl_offs:   ::c_int,
        pub gl_flags:  ::c_int,
        pub gl_pathv:  *mut *mut ::c_char,
        __unused1: *mut ::c_void,
        __unused2: *mut ::c_void,
        __unused3: *mut ::c_void,
        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
        __unused6: *mut ::c_void,
        __unused7: *mut ::c_void,
    }
}
