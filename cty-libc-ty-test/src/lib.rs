#![allow(unused)]

extern crate libc;
extern crate cty;

// Requires that `_x` and `_y` have the same type.
fn same_ty<T>(_x: T, _y: T) {}

macro_rules! test_tys {
    ($ty:ident) => {
        same_ty(0 as cty::$ty, 0 as libc::$ty);
    };
    ($($ty:ident,)*) => { $(test_tys!($ty);)* };
}

// This test is only executed for those platforms
// that libc supports.
#[cfg(any(
    windows,
    target_os = "redox",
    target_os = "cloudabi",
    target_os = "fuchsia",
    target_os = "switch",
    unix,
    target_env = "sgx",
))]
pub fn test() {
    test_tys! {
        int8_t, int16_t, int32_t, int64_t,
        uint8_t, uint16_t, uint32_t, uint64_t,

        c_schar, c_short, c_longlong,
        c_uchar, c_ushort, c_ulonglong,

        c_float, c_double,

        intmax_t, uintmax_t,

        size_t, ptrdiff_t,
        intptr_t, uintptr_t,
    }

    // FIXME: this type is not available in some targets,
    // but cty always exposes it:
    #[cfg(not(windows))]
    test_tys! {
        ssize_t,
    }

    // These are often target_os/target_env/target_arch dependent
    test_tys!{
        c_long, c_ulong,
        c_char, c_int, c_uint,
    }
}
