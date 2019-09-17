cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[repr(C, align(16))] pub struct max_align_t([f64; 4]);
    } else if #[cfg(target_pointer_width = "32")] {
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[repr(C, align(16))] pub struct max_align_t([i64; 6]);
    }
}
