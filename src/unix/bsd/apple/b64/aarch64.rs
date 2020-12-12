s! {
    pub struct __darwin_mcontext64 {
        pub __es: ::__darwin_x86_exception_state64,
        pub __ss: __darwin_arm_thread_state64,
        pub __fs: ::__darwin_x86_float_state64,
    }

    pub struct __darwin_arm_thread_state64 {
        pub __x: [u64; 29],
        pub __fp: u64,
        pub __lr: u64,
        pub __sp: u64,
        pub __pc: u64,
        pub __cpsr: u32,
        pub __pad: u32,
    }
}
