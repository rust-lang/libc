use crate::prelude::*;

pub type __int64_t = c_long;
pub type __uint64_t = c_ulong;

pub type int_fast16_t = c_long;
pub type int_fast32_t = c_long;
pub type int_fast64_t = c_long;
pub type uint_fast16_t = c_ulong;
pub type uint_fast32_t = c_ulong;
pub type uint_fast64_t = c_ulong;

pub type __quad_t = c_long;
pub type __u_quad_t = c_ulong;
pub type __intmax_t = c_long;
pub type __uintmax_t = c_ulong;

pub type __ipc_pid_t = c_int;

pub type Elf64_Half = u16;
pub type Elf64_Word = u32;
pub type Elf64_Off = u64;
pub type Elf64_Addr = u64;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;
pub type Elf64_Section = u16;

pub type Elf_Addr = crate::Elf64_Addr;
pub type Elf_Half = crate::Elf64_Half;
pub type Elf_Ehdr = crate::Elf64_Ehdr;
pub type Elf_Phdr = crate::Elf64_Phdr;
pub type Elf_Shdr = crate::Elf64_Shdr;
pub type Elf_Sym = crate::Elf64_Sym;

// sys/ucontext.h
pub type greg_t = c_longlong;
pub type gregset_t = [greg_t; 23usize];

pub type fpregset_t = *mut _libc_fpstate;

s! {
    pub struct Elf64_Ehdr {
        pub e_ident: [c_uchar; 16],
        pub e_type: Elf64_Half,
        pub e_machine: Elf64_Half,
        pub e_version: Elf64_Word,
        pub e_entry: Elf64_Addr,
        pub e_phoff: Elf64_Off,
        pub e_shoff: Elf64_Off,
        pub e_flags: Elf64_Word,
        pub e_ehsize: Elf64_Half,
        pub e_phentsize: Elf64_Half,
        pub e_phnum: Elf64_Half,
        pub e_shentsize: Elf64_Half,
        pub e_shnum: Elf64_Half,
        pub e_shstrndx: Elf64_Half,
    }

    pub struct Elf64_Shdr {
        pub sh_name: Elf64_Word,
        pub sh_type: Elf64_Word,
        pub sh_flags: Elf64_Xword,
        pub sh_addr: Elf64_Addr,
        pub sh_offset: Elf64_Off,
        pub sh_size: Elf64_Xword,
        pub sh_link: Elf64_Word,
        pub sh_info: Elf64_Word,
        pub sh_addralign: Elf64_Xword,
        pub sh_entsize: Elf64_Xword,
    }

    pub struct Elf64_Sym {
        pub st_name: Elf64_Word,
        pub st_info: c_uchar,
        pub st_other: c_uchar,
        pub st_shndx: Elf64_Section,
        pub st_value: Elf64_Addr,
        pub st_size: Elf64_Xword,
    }

    pub struct Elf64_Phdr {
        pub p_type: crate::Elf64_Word,
        pub p_flags: crate::Elf64_Word,
        pub p_offset: crate::Elf64_Off,
        pub p_vaddr: crate::Elf64_Addr,
        pub p_paddr: crate::Elf64_Addr,
        pub p_filesz: crate::Elf64_Xword,
        pub p_memsz: crate::Elf64_Xword,
        pub p_align: crate::Elf64_Xword,
    }

    // sys/ucontext.h
    pub struct _libc_fpxreg {
        pub significand: [c_ushort; 4],
        pub exponent: c_ushort,
        __glibc_reserved1: Padding<[c_ushort; 3]>,
    }

    pub struct _libc_xmmreg {
        pub element: [crate::__uint32_t; 4],
    }

    pub struct _libc_fpstate {
        pub cwd: crate::__uint16_t,
        pub swd: crate::__uint16_t,
        pub ftw: crate::__uint16_t,
        pub fop: crate::__uint16_t,
        pub rip: crate::__uint64_t,
        pub rdp: crate::__uint64_t,
        pub mxcsr: crate::__uint32_t,
        pub mxcr_mask: crate::__uint32_t,
        pub _st: [_libc_fpxreg; 8],
        pub _xmm: [_libc_xmmreg; 16],
        __glibc_reserved1: Padding<[crate::__uint32_t; 24]>,
    }

    pub struct mcontext_t {
        pub gregs: crate::gregset_t,
        pub fpregs: fpregset_t,
        __reserved1: Padding<[c_ulonglong; 8]>,
    }

    pub struct ucontext_t {
        pub uc_flags: c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: crate::stack_t,
        pub uc_mcontext: mcontext_t,
        pub uc_sigmask: crate::sigset_t,
        __fpregs_mem: _libc_fpstate,
        __ssp: Padding<[c_ulonglong; 4]>,
    }

    // sys/ipc.h
    pub struct ipc_perm {
        pub __key: crate::key_t,
        pub uid: crate::uid_t,
        pub gid: crate::gid_t,
        pub cuid: crate::uid_t,
        pub cgid: crate::gid_t,
        pub mode: crate::mode_t,
        pub __seq: c_ushort,
    }

    pub struct fd_set {
        pub fds_bits: [crate::__fd_mask; 4],
    }
}

// sys/ucontext.h
pub const REG_R8: c_uint = 0;
pub const REG_R9: c_uint = 1;
pub const REG_R10: c_uint = 2;
pub const REG_R11: c_uint = 3;
pub const REG_R12: c_uint = 4;
pub const REG_R13: c_uint = 5;
pub const REG_R14: c_uint = 6;
pub const REG_R15: c_uint = 7;
pub const REG_RDI: c_uint = 8;
pub const REG_RSI: c_uint = 9;
pub const REG_RBP: c_uint = 10;
pub const REG_RSP: c_uint = 11;
pub const REG_RBX: c_uint = 12;
pub const REG_RDX: c_uint = 13;
pub const REG_RCX: c_uint = 14;
pub const REG_RAX: c_uint = 15;
pub const REG_RIP: c_uint = 16;
pub const REG_CS: c_uint = 17;
pub const REG_RFL: c_uint = 18;
pub const REG_ERR: c_uint = 19;
pub const REG_TRAPNO: c_uint = 20;
pub const REG_OLDMASK: c_uint = 21;
pub const REG_CR2: c_uint = 22;

// stdint.h
pub const INTPTR_WIDTH: usize = 64;
pub const UINTPTR_WIDTH: usize = 64;
pub const PTRDIFF_WIDTH: usize = 64;
pub const SIZE_WIDTH: usize = 64;

// stdint.h (word-sized on this ABI)
pub const INT_FAST16_WIDTH: usize = 64;
pub const UINT_FAST16_WIDTH: usize = 64;
pub const INT_FAST32_WIDTH: usize = 64;
pub const UINT_FAST32_WIDTH: usize = 64;

// bits/pthreadtypes-arch.h, sys/ucontext.h
pub const __SIZEOF_PTHREAD_ATTR_T: usize = 48;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 48;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 40;
pub const __SIZEOF_PTHREAD_COND_T: usize = 40;
pub const __NGREG: usize = 23;
pub const NGREG: usize = 23;

// bits/ioctls.h, bits/dl_find_object.h
pub const OSIOCGIFCONF: c_ulong = 0xc0080194;
pub const SIOCGIFCONF: c_ulong = 0xc00801a4;
pub const DLFO_STRUCT_HAS_EH_DBASE: usize = 0;
