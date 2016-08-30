// Native C types
pub type c_char = i8;

/* Header <fcntl.h> */
pub const F_GETLK: ::c_int = 5;
pub const F_SETLK: ::c_int = 6;
pub const F_SETLKW: ::c_int = 7;
pub const F_GETOWN: ::c_int = 9;
pub const F_SETOWN: ::c_int = 8;

// O_CLOEXEC is defined in notbsd/mod.rs
pub const O_CREAT: ::c_int = 0x40;
pub const O_DIRECTORY: ::c_int = 0x10000;
pub const O_EXCL: ::c_int = 0x80;
pub const O_NOCTTY: ::c_int = 0x100;
pub const O_NOFOLLOW: ::c_int = 0x20000;
// O_TRUNC is defined in notbsd/mod.rs

pub const O_APPEND: ::c_int = 0x400;
pub const O_DSYNC: ::c_int = 0x1000;
pub const O_NONBLOCK: ::c_int = 0x800;
pub const O_RSYNC: ::c_int = 0x101000;
pub const O_SYNC: ::c_int = 0x101000;

// Here start non POSIX definitions.
pub const O_ASYNC: ::c_int = 0x2000;
pub const O_DIRECT: ::c_int = 0x4000;
pub const O_LARGEFILE: ::c_int = 0;
pub const O_NOATIME: ::c_int = 0x40000;
pub const O_PATH: ::c_int = 0x200000;
pub const O_TMPFILE: ::c_int = 0x410000;
pub const O_NDELAY: ::c_int = ::O_NONBLOCK;

