s_no_extra_traits! {
    #[allow(missing_debug_implementations)]
    #[repr(align(4))]
    pub struct max_align_t {
        priv_: [i8; 4]
    }
}
