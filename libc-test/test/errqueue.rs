//! Compare libc's SO_EE_OFFENDER function against the actual C macro

extern crate libc;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod t {
    use libc::{self, sock_extended_err, sockaddr};

    extern "C" {
        pub fn so_ee_offender(ee: *const sock_extended_err) -> *mut sockaddr;
    }

    #[test]
    fn test_cmsg_data() {
        for l in 0..128 {
            let ee = l as *const sock_extended_err;
            unsafe {
                assert_eq!(libc::SO_EE_OFFENDER(ee), so_ee_offender(ee));
            }
        }
    }
}
