    {#- Requires the presence of an `ident` variable. +#}

    /// Test that the size and alignment of the aliased type is the same in both Rust and C.
    /// 
    /// This can fail if a different type is used on one side, and uses the built in size and
    /// alignment functions to check.
    pub fn size_align_{{ ident }}() {
        extern "C" {
            fn ctest_size_of__{{ ident }}() -> u64;
            fn ctest_align_of__{{ ident }}() -> u64;
        }
        unsafe {
            check_same(size_of::<{{ ident }}>() as u64,
                    ctest_size_of__{{ ident }}(), "{{ ident }} size");
            check_same(align_of::<{{ ident }}>() as u64,
                    ctest_align_of__{{ ident }}(), "{{ ident }} align");
        }
    }