cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod b64;
        pub use self::b64::*;
    } else {
        // Unknown target_arch
    }
}
