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
}
