pub type c_char = i8;
pub type wchar_t = i32;
pub type greg_t = i32;

s! {
    pub struct _libc_fpreg {
        pub significand: [u16; 4],
        pub exponent: u16,
    }

    pub struct _libc_fpstate {
        pub cw: ::c_ulong,
        pub sw: ::c_ulong,
        pub tag: ::c_ulong,
        pub ipoff: ::c_ulong,
        pub cssel: ::c_ulong,
        pub dataoff: ::c_ulong,
        pub datasel: ::c_ulong,
        pub _st: [_libc_fpreg; 8],
        pub status: ::c_ulong,
    }

    pub struct mcontext_t {
        pub gregs: [greg_t; 19],
        pub fpregs: *mut _libc_fpstate,
        pub oldmask: ::c_ulong,
        pub cr2: ::c_ulong,
    }
}

cfg_if! {
    if #[cfg(libc_union)] {
        s_no_extra_traits! {
            pub struct __c_anonymous_uc_sigmask_with_padding {
                pub uc_sigmask: ::sigset_t,
                /* Android has a wrong (smaller) sigset_t on x86. */
                __padding_rt_sigset: u32,
            }

            pub union __c_anonymous_uc_sigmask {
                uc_sigmask: __c_anonymous_uc_sigmask_with_padding,
                uc_sigmask64: ::sigset64_t,
            }

            pub struct ucontext_t {
                pub uc_flags: ::c_ulong,
                pub uc_link: *mut ucontext_t,
                pub uc_stack: ::stack_t,
                pub uc_mcontext: mcontext_t,
                pub uc_sigmask__c_anonymous_union: __c_anonymous_uc_sigmask,
                __padding_rt_sigset: u32,
                __fpregs_mem: _libc_fpstate,
            }
        }

        cfg_if! {
            if #[cfg(feature = "extra_traits")] {
                impl PartialEq for __c_anonymous_uc_sigmask_with_padding {
                    fn eq(
                        &self, other: &__c_anonymous_uc_sigmask_with_padding
                    ) -> bool {
                        self.uc_sigmask == other.uc_sigmask
                            // Ignore padding
                    }
                }
                impl Eq for __c_anonymous_uc_sigmask_with_padding {}
                impl ::fmt::Debug for __c_anonymous_uc_sigmask_with_padding {
                    fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                        f.debug_struct("uc_sigmask_with_padding")
                            .field("uc_sigmask_with_padding", &self.uc_sigmask)
                            // Ignore padding
                            .finish()
                    }
                }
                impl ::hash::Hash for __c_anonymous_uc_sigmask_with_padding {
                    fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                        self.uc_sigmask.hash(state)
                            // Ignore padding
                    }
                }

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

                impl PartialEq for ucontext_t {
                    fn eq(&self, other: &Self) -> bool {
                        self.uc_flags == other.uc_flags
                            && self.uc_link == other.uc_link
                            && self.uc_stack == other.uc_stack
                            && self.uc_mcontext == other.uc_mcontext
                            && self.uc_sigmask__c_anonymous_union
                                == other.uc_sigmask__c_anonymous_union
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
                            .field(
                                "uc_sigmask__c_anonymous_union",
                                &self.uc_sigmask__c_anonymous_union
                            )
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
                        self.uc_sigmask__c_anonymous_union.hash(state);
                        // Ignore padding field
                    }
                }
            }
        }
    }
}

pub const O_DIRECT: ::c_int = 0x4000;
pub const O_DIRECTORY: ::c_int = 0x10000;
pub const O_NOFOLLOW: ::c_int = 0x20000;
pub const O_LARGEFILE: ::c_int = 0o0100000;

pub const MAP_32BIT: ::c_int = 0x40;

// offsets in user_regs_structs, from sys/reg.h
pub const EBX: ::c_int = 0;
pub const ECX: ::c_int = 1;
pub const EDX: ::c_int = 2;
pub const ESI: ::c_int = 3;
pub const EDI: ::c_int = 4;
pub const EBP: ::c_int = 5;
pub const EAX: ::c_int = 6;
pub const DS: ::c_int = 7;
pub const ES: ::c_int = 8;
pub const FS: ::c_int = 9;
pub const GS: ::c_int = 10;
pub const ORIG_EAX: ::c_int = 11;
pub const EIP: ::c_int = 12;
pub const CS: ::c_int = 13;
pub const EFL: ::c_int = 14;
pub const UESP: ::c_int = 15;
pub const SS: ::c_int = 16;

// offsets in mcontext_t.gregs from sys/ucontext.h
pub const REG_GS: ::c_int = 0;
pub const REG_FS: ::c_int = 1;
pub const REG_ES: ::c_int = 2;
pub const REG_DS: ::c_int = 3;
pub const REG_EDI: ::c_int = 4;
pub const REG_ESI: ::c_int = 5;
pub const REG_EBP: ::c_int = 6;
pub const REG_ESP: ::c_int = 7;
pub const REG_EBX: ::c_int = 8;
pub const REG_EDX: ::c_int = 9;
pub const REG_ECX: ::c_int = 10;
pub const REG_EAX: ::c_int = 11;
pub const REG_TRAPNO: ::c_int = 12;
pub const REG_ERR: ::c_int = 13;
pub const REG_EIP: ::c_int = 14;
pub const REG_CS: ::c_int = 15;
pub const REG_EFL: ::c_int = 16;
pub const REG_UESP: ::c_int = 17;
pub const REG_SS: ::c_int = 18;

cfg_if! {
    if #[cfg(libc_align)] {
        mod align;
        pub use self::align::*;
    }
}
