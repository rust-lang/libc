//! 32-bit specific Apple (ios/darwin) definitions

pub type c_long = i32;
pub type c_ulong = u32;
pub type size_t = u32;
pub type ptrdiff_t = i32;
pub type intptr_t = i32;
pub type uintptr_t = u32;

s! {
    pub struct pthread_attr_t {
        __sig: c_long,
        __opaque: [::c_char; 36]
    }
}
