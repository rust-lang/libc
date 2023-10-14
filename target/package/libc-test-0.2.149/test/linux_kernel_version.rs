//! Compare libc's KERNEL_VERSION macro against a specific kernel version.

extern crate libc;

#[cfg(
    target_os = "linux",
)]
mod t {
    use libc;

    #[test]
    fn test_kernel_version() {
        unsafe {
            assert_eq!(libc::KERNEL_VERSION(6, 0, 0), 393216);
        }
    }
}
