pub type clock_t = i32;
pub type ino_t = u32;
pub type nlink_t = u16;
pub type blksize_t = u32;

pub const PTHREAD_STACK_MIN: ::size_t = 2048;
pub const KERN_PROC_PATHNAME: ::c_int = 12;
pub const SIGSTKSZ: ::size_t = 34816;
pub const HW_AVAILCPU: ::c_int = 25;

s! {
    pub struct dirent {
        pub d_fileno: u32,
        pub d_reclen: u16,
        pub d_type: u8,
        pub d_namelen: u8,
        pub d_name: [::c_char; 256],
    }

    pub struct utsname {
        pub sysname: [::c_char; 256],
        pub nodename: [::c_char; 256],
        pub release: [::c_char; 256],
        pub version: [::c_char; 256],
        pub machine: [::c_char; 256],
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_size: ::size_t,
        pub ss_flags: ::c_int,
    }
}

extern {
    pub fn mprotect(addr: *const ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn clock_gettime(clk_id: ::c_int, tp: *mut ::timespec) -> ::c_int;
    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t,
                           len: ::off_t) -> ::c_int;
    pub fn __error() -> *mut ::c_int;
}
