pub const PTHREAD_STACK_MIN: ::size_t = 1024;
pub const KERN_PROC_PATHNAME: ::c_int = 9;

extern {
    pub fn __dfly_error() -> *const ::c_int;
}
