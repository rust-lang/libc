use crate::prelude::*;

pub type __int64_t = c_longlong;
pub type __uint64_t = c_ulonglong;

pub type int_fast16_t = c_int;
pub type int_fast32_t = c_int;
pub type int_fast64_t = c_longlong;
pub type uint_fast16_t = c_uint;
pub type uint_fast32_t = c_uint;
pub type uint_fast64_t = c_ulonglong;

pub type __quad_t = c_longlong;
pub type __u_quad_t = c_ulonglong;
pub type __intmax_t = c_longlong;
pub type __uintmax_t = c_ulonglong;

pub type __squad_type = __int64_t;
pub type __uquad_type = __uint64_t;
pub type __sword_type = c_int;
pub type __uword_type = c_uint;
pub type __slong32_type = c_long;
pub type __ulong32_type = c_ulong;
pub type __s64_type = __int64_t;
pub type __u64_type = __uint64_t;

pub type __ipc_pid_t = c_ushort;

pub type Elf_Addr = crate::Elf32_Addr;
pub type Elf_Half = crate::Elf32_Half;
pub type Elf_Ehdr = crate::Elf32_Ehdr;
pub type Elf_Phdr = crate::Elf32_Phdr;
pub type Elf_Shdr = crate::Elf32_Shdr;
pub type Elf_Sym = crate::Elf32_Sym;
