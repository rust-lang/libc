//! Compare libc's KERNEL_VERSION macro against a specific kernel version.

#[cfg(target_os = "linux")]
mod t {
    use libc;

    #[test]
    fn test_kernel_version() {
        assert_eq!(unsafe { libc::KERNEL_VERSION(6, 0, 0) }, 393216);
        // Check that the patch level saturates
        assert_eq!(unsafe { libc::KERNEL_VERSION(6, 0, 255) }, 393471);
        assert_eq!(unsafe { libc::KERNEL_VERSION(6, 0, 256) }, 393471);
        assert_eq!(unsafe { libc::KERNEL_VERSION(6, 0, 300) }, 393471);
        assert_eq!(unsafe { libc::KERNEL_VERSION(6, 0, u32::MAX) }, 393471);
    }
}
