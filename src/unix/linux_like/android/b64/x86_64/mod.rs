pub type c_char = i8;
pub type wchar_t = i32;
pub type greg_t = i64;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_nlink: ::c_ulong,
        pub st_mode: ::c_uint,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off64_t,
        pub st_blksize: ::c_long,
        pub st_blocks: ::c_long,
        pub st_atime: ::c_long,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::c_long,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::c_long,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_long; 3],
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_nlink: ::c_ulong,
        pub st_mode: ::c_uint,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off64_t,
        pub st_blksize: ::c_long,
        pub st_blocks: ::c_long,
        pub st_atime: ::c_long,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::c_long,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::c_long,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_long; 3],
    }

    pub struct _libc_xmmreg {
        pub element: [u32; 4],
    }
}

cfg_if! {
    if #[cfg(libc_union)] {
        s_no_extra_traits! {
            pub union __c_anonymous_uc_sigmask {
                uc_sigmask: ::sigset_t,
                uc_sigmask64: ::sigset64_t,
            }
        }

        cfg_if! {
            if #[cfg(feature = "extra_traits")] {
                impl PartialEq for __c_anonymous_uc_sigmask {
                    fn eq(&self, other: &__c_anonymous_uc_sigmask) -> bool {
                        unsafe { self.uc_sigmask == other.uc_sigmask }
                    }
                }
                impl Eq for __c_anonymous_uc_sigmask {}
                impl ::fmt::Debug for __c_anonymous_uc_sigmask {
                    fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                        f.debug_struct("uc_sigmask")
                            .field("uc_sigmask", unsafe { &self.uc_sigmask })
                            .finish()
                    }
                }
                impl ::hash::Hash for __c_anonymous_uc_sigmask {
                    fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                        unsafe { self.uc_sigmask.hash(state) }
                    }
                }
            }
        }
    }
}

s_no_extra_traits! {
    pub struct _libc_fpxreg {
        pub significand: [u16; 4],
        pub exponent: u16,
        __padding: [u16; 3],
    }

    pub struct _libc_fpstate {
        pub cwd: u16,
        pub swd: u16,
        pub ftw: u16,
        pub fop: u16,
        pub rip: u64,
        pub rdp: u64,
        pub mxcsr: u32,
        pub mxcr_mask: u32,
        pub _st: [_libc_fpxreg; 8],
        pub _xmm: [_libc_xmmreg; 16],
        __private: [u32; 24],
    }

    pub struct mcontext_t {
        pub gregs: [greg_t; 23],
        pub fpregs: *mut _libc_fpstate,
        __private: [u64; 8],
    }

    pub struct ucontext_t {
        pub uc_flags: ::c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_mcontext: mcontext_t,
        pub uc_sigmask64: __c_anonymous_uc_sigmask,
        __fpregs_mem: _libc_fpstate,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for _libc_fpxreg {
            fn eq(&self, other: &Self) -> bool {
                self.significand == other.significand
                    && self.exponent == other.exponent
                    // Ignore padding field
            }
        }
        impl Eq for _libc_fpxreg {}
        impl ::fmt::Debug for _libc_fpxreg {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("_libc_fpxreg")
                    .field("significand", &self.significand)
                    .field("exponent", &self.exponent)
                    // Ignore padding field
                    .finish()
            }
        }
        impl ::hash::Hash for _libc_fpxreg {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.significand.hash(state);
                self.exponent.hash(state);
                // Ignore padding field
            }
        }

        impl PartialEq for _libc_fpstate {
            fn eq(&self, other: &Self) -> bool {
                self.cwd == other.cwd
                    && self.swd == other.swd
                    && self.ftw == other.ftw
                    && self.fop == other.fop
                    && self.rip == other.rip
                    && self.rdp == other.rdp
                    && self.mxcsr == other.mxcsr
                    && self.mxcr_mask == other.mxcr_mask
                    && self._st == other._st
                    && self._xmm == other._xmm
                    // Ignore padding field
            }
        }
        impl Eq for _libc_fpstate {}
        impl ::fmt::Debug for _libc_fpstate {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("_libc_fpstate")
                    .field("cwd", &self.cwd)
                    .field("swd", &self.swd)
                    .field("ftw", &self.ftw)
                    .field("fop", &self.fop)
                    .field("rip", &self.rip)
                    .field("rdp", &self.rdp)
                    .field("mxcsr", &self.mxcsr)
                    .field("mxcr_mask", &self.mxcr_mask)
                    .field("_st", &self._st)
                    .field("_xmm", &self._xmm)
                    // Ignore padding field
                    .finish()
            }
        }
        impl ::hash::Hash for _libc_fpstate {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.cwd.hash(state);
                self.swd.hash(state);
                self.ftw.hash(state);
                self.fop.hash(state);
                self.rip.hash(state);
                self.rdp.hash(state);
                self.mxcsr.hash(state);
                self.mxcr_mask.hash(state);
                self._st.hash(state);
                self._xmm.hash(state);
                // Ignore padding field
            }
        }

        impl PartialEq for mcontext_t {
            fn eq(&self, other: &Self) -> bool {
                self.gregs == other.gregs
                    && self.fpregs == other.fpregs
                    // Ignore padding field
            }
        }
        impl Eq for mcontext_t {}
        impl ::fmt::Debug for mcontext_t {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("mcontext_t")
                    .field("gregs", &self.gregs)
                    .field("fpregs", &self.fpregs)
                    // Ignore padding field
                    .finish()
            }
        }
        impl ::hash::Hash for mcontext_t {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.gregs.hash(state);
                self.fpregs.hash(state);
                // Ignore padding field
            }
        }

        impl PartialEq for ucontext_t {
            fn eq(&self, other: &Self) -> bool {
                self.uc_flags == other.uc_flags
                    && self.uc_link == other.uc_link
                    && self.uc_stack == other.uc_stack
                    && self.uc_mcontext == other.uc_mcontext
                    && self.uc_sigmask64 == other.uc_sigmask64
                    // Ignore padding field
            }
        }
        impl Eq for ucontext_t {}
        impl ::fmt::Debug for ucontext_t {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("ucontext_t")
                    .field("uc_flags", &self.uc_flags)
                    .field("uc_link", &self.uc_link)
                    .field("uc_stack", &self.uc_stack)
                    .field("uc_mcontext", &self.uc_mcontext)
                    .field("uc_sigmask64", &self.uc_sigmask64)
                    // Ignore padding field
                    .finish()
            }
        }
        impl ::hash::Hash for ucontext_t {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.uc_flags.hash(state);
                self.uc_link.hash(state);
                self.uc_stack.hash(state);
                self.uc_mcontext.hash(state);
                self.uc_sigmask64.hash(state);
                // Ignore padding field
            }
        }
    }
}

pub const O_DIRECT: ::c_int = 0x4000;
pub const O_DIRECTORY: ::c_int = 0x10000;
pub const O_NOFOLLOW: ::c_int = 0x20000;
pub const O_LARGEFILE: ::c_int = 0o0100000;

pub const SIGSTKSZ: ::size_t = 8192;
pub const MINSIGSTKSZ: ::size_t = 2048;

pub const MAP_32BIT: ::c_int = 0x40;

// offsets in user_regs_structs, from sys/reg.h
pub const R15: ::c_int = 0;
pub const R14: ::c_int = 1;
pub const R13: ::c_int = 2;
pub const R12: ::c_int = 3;
pub const RBP: ::c_int = 4;
pub const RBX: ::c_int = 5;
pub const R11: ::c_int = 6;
pub const R10: ::c_int = 7;
pub const R9: ::c_int = 8;
pub const R8: ::c_int = 9;
pub const RAX: ::c_int = 10;
pub const RCX: ::c_int = 11;
pub const RDX: ::c_int = 12;
pub const RSI: ::c_int = 13;
pub const RDI: ::c_int = 14;
pub const ORIG_RAX: ::c_int = 15;
pub const RIP: ::c_int = 16;
pub const CS: ::c_int = 17;
pub const EFLAGS: ::c_int = 18;
pub const RSP: ::c_int = 19;
pub const SS: ::c_int = 20;
pub const FS_BASE: ::c_int = 21;
pub const GS_BASE: ::c_int = 22;
pub const DS: ::c_int = 23;
pub const ES: ::c_int = 24;
pub const FS: ::c_int = 25;
pub const GS: ::c_int = 26;

// offsets in mcontext_t.gregs from sys/ucontext.h
pub const REG_R8: ::c_int = 0;
pub const REG_R9: ::c_int = 1;
pub const REG_R10: ::c_int = 2;
pub const REG_R11: ::c_int = 3;
pub const REG_R12: ::c_int = 4;
pub const REG_R13: ::c_int = 5;
pub const REG_R14: ::c_int = 6;
pub const REG_R15: ::c_int = 7;
pub const REG_RDI: ::c_int = 8;
pub const REG_RSI: ::c_int = 9;
pub const REG_RBP: ::c_int = 10;
pub const REG_RBX: ::c_int = 11;
pub const REG_RDX: ::c_int = 12;
pub const REG_RAX: ::c_int = 13;
pub const REG_RCX: ::c_int = 14;
pub const REG_RSP: ::c_int = 15;
pub const REG_RIP: ::c_int = 16;
pub const REG_EFL: ::c_int = 17;
pub const REG_CSGSFS: ::c_int = 18;
pub const REG_ERR: ::c_int = 19;
pub const REG_TRAPNO: ::c_int = 20;
pub const REG_OLDMASK: ::c_int = 21;
pub const REG_CR2: ::c_int = 22;

cfg_if! {
    if #[cfg(libc_align)] {
        mod align;
        pub use self::align::*;
    }
}
