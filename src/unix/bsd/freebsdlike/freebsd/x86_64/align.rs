use {c_long, register_t};

s! {
    #[repr(align(16))]
    pub struct max_align_t {
        priv_: [f64; 4]
    }

    #[repr(align(16))]
    pub struct mcontext_t {
        pub mc_onstack: register_t,
        pub mc_rdi: register_t,
        pub mc_rsi: register_t,
        pub mc_rdx: register_t,
        pub mc_rcx: register_t,
        pub mc_r8: register_t,
        pub mc_r9: register_t,
        pub mc_rax: register_t,
        pub mc_rbx: register_t,
        pub mc_rbp: register_t,
        pub mc_r10: register_t,
        pub mc_r11: register_t,
        pub mc_r12: register_t,
        pub mc_r13: register_t,
        pub mc_r14: register_t,
        pub mc_r15: register_t,
        pub mc_trapno: u32,
        pub mc_fs: u16,
        pub mc_gs: u16,
        pub mc_addr: register_t,
        pub mc_flags: u32,
        pub mc_es: u16,
        pub mc_ds: u16,
        pub mc_err: register_t,
        pub mc_rip: register_t,
        pub mc_cs: register_t,
        pub mc_rflags: register_t,
        pub mc_rsp: register_t,
        pub mc_ss: register_t,
        pub mc_len: c_long,
        pub mc_fpformat: c_long,
        pub mc_ownedfp: c_long,
        pub mc_fpstate: [c_long; 64],
        pub mc_fsbase: register_t,
        pub mc_gsbase: register_t,
        pub mc_xfpustate: register_t,
        pub mc_xfpustate_len: register_t,
        pub mc_spare: [c_long; 4],
    }

    pub struct ucontext_t {
        pub uc_sigmask: ::sigset_t,
        pub uc_mcontext: ::mcontext_t,
        pub uc_link: *mut ::ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_flags: ::c_int,
        __spare__: [::c_int; 4],
    }
}
