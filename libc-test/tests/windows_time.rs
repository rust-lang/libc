//! Ensures Windows `time`-related routines align with `libc`'s interface. By
//! default, both MSVC and GNU (under `mingw`) expose 64-bit symbols, but in
//! stable we need to cope with a 32-bit `time-t`, so the routines should link
//! to their 32-bit variants.

#![cfg(windows)]

/// Ensures a 64-bit write is performed on values that should always be 64 bits,
/// and that a corresponding 32-bit write is performed on values that should be
/// 32 bits. This basically makes sure `time_t`'s bit-width aligns with those of
/// functions that expect it as a parameter.
#[test]
fn test_bitwidth_store() {
    #[cfg(all(target_arch = "x86", target_env = "gnu"))]
    assert_eq!(size_of::<libc::time_t>(), 4);
    #[cfg(not(all(target_arch = "x86", target_env = "gnu")))]
    assert_eq!(size_of::<libc::time_t>(), 8);

    let mut time_values: [libc::time_t; 2] = [123, 456];
    let ptr = time_values.as_mut_ptr();

    unsafe { libc::time(ptr) };

    assert_ne!(time_values[0], 123);
    assert_eq!(time_values[1], 456);
}
