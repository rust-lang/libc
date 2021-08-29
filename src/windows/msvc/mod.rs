pub const L_tmpnam: ::c_uint = 260;
pub const TMP_MAX: ::c_uint = 0x7fff_ffff;

// POSIX Supplement (from errno.h)
// This particular error code is only currently available in msvc toolchain
pub const EOTHER: ::c_int = 131;

extern "C" {
    #[link_name = "_stricmp"]
    pub fn stricmp(s1: *const ::c_char, s2: *const ::c_char) -> ::c_int;
    #[link_name = "_strnicmp"]
    pub fn strnicmp(s1: *const ::c_char, s2: *const ::c_char, n: ::size_t) -> ::c_int;
}

#[link(name = "Bcrypt")]
extern "stdcall" {
    pub fn BCryptOpenAlgorithmProvider(
        phAlgorithm: *mut *mut ::c_void,
        pszAlgId: *mut ::wchar_t,
        pszImplementation: *mut ::wchar_t,
        dwFlags: ::c_ulong,
    ) -> ::c_long;
    pub fn BCryptGenRandom(
        hAlgorithm: *mut ::c_void,
        pbBuffer: *mut ::c_uchar,
        cbBuffer: ::c_ulong,
        dwFlags: ::c_ulong,
    ) -> ::c_long;
    pub fn BCryptCloseAlgorithmProvider(hAlgorithm: *mut ::c_void, dwFlags: ::c_ulong) -> ::c_long;
}

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    } else {
        // Unknown target_arch
    }
}
