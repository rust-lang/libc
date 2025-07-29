//! Compare libc's SIGRTMAX and SIGRTMIN functions against the actual C macros

#[cfg(any(
    target_os = "linux",
    target_os = "l4re",
    target_os = "android",
    target_os = "emscripten",
    target_os = "solaris",
    target_os = "illumos",
))]
mod t {
    use libc;

    extern "C" {
        pub fn sigrtmax() -> libc::c_int;
        pub fn sigrtmin() -> libc::c_int;
    }

    #[test]
    fn test_sigrtmax() {
        unsafe {
            assert_eq!(libc::SIGRTMAX(), sigrtmax());
        }
    }

    #[test]
    fn test_sigrtmin() {
        unsafe {
            assert_eq!(libc::SIGRTMIN(), sigrtmin());
        }
    }
}
