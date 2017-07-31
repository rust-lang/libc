pub type c_char = i8;
pub type wchar_t = i32;

pub const O_DIRECT: ::c_int = 0x4000;
pub const O_DIRECTORY: ::c_int = 0x10000;
pub const O_NOFOLLOW: ::c_int = 0x20000;
pub const O_LARGEFILE: ::c_int = 0o0100000;

pub const MAP_32BIT: ::c_int = 0x40;
