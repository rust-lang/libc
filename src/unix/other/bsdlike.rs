extern {
    pub fn shm_open(name: *const ::c_char, oflag: ::c_int, ...) -> ::c_int;
    pub fn sysctl(name: *mut ::c_int,
                  namelen: ::c_uint,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *mut ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
    pub fn mincore(addr: *const ::c_void, len: ::size_t,
                   vec: *mut ::c_char) -> ::c_int;
    pub fn sysctlbyname(name: *const ::c_char,
                        oldp: *mut ::c_void,
                        oldlenp: *mut ::size_t,
                        newp: *mut ::c_void,
                        newlen: ::size_t)
                        -> ::c_int;
    pub fn sysctlnametomib(name: *const ::c_char,
                           mibp: *mut ::c_int,
                           sizep: *mut ::size_t)
                           -> ::c_int;
}
