pub type clock_t = i32;
pub type ino_t = u32;
pub type nlink_t = u16;
pub type blksize_t = u32;

pub const PTHREAD_STACK_MIN: ::size_t = 2048;
pub const KERN_PROC_PATHNAME: ::c_int = 12;
pub const SIGSTKSZ: ::size_t = 34816;

s! {
    pub struct dirent {
        pub d_fileno: u32,
        pub d_reclen: u16,
        pub d_type: u8,
        pub d_namelen: u8,
        pub d_name: [::c_char; 256],
    }
}

extern {
    pub fn __error() -> *mut ::c_int;
}
