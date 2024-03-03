s! {
    // linux/openat2.h
    #[non_exhaustive]
    #[cfg_attr(feature = "zerocopy", derive(zerocopy::FromZeroes, zerocopy::FromBytes, zerocopy::AsBytes))]
    pub struct open_how {
        pub flags: ::__u64,
        pub mode: ::__u64,
        pub resolve: ::__u64,
    }
}
