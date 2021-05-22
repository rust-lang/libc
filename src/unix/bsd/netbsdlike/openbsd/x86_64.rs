use PT_FIRSTMACH;

pub type c_long = i64;
pub type c_ulong = u64;
pub type c_char = i8;
pub type ucontext_t = sigcontext;

s_no_extra_traits! {
    #[repr(C, packed)]
    pub struct fxsave64 {
        pub fx_fcw: u16,
        pub fx_fsw: u16,
        pub fx_ftw: u8,
        __unused: u8,
        pub fx_fop: u16,
        pub fx_rip: u64,
        pub fx_rdp: u64,
        pub fx_mxcsr: u32,
        pub fx_mxcsr_mask: u32,
        pub fx_st: [[u64; 2]; 8],
        pub fx_xmm: [[u64; 2]; 16],
        __reserved: [u8; 96],
    }
}

cfg_if! {
    if #[cfg(all(feature = "extra_traits", libc_addr_of, std))] {
            impl PartialEq for fxsave64 {
                fn eq(&self, other: &fxsave64) -> bool {
                    unsafe {
                    let fx_xmm = self.fx_xmm.as_ptr();
                    let ofx_xmm = other.fx_xmm.as_ptr();
                    self.fx_fcw == other.fx_fcw &&
                        self.fx_fsw == other.fx_fsw &&
                        self.fx_ftw == other.fx_ftw &&
                        self.fx_fop == other.fx_fop &&
                        self.fx_rip == other.fx_rip &&
                        self.fx_rdp == other.fx_rdp &&
                        self.fx_mxcsr == other.fx_mxcsr &&
                        self.fx_mxcsr_mask == other.fx_mxcsr_mask &&
                        self.fx_st == other.fx_st &&
                        std::ptr::addr_of!(fx_xmm) == std::ptr::addr_of!(ofx_xmm)
                    }
                }
            }
            impl Eq for fxsave64 {}
            impl ::fmt::Debug for fxsave64 {
                fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                    unsafe {
                    let fx_xmm = self.fx_xmm.as_ptr();
                    f.debug_struct("fxsave64")
                        .field("fx_fcw", &self.fx_fcw)
                        .field("fx_fsw", &self.fx_fsw)
                        .field("fx_ftw", &self.fx_ftw)
                        .field("fx_fop", &self.fx_fop)
                        .field("fx_rip", &self.fx_rip)
                        .field("fx_rdp", &self.fx_rdp)
                        .field("fx_mxcsr", &self.fx_mxcsr)
                        .field("fx_mxcsr_mask", &self.fx_mxcsr_mask)
                        .field("fx_st", &self.fx_st)
                        .field("fx_xmm", &*std::ptr::addr_of!(fx_xmm))
                        .finish()
                    }
                }
            }
        }

}

cfg_if! {
        if #[cfg(all(feature = "extra_traits", not(std)))] {
            impl PartialEq for fxsave64 {
                fn eq(&self, other: &fxsave64) -> bool {
                    unsafe {
                    let fx_xmm = self.fx_xmm.as_ptr();
                    let ofx_xmm = other.fx_xmm.as_ptr();
                    self.fx_fcw == other.fx_fcw &&
                        self.fx_fsw == other.fx_fsw &&
                        self.fx_ftw == other.fx_ftw &&
                        self.fx_fop == other.fx_fop &&
                        self.fx_rip == other.fx_rip &&
                        self.fx_rdp == other.fx_rdp &&
                        self.fx_mxcsr == other.fx_mxcsr &&
                        self.fx_mxcsr_mask == other.fx_mxcsr_mask &&
                        self.fx_st == other.fx_st &&
                        fx_xmm == ofx_xmm
                    }
                }
            }
            impl Eq for fxsave64 {}
            impl ::fmt::Debug for fxsave64 {
                fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                    unsafe {
                    f.debug_struct("fxsave64")
                        .field("fx_fcw", &self.fx_fcw)
                        .field("fx_fsw", &self.fx_fsw)
                        .field("fx_ftw", &self.fx_ftw)
                        .field("fx_fop", &self.fx_fop)
                        .field("fx_rip", &self.fx_rip)
                        .field("fx_rdp", &self.fx_rdp)
                        .field("fx_mxcsr", &self.fx_mxcsr)
                        .field("fx_mxcsr_mask", &self.fx_mxcsr_mask)
                        .field("fx_st", &self.fx_st)
                        .field("fx_xmm", &self.fx_xmm)
                        .finish()
                    }
                }
            }
        }
}

s! {
    pub struct sigcontext {
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
        sc_fpstate: *mut fxsave64,
        __unused: ::c_int,
        pub sc_mask: ::c_int,
        pub sc_cookie: ::c_long,
    }
}

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
