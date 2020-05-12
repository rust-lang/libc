cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        mod x32;
        pub use self::x32::*;
    } else {
        mod not_x32;
        pub use self::not_x32::*;
    }
}
