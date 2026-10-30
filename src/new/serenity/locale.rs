//! Header: `locale.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Userland/Libraries/LibC/locale.h>
use crate::prelude::*;

pub const LC_ALL: c_int = 0;
pub const LC_NUMERIC: c_int = 1;
pub const LC_CTYPE: c_int = 2;
pub const LC_COLLATE: c_int = 3;
pub const LC_TIME: c_int = 4;
pub const LC_MONETARY: c_int = 5;
pub const LC_MESSAGES: c_int = 6;

pub const LC_NUMERIC_MASK: c_int = 1 << LC_NUMERIC;
pub const LC_CTYPE_MASK: c_int = 1 << LC_CTYPE;
pub const LC_COLLATE_MASK: c_int = 1 << LC_COLLATE;
pub const LC_TIME_MASK: c_int = 1 << LC_TIME;
pub const LC_MONETARY_MASK: c_int = 1 << LC_MONETARY;
pub const LC_MESSAGES_MASK: c_int = 1 << LC_MESSAGES;
pub const LC_ALL_MASK: c_int = LC_NUMERIC_MASK
    | LC_CTYPE_MASK
    | LC_COLLATE_MASK
    | LC_TIME_MASK
    | LC_MONETARY_MASK
    | LC_MESSAGES_MASK;

pub const LC_GLOBAL_LOCALE: crate::locale_t = -1isize as crate::locale_t;

s! {
    pub struct lconv {
        pub decimal_point: *mut c_char,
        pub thousands_sep: *mut c_char,
        pub grouping: *mut c_char,
        pub int_curr_symbol: *mut c_char,
        pub currency_symbol: *mut c_char,
        pub mon_decimal_point: *mut c_char,
        pub mon_thousands_sep: *mut c_char,
        pub mon_grouping: *mut c_char,
        pub positive_sign: *mut c_char,
        pub negative_sign: *mut c_char,
        pub int_frac_digits: c_char,
        pub frac_digits: c_char,
        pub p_cs_precedes: c_char,
        pub p_sep_by_space: c_char,
        pub n_cs_precedes: c_char,
        pub n_sep_by_space: c_char,
        pub p_sign_posn: c_char,
        pub n_sign_posn: c_char,
        pub int_p_cs_precedes: c_char,
        pub int_p_sep_by_space: c_char,
        pub int_n_cs_precedes: c_char,
        pub int_n_sep_by_space: c_char,
        pub int_p_sign_posn: c_char,
        pub int_n_sign_posn: c_char,
    }
}

extern "C" {
    pub fn freelocale(locale: crate::locale_t);
    pub fn getlocalename_l(category: c_int, locale: crate::locale_t) -> *const c_char;
    pub fn newlocale(
        category: c_int,
        locale: *const c_char,
        base: crate::locale_t,
    ) -> crate::locale_t;
    pub fn uselocale(locale: crate::locale_t) -> crate::locale_t;
}
