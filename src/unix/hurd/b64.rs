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

pub type __squad_type = c_long;
pub type __uquad_type = c_ulong;
pub type __sword_type = c_long;
pub type __uword_type = c_ulong;
pub type __slong32_type = c_int;
pub type __ulong32_type = c_uint;
pub type __s64_type = c_long;
pub type __u64_type = c_ulong;

pub type __ipc_pid_t = c_int;

pub type Elf_Addr = crate::Elf64_Addr;
pub type Elf_Half = crate::Elf64_Half;
pub type Elf_Ehdr = crate::Elf64_Ehdr;
pub type Elf_Phdr = crate::Elf64_Phdr;
pub type Elf_Shdr = crate::Elf64_Shdr;
pub type Elf_Sym = crate::Elf64_Sym;
