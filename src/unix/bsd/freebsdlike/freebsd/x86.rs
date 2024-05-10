pub type c_char = i8;
pub type c_long = i32;
pub type c_ulong = u32;
pub type wchar_t = i32;
pub type time_t = i32;
pub type suseconds_t = i32;
pub type register_t = i32;

s! {
    pub struct mcontext_t {
        pub mc_onstack: register_t,
        pub mc_gs: register_t,
        pub mc_fs: register_t,
        pub mc_es: register_t,
        pub mc_ds: register_t,
        pub mc_edi: register_t,
        pub mc_esi: register_t,
        pub mc_ebp: register_t,
        pub mc_isp: register_t,
        pub mc_ebx: register_t,
        pub mc_edx: register_t,
        pub mc_ecx: register_t,
        pub mc_eax: register_t,
        pub mc_trapno: register_t,
        pub mc_err: register_t,
        pub mc_eip: register_t,
        pub mc_cs: register_t,
        pub mc_eflags: register_t,
        pub mc_esp: register_t,
        pub mc_ss: register_t,
        pub mc_len: ::c_int,
        pub mc_fpformat: ::c_int,
        pub mc_ownedfp: ::c_int,
        pub mc_flags: register_t,
        pub mc_fpstate: [[::c_int; 32]; 4],
        pub mc_fsbase: register_t,
        pub mc_gsbase: register_t,
        pub mc_xfpustate: register_t,
        pub mc_xfpustate_len: register_t,
        pub mc_spare2: [::c_int; 4],
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_size: ::off_t,
        pub st_blocks: ::blkcnt_t,
        pub st_blksize: ::blksize_t,
        pub st_flags: ::fflags_t,
        pub st_gen: u32,
        pub st_lspare: i32,
        pub st_birthtime: ::time_t,
        pub st_birthtime_nsec: ::c_long,
        __unused: [u8; 8],
    }

    pub struct ucontext_t {
        pub uc_sigmask: ::sigset_t,
        pub uc_mcontext: ::mcontext_t,
        pub uc_link: *mut ::ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_flags: ::c_int,
        __spare__: [::c_int; 4],
    }
}

pub(crate) const _ALIGNBYTES: usize = ::mem::size_of::<::c_long>() - 1;

pub const MINSIGSTKSZ: ::size_t = 2048; // 512 * 4
