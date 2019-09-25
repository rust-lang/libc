s_no_extra_traits! {
    #[repr(align(8))]
    pub struct max_align_t {
        priv_: [i64; 2]
    }
}
