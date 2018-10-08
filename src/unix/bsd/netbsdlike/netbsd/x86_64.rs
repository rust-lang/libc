use self::*;

pub type c_long = i64;
pub type c_ulong = u64;
pub type c_char = i8;

pub const PT_STEP: ::c_int = PT_FIRSTMACH + 0;
pub const PT_GETREGS: ::c_int = PT_FIRSTMACH + 1;
pub const PT_SETREGS: ::c_int = PT_FIRSTMACH + 2;
pub const PT_GETFPREGS: ::c_int = PT_FIRSTMACH + 3;
pub const PT_SETFPREGS: ::c_int = PT_FIRSTMACH + 4;
pub const PT_GETDBREGS: ::c_int = PT_FIRSTMACH + 5;
pub const PT_SETDBREGS: ::c_int = PT_FIRSTMACH + 6;
pub const PT_SETSTEP: ::c_int = PT_FIRSTMACH + 7;
pub const PT_CLEARSTEP: ::c_int = PT_FIRSTMACH + 8;
