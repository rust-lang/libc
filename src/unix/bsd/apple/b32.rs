//! 32-bit specific Apple (ios/darwin) definitions

pub type c_long = i32;
pub type c_ulong = u32;

pub const __PTHREAD_MUTEX_SIZE__: usize = 40;
pub const __PTHREAD_COND_SIZE__: usize = 24;
pub const __PTHREAD_RWLOCK_SIZE__: usize = 124;

s! {
    pub struct pthread_attr_t {
        __sig: c_long,
        __opaque: [::c_char; 36]
    }

    pub struct mcontext_t {
        __gregs: [usize; 5],
        __fpregs: *mut u8,
        __reserved: [::c_ulonglong; 8],
    }

    pub struct ucontext_t {
        pub uc_flags: ::c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_mcontext: mcontext_t,
        pub uc_sigmask: ::sigset_t,
        __rest: [usize; 28],
    }
}
