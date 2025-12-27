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
        let mut buf = [0u8; 256];
        let mut mhdr: msghdr = unsafe { mem::zeroed() };
        mhdr.msg_control = buf.as_mut_ptr().cast::<c_void>();

        for l in 0..128 {
            mhdr.msg_controllen = l as _;
            unsafe {
                assert_eq!(libc::CMSG_FIRSTHDR(&mhdr), cmsg_firsthdr(&mhdr));
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
        let mut mhdr: msghdr = unsafe { mem::zeroed() };
        for start_ofs in 0..64 {
            let pcmsghdr = buffer.0.as_mut_ptr().cast::<cmsghdr>();
            mhdr.msg_control = pcmsghdr.cast::<c_void>();
            mhdr.msg_controllen = (160 - start_ofs) as _;
            for cmsg_payload_len in 0..64 {
                unsafe {
                    pcmsghdr.cast::<u8>().write_bytes(0, CAPACITY);
                    (*pcmsghdr).cmsg_len = libc::CMSG_LEN(cmsg_payload_len as _) as _;
                    let libc_next = libc::CMSG_NXTHDR(&mhdr, pcmsghdr);
                    let next = cmsg_nxthdr(&mhdr, pcmsghdr);
                    assert_eq!(libc_next, next);
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
