pub const PTHREAD_STACK_MIN: ::size_t = 2048;
pub const KERN_PROC_PATHNAME: ::c_int = 12;
pub const SIGSTKSZ: ::size_t = 2048 /* MINSIGSTKSZ */ + 32768;

extern {
    pub fn __error() -> *mut ::c_int;
}
