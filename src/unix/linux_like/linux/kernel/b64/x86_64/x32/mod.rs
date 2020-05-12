pub const __X32_SYSCALL_BIT: ::c_long = 0x40000000;

mod syscalls;
pub use self::syscalls::*;
