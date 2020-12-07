s! {
    pub struct __darwin_mcontext64 {
        pub __es: ::__darwin_x86_exception_state64,
        pub __ss: __darwin_x86_thread_state64,
        pub __fs: ::__darwin_x86_float_state64,
    }

    pub struct __darwin_x86_thread_state64 {
        pub __rax: u64,
        pub __rbx: u64,
        pub __rcx: u64,
        pub __rdx: u64,
        pub __rdi: u64,
        pub __rsi: u64,
        pub __rbp: u64,
        pub __rsp: u64,
        pub __r8: u64,
        pub __r9: u64,
        pub __r10: u64,
        pub __r11: u64,
        pub __r12: u64,
        pub __r13: u64,
        pub __r14: u64,
        pub __r15: u64,
        pub __rip: u64,
        pub __rflags: u64,
        pub __cs: u64,
        pub __fs: u64,
        pub __gs: u64,
    }
}
