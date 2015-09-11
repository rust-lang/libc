pub type c_long = i64;
pub type c_ulong = u64;
pub type size_t = u64;
pub type ptrdiff_t = i64;
pub type intptr_t = i64;
pub type uintptr_t = u64;

s! {
    pub struct pthread_attr_t {
        __sig: ::c_long,
        __opaque: [::c_char; 56]
    }
}
