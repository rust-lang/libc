pub const SYS_uselib: ::c_long = 134;
pub const SYS__sysctl: ::c_long = 156;
pub const SYS_create_module: ::c_long = 174;
pub const SYS_get_kernel_syms: ::c_long = 177;
pub const SYS_query_module: ::c_long = 178;
pub const SYS_nfsservctl: ::c_long = 180;
pub const SYS_set_thread_area: ::c_long = 205;
pub const SYS_get_thread_area: ::c_long = 211;
pub const SYS_epoll_ctl_old: ::c_long = 214;
pub const SYS_epoll_wait_old: ::c_long = 215;
pub const SYS_vserver: ::c_long = 236;

#[link(name = "util")]
extern {
    pub fn sysctl(name: *mut ::c_int,
                  namelen: ::c_int,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *mut ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
}
