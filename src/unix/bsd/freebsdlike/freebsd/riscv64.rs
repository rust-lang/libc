pub type c_char = u8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type wchar_t = ::c_int;
pub type time_t = i64;
pub type suseconds_t = ::c_long;
pub type register_t = i64;

s! {
    pub struct gpregs {
        pub gp_ra: ::register_t,
        pub gp_sp: ::register_t,
        pub gp_gp: ::register_t,
        pub gp_tp: ::register_t,
        pub gp_t: [::register_t; 7],
        pub gp_s: [::register_t; 12],
        pub gp_a: [::register_t; 8],
        pub gp_sepc: ::register_t,
        pub gp_sstatus: ::register_t,
    }

    pub struct fpregs {
        pub fp_x: [[::register_t; 2]; 32],
        pub fp_fcsr: ::register_t,
        pub fp_flags: ::c_int,
        pub fp_pad: ::c_int,
    }

    pub struct mcontext_t {
        pub mc_gpregs: gpregs,
        pub mc_fpregs: fpregs,
        pub mc_flags: ::c_int,
        pub mc_pad: ::c_int,
        pub mc_spare: [u64; 8],
    }
}

pub(crate) const _ALIGNBYTES: usize = ::mem::size_of::<::c_longlong>() - 1;

pub const MAP_32BIT: ::c_int = 0x00080000;
pub const MINSIGSTKSZ: ::size_t = 4096; // 1024 * 4
