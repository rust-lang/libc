pub type Elf32_Addr = ::c_ulong;
pub type Elf32_Half = ::c_ushort;
pub type Elf32_Off = ::c_ulong;
pub type Elf32_Sword = ::c_long;
pub type Elf32_Word = ::c_ulong;
pub type Elf32_Lword = ::c_ulonglong;

pub type Elf_Addr = ::Elf32_Addr;
pub type Elf_Half = ::Elf32_Half;
pub type Elf_Off = ::Elf32_Off;
pub type Elf_Sword = ::Elf32_Sword;
pub type Elf_Word = ::Elf32_Word;
pub type Elf_Lword = ::Elf32_Lword;

pub type Elf_Phdr = ::Elf32_Phdr;

s! {
    pub struct Elf32_Phdr {
        pub p_type: ::Elf32_Word,
        pub p_offset: ::Elf32_Off,
        pub p_vaddr: ::Elf32_Addr,
        pub p_paddr: ::Elf32_Addr,
        pub p_filesz: ::Elf32_Word,
        pub p_memsz: ::Elf32_Word,
        pub p_flags: ::Elf32_Word,
        pub p_align: ::Elf32_Word,
    }
}
