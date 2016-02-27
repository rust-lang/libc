pub type c_char = i8;
pub type wchar_t = i32;

pub const O_DIRECT: ::c_int = 0x4000;
pub const O_DIRECTORY: ::c_int = 0x10000;
pub const O_NOFOLLOW: ::c_int = 0x20000;

pub const MAP_LOCKED: ::c_int = 0x02000;
pub const MAP_NORESERVE: ::c_int = 0x04000;

pub const EDEADLOCK: ::c_int = 35;

pub const SO_PEERCRED: ::c_int = 17;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_SNDTIMEO: ::c_int = 21;

pub const FIOCLEX: ::c_ulong = 0x5451;
pub const FIONBIO: ::c_ulong = 0x5421;

type greg_t = ::c_int;

const NREG : usize = 19;

type gregset_t = [greg_t; NREG];

s! {

    pub struct _libc_fpreg {
        significand: [::c_ushort; 4],
        exponent: ::c_ushort,
    }

    pub struct _libc_fpstate {
        cw: ::c_ulong,
        sw: ::c_ulong,
        tag: ::c_ulong,
        ipoff: ::c_ulong,
        cssel: ::c_ulong,
        dataoff: ::c_ulong,
        datasel: ::c_ulong,
        _st: [_libc_fpreg; 8],
        status: ::c_ulong,
    }

    pub struct mcontext_t {
        __gregs: gregset_t,
        __fpregs: *mut _libc_fpstate,
        __oldmask: ::c_ulong,
        __cr2: ::c_ulong,
    }

    pub struct ucontext_t {
        pub uc_flags: ::c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_mcontext: mcontext_t,
        pub uc_sigmask: ::sigset_t,
        __fpregs_mem: _libc_fpstate,
    }

}

extern {
    pub fn getcontext(ucp: *mut ucontext_t) -> ::c_int;
    pub fn setcontext(ucp: *const ucontext_t) -> ::c_int;
    pub fn makecontext(ucp: *mut ucontext_t, func:  extern fn (), argc: ::c_int, ...);
    pub fn swapcontext(uocp: *mut ucontext_t, ucp: *const ucontext_t) -> ::c_int;
}
