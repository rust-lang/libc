pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type wchar_t = i32;
pub type time_t = i64;
pub type suseconds_t = i64;
pub type register_t = i64;

s! {
    pub struct reg32 {
        pub r_fs: u32,
        pub r_es: u32,
        pub r_ds: u32,
        pub r_edi: u32,
        pub r_esi: u32,
        pub r_ebp: u32,
        pub r_isp: u32,
        pub r_ebx: u32,
        pub r_edx: u32,
        pub r_ecx: u32,
        pub r_eax: u32,
        pub r_trapno: u32,
        pub r_err: u32,
        pub r_eip: u32,
        pub r_cs: u32,
        pub r_eflags: u32,
        pub r_esp: u32,
        pub r_ss: u32,
        pub r_gs: u32,
    }

    pub struct reg {
        pub r_r15: i64,
        pub r_r14: i64,
        pub r_r13: i64,
        pub r_r12: i64,
        pub r_r11: i64,
        pub r_r10: i64,
        pub r_r9: i64,
        pub r_r8: i64,
        pub r_rdi: i64,
        pub r_rsi: i64,
        pub r_rbp: i64,
        pub r_rbx: i64,
        pub r_rdx: i64,
        pub r_rcx: i64,
        pub r_rax: i64,
        pub r_trapno: u32,
        pub r_fs: u16,
        pub r_gs: u16,
        pub r_err: u32,
        pub r_es: u16,
        pub r_ds: u16,
        pub r_rip: i64,
        pub r_cs: i64,
        pub r_rflags: i64,
        pub r_rsp: i64,
        pub r_ss: i64,
    }

    pub struct fpreg32 {
        pub fpr_env: [u32; 7],
        pub fpr_acc: [[u8; 10]; 8],
        pub fpr_ex_sw: u32,
        pub fpr_pad: [u8; 64],
    }

    pub struct fpreg {
        pub fpr_env: [u64; 4],
        pub fpr_acc: [[u8; 16]; 8],
        pub fpr_xacc: [[u8; 16]; 16],
        pub fpr_spare: [u64; 12],
    }

    pub struct xmmreg {
        pub xmm_env: [u32; 8],
        pub xmm_acc: [[u8; 16]; 8],
        pub xmm_reg: [[u8; 16]; 8],
        pub xmm_pad: [u8; 224],
    }

    pub union __c_anonymous_elf64_auxv_union {
        pub a_val: ::c_long,
        pub a_ptr: *mut ::c_void,
        pub a_fcn: extern "C" fn(),
    }

    pub struct Elf64_Auxinfo {
        pub a_type: ::c_long,
        pub a_un: __c_anonymous_elf64_auxv_union,
    }

    pub struct kinfo_file {
        pub kf_structsize: ::c_int,
        pub kf_type: ::c_int,
        pub kf_fd: ::c_int,
        pub kf_ref_count: ::c_int,
        pub kf_flags: ::c_int,
        _kf_pad0: ::c_int,
        pub kf_offset: i64,
        _priv: [::uintptr_t; 38], // FIXME if needed
        pub kf_status: u16,
        _kf_pad1: u16,
        _kf_ispare0: ::c_int,
        pub kf_cap_rights: ::cap_rights_t,
        _kf_cap_spare: u64,
        pub kf_path: [::c_char; ::PATH_MAX as usize],
    }
}

pub(crate) const _ALIGNBYTES: usize = ::mem::size_of::<::c_long>() - 1;

pub const MAP_32BIT: ::c_int = 0x00080000;
pub const MINSIGSTKSZ: ::size_t = 2048; // 512 * 4

pub const _MC_HASSEGS: u32 = 0x1;
pub const _MC_HASBASES: u32 = 0x2;
pub const _MC_HASFPXSTATE: u32 = 0x4;
pub const _MC_FLAG_MASK: u32 = _MC_HASSEGS | _MC_HASBASES | _MC_HASFPXSTATE;

pub const _MC_FPFMT_NODEV: c_long = 0x10000;
pub const _MC_FPFMT_XMM: c_long = 0x10002;
pub const _MC_FPOWNED_NONE: c_long = 0x20000;
pub const _MC_FPOWNED_FPU: c_long = 0x20001;
pub const _MC_FPOWNED_PCB: c_long = 0x20002;

mod align;
pub use self::align::*;
