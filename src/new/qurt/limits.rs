//! Header: `limits.h`

use crate::prelude::*;

// Character properties
pub const CHAR_BIT: c_uint = 8;
pub const CHAR_MAX: c_char = 255; // unsigned char on Hexagon
pub const CHAR_MIN: c_char = 0;
pub const SCHAR_MAX: c_schar = 127;
pub const SCHAR_MIN: c_schar = -128;
pub const UCHAR_MAX: c_uchar = 255;

// Integer limits (ILP32 architecture)
pub const INT_MAX: c_int = 2147483647;
pub const INT_MIN: c_int = -2147483648;
pub const UINT_MAX: c_uint = 4294967295;

pub const LONG_MAX: c_long = 2147483647; // 32-bit on Hexagon ILP32
pub const LONG_MIN: c_long = -2147483648; // 32-bit on Hexagon ILP32
pub const ULONG_MAX: c_ulong = 4294967295; // 32-bit on Hexagon ILP32

pub const SHRT_MAX: c_short = 32767;
pub const SHRT_MIN: c_short = -32768;
pub const USHRT_MAX: c_ushort = 65535;

// POSIX limits
pub const PATH_MAX: c_int = 260;
pub const NAME_MAX: c_int = 255;
pub const IOV_MAX: c_int = 16;
pub const ARG_MAX: c_int = 4096;
pub const CHILD_MAX: c_int = 25;
pub const LINK_MAX: c_int = 8;
pub const MAX_CANON: c_int = 255;
pub const MAX_INPUT: c_int = 255;
pub const OPEN_MAX: c_int = 16;
pub const PIPE_BUF: c_int = 4096;
pub const STREAM_MAX: c_int = 8;
pub const TZNAME_MAX: c_int = 6;
