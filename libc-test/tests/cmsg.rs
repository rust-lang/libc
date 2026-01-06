//! Compare libc's CMSG(3) family of functions against the actual C macros, for
//! various inputs.

#[cfg(unix)]
mod t {

    use std::mem;

    use libc::{
        self,
        c_uchar,
        c_uint,
        c_void,
        cmsghdr,
        msghdr,
    };

    extern "C" {
        pub fn cmsg_firsthdr(msgh: *const msghdr) -> *mut cmsghdr;
        pub fn cmsg_nxthdr(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr;
        pub fn cmsg_space(length: c_uint) -> usize;
        pub fn cmsg_len(length: c_uint) -> usize;
        pub fn cmsg_data(cmsg: *const cmsghdr) -> *mut c_uchar;
    }

    #[test]
    fn test_cmsg_data() {
        for l in 0..128 {
            let pcmsghdr = l as *const cmsghdr;
            unsafe {
                assert_eq!(libc::CMSG_DATA(pcmsghdr), cmsg_data(pcmsghdr));
            }
        }
    }

    #[test]
    #[allow(unused_assignments)] // false-positive: https://github.com/rust-lang/rust/issues/147648
    fn test_cmsg_firsthdr() {
        let mut mhdr: msghdr = unsafe { mem::zeroed() };
        mhdr.msg_control = 0xdeadbeef as *mut c_void;
        let pmhdr = &mhdr as *const msghdr;
        for l in 0..128 {
            mhdr.msg_controllen = l;
            unsafe {
                assert_eq!(libc::CMSG_FIRSTHDR(pmhdr), cmsg_firsthdr(pmhdr));
            }
        }
    }

    #[test]
    fn test_cmsg_len() {
        for l in 0..128 {
            unsafe {
                assert_eq!(libc::CMSG_LEN(l) as usize, cmsg_len(l));
            }
        }
    }

    #[test]
    fn test_cmsg_nxthdr() {
        // Helps to align the buffer on the stack.
        #[repr(align(8))]
        struct Align8<T>(T);

        const CAPACITY: usize = 512;
        let mut buffer = Align8([0_u8; CAPACITY]);
        let pcmsghdr = buffer.0.as_mut_ptr().cast::<cmsghdr>();

        let mut mhdr: msghdr = unsafe { mem::zeroed() };
        mhdr.msg_control = pcmsghdr.cast::<c_void>();

        for trunc in 0..64 {
            mhdr.msg_controllen = (160 - trunc) as _;

            for cmsg_payload_len in 0..64 {
                // AIX does not apply any alignment or padding to ancillary
                // data and CMSG_ALIGN() is a noop. So only test addresses
                // that are multiples of the size of cmsghdr here.
                if cfg!(target_os = "aix") && cmsg_payload_len % std::mem::size_of::<cmsghdr>() != 0
                {
                    continue;
                }

                let mut current_cmsghdr_ptr = pcmsghdr;
                assert!(!current_cmsghdr_ptr.is_null());
                let mut count = 0;

                // Go from first cmsghdr to the last (until null) using various
                // cmsg_len increments. `cmsg_len` is set by us to check that
                // the jump to the next cmsghdr is correct with respect to
                // alignment and payload padding.
                while !current_cmsghdr_ptr.is_null() {
                    unsafe {
                        (*current_cmsghdr_ptr).cmsg_len =
                            libc::CMSG_LEN(cmsg_payload_len as _) as _;

                        let libc_next = libc::CMSG_NXTHDR(&mhdr, current_cmsghdr_ptr);
                        let system_next = cmsg_nxthdr(&mhdr, current_cmsghdr_ptr);
                        assert_eq!(
                            system_next, libc_next,
                            "msg_crontrollen: {}, payload_len: {}, count: {}",
                            mhdr.msg_controllen, cmsg_payload_len, count
                        );

                        current_cmsghdr_ptr = libc_next;
                        count += 1;
                    }
                }

                unsafe {
                    pcmsghdr.cast::<u8>().write_bytes(0, CAPACITY);
                }
            }
        }
    }

    #[test]
    fn test_cmsg_space() {
        unsafe {
            for l in 0..128 {
                assert_eq!(libc::CMSG_SPACE(l) as usize, cmsg_space(l));
            }
        }
    }
}
