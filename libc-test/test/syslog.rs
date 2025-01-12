#[cfg(unix)]
mod t {
    use libc::c_int;

    use libc::LOG_ALERT;
    use libc::LOG_CRIT;
    use libc::LOG_DEBUG;
    use libc::LOG_EMERG;
    use libc::LOG_ERR;
    use libc::LOG_INFO;
    use libc::LOG_NOTICE;
    use libc::LOG_WARNING;

    use libc::LOG_MASK;
    use libc::LOG_UPTO;

    extern "C" {
        pub fn LOG_MASK_ffi(priority: c_int) -> c_int;
        pub fn LOG_UPTO_ffi(priority: c_int) -> c_int;
    }

    #[test]
    fn test_log_mask() {
        // Ensure our Rust impl returns the same value as the C macros

        // *_ffi interfaces are unsafe
        unsafe {
            assert_eq!(LOG_MASK(LOG_EMERG), LOG_MASK_ffi(LOG_EMERG));
            assert_eq!(LOG_MASK(LOG_ALERT), LOG_MASK_ffi(LOG_ALERT));
            assert_eq!(LOG_MASK(LOG_CRIT), LOG_MASK_ffi(LOG_CRIT));
            assert_eq!(LOG_MASK(LOG_ERR), LOG_MASK_ffi(LOG_ERR));
            assert_eq!(LOG_MASK(LOG_WARNING), LOG_MASK_ffi(LOG_WARNING));
            assert_eq!(LOG_MASK(LOG_NOTICE), LOG_MASK_ffi(LOG_NOTICE));
            assert_eq!(LOG_MASK(LOG_INFO), LOG_MASK_ffi(LOG_INFO));
            assert_eq!(LOG_MASK(LOG_DEBUG), LOG_MASK_ffi(LOG_DEBUG));
        }
    }

    #[test]
    fn test_log_upto() {
        // Ensure our Rust impl returns the same value as the C macros

        // *_ffi interfaces are unsafe
        unsafe {
            assert_eq!(LOG_UPTO(LOG_EMERG), LOG_UPTO_ffi(LOG_EMERG));
            assert_eq!(LOG_UPTO(LOG_ALERT), LOG_UPTO_ffi(LOG_ALERT));
            assert_eq!(LOG_UPTO(LOG_CRIT), LOG_UPTO_ffi(LOG_CRIT));
            assert_eq!(LOG_UPTO(LOG_ERR), LOG_UPTO_ffi(LOG_ERR));
            assert_eq!(LOG_UPTO(LOG_WARNING), LOG_UPTO_ffi(LOG_WARNING));
            assert_eq!(LOG_UPTO(LOG_NOTICE), LOG_UPTO_ffi(LOG_NOTICE));
            assert_eq!(LOG_UPTO(LOG_INFO), LOG_UPTO_ffi(LOG_INFO));
            assert_eq!(LOG_UPTO(LOG_DEBUG), LOG_UPTO_ffi(LOG_DEBUG));
        }
    }
}
