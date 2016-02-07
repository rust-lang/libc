pub type fsblkcnt_t = ::uint64_t;
pub type fsfilcnt_t = ::uint64_t;

pub const PTHREAD_STACK_MIN: ::size_t = 2048;
pub const KERN_PROC_PATHNAME: ::c_int = 12;
pub const SIGSTKSZ: ::size_t = 34816;
pub const SF_NODISKIO: ::c_int = 0x00000001;
pub const SF_MNOWAIT: ::c_int = 0x00000002;
pub const SF_SYNC: ::c_int = 0x00000004;

extern {
    pub fn __error() -> *mut ::c_int;
}
