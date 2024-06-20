s_no_extra_traits! {
    #[allow(missing_debug_implementations)]
    #[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeroes, zerocopy::FromBytes, zerocopy::AsBytes))]
    #[repr(align(2))]
    pub struct max_align_t {
        priv_: [i8; 20]
    }
}
