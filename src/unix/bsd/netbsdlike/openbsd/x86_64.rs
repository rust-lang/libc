use PT_FIRSTMACH;

pub type c_long = i64;
pub type c_ulong = u64;
pub type c_char = i8;

s! {
    // Defined in <machine/signal.h>.
    pub struct sigcontext_t {
        pub sc_rdi: ::c_long,
        pub sc_rsi: ::c_long,
        pub sc_rdx: ::c_long,
        pub sc_rcx: ::c_long,
        pub sc_r8: ::c_long,
        pub sc_r9: ::c_long,
        pub sc_r10: ::c_long,
        pub sc_r11: ::c_long,
        pub sc_r12: ::c_long,
        pub sc_r13: ::c_long,
        pub sc_r14: ::c_long,
        pub sc_r15: ::c_long,
        pub sc_rbp: ::c_long,
        pub sc_rbx: ::c_long,
        pub sc_rax: ::c_long,
        pub sc_gs: ::c_long,
        pub sc_fs: ::c_long,
        pub sc_es: ::c_long,
        pub sc_ds: ::c_long,
        pub sc_trapno: ::c_long,
        pub sc_err: ::c_long,
        pub sc_rip: ::c_long,
        pub sc_cs: ::c_long,
        pub sc_rflags: ::c_long,
        pub sc_rsp: ::c_long,
        pub sc_ss: ::c_long,
        pub sc_fpstate: *mut ::c_char,
        pub __sc_unused: ::c_int,
        pub sc_mask: ::c_int,
        pub sc_cookie: ::c_long,
    }
}

pub type ucontext_t = sigcontext_t;

// should be pub(crate), but that requires Rust 1.18.0
cfg_if! {
    if #[cfg(libc_const_size_of)] {
        #[doc(hidden)]
        pub const _ALIGNBYTES: usize = ::mem::size_of::<::c_long>() - 1;
    } else {
        #[doc(hidden)]
        pub const _ALIGNBYTES: usize = 8 - 1;
    }
}

pub const _MAX_PAGE_SHIFT: u32 = 12;

pub const PT_STEP: ::c_int = PT_FIRSTMACH + 0;
pub const PT_GETREGS: ::c_int = PT_FIRSTMACH + 1;
pub const PT_SETREGS: ::c_int = PT_FIRSTMACH + 2;
pub const PT_GETFPREGS: ::c_int = PT_FIRSTMACH + 3;
pub const PT_SETFPREGS: ::c_int = PT_FIRSTMACH + 4;
