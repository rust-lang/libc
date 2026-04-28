//! Ensures Windows `time`-related routines align with `libc`'s interface. By
//! default, both MSVC and GNU (under `mingw`) expose 64-bit symbols. `libc`
//! also does that, but there's been slight inconsistencies in the past.

#![cfg(windows)]

/// Ensures a 64-bit write is performed on values that should always be 64 bits.
/// This may fail if
/// (1) the routine links with its 32-bit variant. This only happens if
///     `_USE_32BIT_TIME_T` is defined. In theory, this should not be
///     possible when working with Rust's `libc`.
/// (2) Or `time_t` is 32-bits, and a 64-bit write overwrites both array items.
///     This should neither be possible unless the above macro is defined.
///     Support for non-64-bit values in `libc` should thus be non-existent.
#[test]
fn test_64_bit_store() {
    let mut time_values: [libc::time_t; 2] = [123, 456];
    let ptr = time_values.as_mut_ptr();
    unsafe { libc::time(ptr) };
    assert!(time_values[0] != 123);
    assert_eq!(time_values[1], 456);
}
