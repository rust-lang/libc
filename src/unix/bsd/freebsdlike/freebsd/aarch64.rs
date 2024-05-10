pub type c_char = u8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type wchar_t = u32;
pub type time_t = i64;
pub type suseconds_t = i64;
pub type register_t = i64;

s! {
    pub struct gpregs {
        pub gp_x: [::register_t; 30],
        pub gp_lr: ::register_t,
        pub gp_sp: ::register_t,
        pub gp_elr: ::register_t,
        pub gp_spsr: u32,
        pub gp_pad: ::c_int,
    }

    pub struct fpregs {
        pub fp_q: u128,
        pub fp_sr: u32,
        pub fp_cr: u32,
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
