s_no_extra_traits! {
    #[allow(missing_debug_implementations)]
    #[repr(align(8))]
    pub struct max_align_t {
        priv_: [i64; 2]
    }

    #[allow(missing_debug_implementations)]
    #[repr(align(8))]
    pub struct ucontext_t {
        pub uc_flags: ::c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_mcontext: ::mcontext_t,
        pub uc_sigmask: ::sigset_t,
        pub uc_regspace: [::c_ulong; 128],
    }
}
