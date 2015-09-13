extern {
    pub fn shm_open(name: *const ::c_char, oflag: ::c_int,
                    mode: ::mode_t) -> ::c_int;
    #[cfg(not(target_env = "musl"))]
    pub fn sysctl(name: *mut ::c_int,
                  namelen: ::c_int,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *mut ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
    pub fn mincore(addr: *mut ::c_void, len: ::size_t,
                   vec: *mut ::c_uchar) -> ::c_int;
}
