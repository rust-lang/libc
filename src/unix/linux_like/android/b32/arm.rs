pub type c_char = u8;
pub type wchar_t = u32;
pub type greg_t = i32;
pub type mcontext_t = sigcontext;

s! {
    pub struct sigcontext {
        pub trap_no: ::c_ulong,
        pub error_code: ::c_ulong,
        pub oldmask: ::c_ulong,
        pub arm_r0: ::c_ulong,
        pub arm_r1: ::c_ulong,
        pub arm_r2: ::c_ulong,
        pub arm_r3: ::c_ulong,
        pub arm_r4: ::c_ulong,
        pub arm_r5: ::c_ulong,
        pub arm_r6: ::c_ulong,
        pub arm_r7: ::c_ulong,
        pub arm_r8: ::c_ulong,
        pub arm_r9: ::c_ulong,
        pub arm_r10: ::c_ulong,
        pub arm_fp: ::c_ulong,
        pub arm_ip: ::c_ulong,
        pub arm_sp: ::c_ulong,
        pub arm_lr: ::c_ulong,
        pub arm_pc: ::c_ulong,
        pub arm_cpsr: ::c_ulong,
        pub fault_address: ::c_ulong,
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
                /* The kernel adds extra padding after uc_sigmask to match
                 * glibc sigset_t on ARM. */
                __padding: [c_char; 120],
                __align: [::c_longlong; 0],
                uc_regspace: [::c_ulong; 128],
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
                            && &self.uc_regspace[..] == &other.uc_regspace[..]
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
                            .field("uc_regspace", &&self.uc_regspace[..])
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
                        &self.uc_regspace[..].hash(state);
                        // Ignore padding field
                    }
                }
            }
        }
    }
}

// offsets in mcontext_t.gregs from sys/ucontext.h
pub const REG_R0: ::c_int = 0;
pub const REG_R1: ::c_int = 1;
pub const REG_R2: ::c_int = 2;
pub const REG_R3: ::c_int = 3;
pub const REG_R4: ::c_int = 4;
pub const REG_R5: ::c_int = 5;
pub const REG_R6: ::c_int = 6;
pub const REG_R7: ::c_int = 7;
pub const REG_R8: ::c_int = 8;
pub const REG_R9: ::c_int = 9;
pub const REG_R10: ::c_int = 10;
pub const REG_R11: ::c_int = 11;
pub const REG_R12: ::c_int = 12;
pub const REG_R13: ::c_int = 13;
pub const REG_R14: ::c_int = 14;
pub const REG_R15: ::c_int = 15;

pub const NGREG: ::c_int = 18;
