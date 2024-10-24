s_no_extra_traits! {
    #[allow(missing_debug_implementations)]
    #[repr(align(16))]
    pub struct max_align_t {
        priv_: [i64; 4]
    }

    #[allow(missing_debug_implementations)]
    pub struct ucontext_t {
        pub uc_flags: ::c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_sigmask: ::sigset_t,
        pub uc_mcontext: mcontext_t,
    }

    #[allow(missing_debug_implementations)]
    pub struct pt_regs {
        pub gpr: [::c_ulong; 32],
        pub nip: ::c_ulong,
        pub msr: ::c_ulong,
        pub orig_gpr3: ::c_ulong,
        pub ctr: ::c_ulong,
        pub link: ::c_ulong,
        pub xer: ::c_ulong,
        pub ccr: ::c_ulong,
        pub softe: ::c_ulong,
        pub trap: ::c_ulong,
        pub dar: ::c_ulong,
        pub dsisr: ::c_ulong,
        pub result: ::c_ulong,
    }

    #[allow(missing_debug_implementations)]
    #[repr(align(8))]
    pub struct mcontext_t {
        __glibc_reserved: [::c_ulong; 4],
        pub signal: ::c_int,
        __pad0: ::c_int,
        pub handler: ::c_ulong,
        pub oldmask: ::c_ulong,
        pub regs: *mut pt_regs,
        pub gp_regs: [::c_ulong; 48], // # define __NGREG	48
        pub fp_regs: [::c_double; 33], // # define __NFPREG	33
        pub v_regs: *mut vrregset_t,
        pub vmx_reserve: [::c_long; 69], // # define __NVRREG	34 (34*2+1)
    }

    #[allow(missing_debug_implementations)]
    #[repr(align(16))]
    pub struct vrregset_t {
        pub vrregs: [[::c_uint; 4]; 32],
        pub vscr: vscr_t,
        pub vrsave: ::c_uint,
        __pad: [::c_uint; 3],
    }

    #[allow(missing_debug_implementations)]
    #[repr(align(4))]
    pub struct vscr_t {
        #[cfg(target_endian = "big")]
        __pad: [::c_uint; 3],
        #[cfg(target_endian = "big")]
        pub vscr_word: ::c_uint,

        #[cfg(target_endian = "little")]
        pub vscr_word: ::c_uint,
        #[cfg(target_endian = "little")]
        __pad: [::c_uint; 3],
    }
}

s! {
    #[repr(align(8))]
    pub struct clone_args {
        pub flags: ::c_ulonglong,
        pub pidfd: ::c_ulonglong,
        pub child_tid: ::c_ulonglong,
        pub parent_tid: ::c_ulonglong,
        pub exit_signal: ::c_ulonglong,
        pub stack: ::c_ulonglong,
        pub stack_size: ::c_ulonglong,
        pub tls: ::c_ulonglong,
        pub set_tid: ::c_ulonglong,
        pub set_tid_size: ::c_ulonglong,
        pub cgroup: ::c_ulonglong,
    }
}

extern "C" {
    pub fn getcontext(ucp: *mut ucontext_t) -> ::c_int;
    pub fn setcontext(ucp: *const ucontext_t) -> ::c_int;
    pub fn makecontext(ucp: *mut ucontext_t, func: extern "C" fn(), argc: ::c_int, ...);
    pub fn swapcontext(uocp: *mut ucontext_t, ucp: *const ucontext_t) -> ::c_int;
}
