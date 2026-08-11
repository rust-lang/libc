//! Header: `limits.h`

use super::*;
use crate::prelude::*;

// Character properties
pub const CHAR_BIT: c_uint = 8;
#[deprecated(since = "0.2.190", note = "Use `c_char::MAX` instead.")]
pub const CHAR_MAX: c_char = 255; // unsigned char on Hexagon
#[deprecated(since = "0.2.190", note = "Use `c_char::MIN` instead.")]
pub const CHAR_MIN: c_char = 0;
#[deprecated(since = "0.2.190", note = "Use `c_schar::MAX` instead.")]
pub const SCHAR_MAX: i8 = 127;
#[deprecated(since = "0.2.190", note = "Use `c_schar::MIN` instead.")]
pub const SCHAR_MIN: i8 = -128;
#[deprecated(since = "0.2.190", note = "Use `c_uchar::MAX` instead.")]
pub const UCHAR_MAX: c_uchar = 255;

// Integer properties
#[deprecated(since = "0.2.190", note = "Use `c_int::MAX` instead.")]
pub const INT_MAX: c_int = 2147483647;
#[deprecated(since = "0.2.190", note = "Use `c_int::MIN` instead.")]
pub const INT_MIN: c_int = -2147483647 - 1;
#[deprecated(since = "0.2.190", note = "Use `c_uint::MAX` instead.")]
pub const UINT_MAX: c_uint = 4294967295;

#[deprecated(since = "0.2.190", note = "Use `c_long::MAX` instead.")]
pub const LONG_MAX: c_long = 2147483647;
#[deprecated(since = "0.2.190", note = "Use `c_long::MIN` instead.")]
pub const LONG_MIN: c_long = -2147483647 - 1;
#[deprecated(since = "0.2.190", note = "Use `c_ulong::MAX` instead.")]
pub const ULONG_MAX: c_ulong = 4294967295;

#[deprecated(since = "0.2.190", note = "Use `c_short::MAX` instead.")]
pub const SHRT_MAX: c_short = 32767;
#[deprecated(since = "0.2.190", note = "Use `c_short::MIN` instead.")]
pub const SHRT_MIN: c_short = -32768;
#[deprecated(since = "0.2.190", note = "Use `c_ushort::MAX` instead.")]
pub const USHRT_MAX: c_ushort = 65535;

// POSIX Limits
pub const ARG_MAX: c_int = 4096;
pub const CHILD_MAX: c_int = 25;

/// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
/// for details.
pub const LINK_MAX: c_int = 8;

pub const MAX_CANON: c_int = 255;
pub const MAX_INPUT: c_int = 255;

/// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
/// for details.
pub const NAME_MAX: c_int = 255;

pub const OPEN_MAX: c_int = 20;

/// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
/// for details.
pub const PATH_MAX: c_int = 260;

pub const PIPE_BUF: c_int = 512;
pub const STREAM_MAX: c_int = 20;
pub const TZNAME_MAX: c_int = 50;

// Additional limits
pub const IOV_MAX: c_int = 16;
