//! Compare libc's NLMSG_* functions against the actual C macros, for various inputs.

#![cfg(target_os = "linux")]

use libc::{
    self,
    c_int,
    c_void,
    nlmsghdr,
    size_t,
};

extern "C" {
    fn nlmsg_align_ffi(size: size_t) -> c_int;
    fn nlmsg_length_ffi(size: size_t) -> c_int;
    fn nlmsg_space_ffi(size: size_t) -> c_int;
    fn nlmsg_data_ffi(nlh: *mut nlmsghdr) -> *mut c_void;
    fn nlmsg_next_ffi(nlh: *mut nlmsghdr, size: *mut c_int) -> *mut nlmsghdr;
    fn nlmsg_ok_ffi(nlh: *mut nlmsghdr, size: c_int) -> c_int;
    fn nlmsg_payload_ffi(nlh: *mut nlmsghdr, size: c_int) -> c_int;
}

#[test]
fn test_sizes() {
    for size in [0, 1, 2, 3, 4, 5, 15, 16, 17, 100, 4095, 4096] {
        assert_eq!(libc::NLMSG_ALIGN(size), unsafe { nlmsg_align_ffi(size) });
        assert_eq!(libc::NLMSG_LENGTH(size), unsafe { nlmsg_length_ffi(size) });
        assert_eq!(libc::NLMSG_SPACE(size), unsafe { nlmsg_space_ffi(size) });
    }
}

/// Write message headers with lengths `lens` into `buf`, at NLMSG_ALIGN spacing.
///
/// `[u32]` keeps the buffer 4-byte aligned for `nlmsghdr`; NLMSG_ALIGN keeps every
/// message offset a multiple of 4, so each header dereference stays aligned.
fn fill(buf: &mut [u32], lens: &[u32]) -> c_int {
    let mut total = 0;
    for len in lens {
        assert!(total + size_of::<nlmsghdr>() <= size_of_val(buf));
        let hdr = nlmsghdr {
            nlmsg_len: *len,
            nlmsg_type: 0,
            nlmsg_flags: 0,
            nlmsg_seq: 0,
            nlmsg_pid: 0,
        };
        unsafe {
            buf.as_mut_ptr()
                .cast::<u8>()
                .add(total)
                .cast::<nlmsghdr>()
                .write(hdr);
        }
        total += libc::NLMSG_ALIGN(*len as size_t) as usize;
    }
    assert!(total <= size_of_val(buf));
    total as c_int
}

// Walk a multipart message with the Rust and C implementations in lockstep,
// comparing every step.
#[test]
fn test_walk() {
    let mut buf = [0u32; 32];
    let total = fill(&mut buf, &[21, 16, 30]);

    let mut nlh = buf.as_mut_ptr().cast::<nlmsghdr>();
    let mut len = total;
    let mut len_c = total;
    let mut seen = 0;
    loop {
        let ok = unsafe { libc::NLMSG_OK(nlh, len) };
        assert_eq!(ok, unsafe { nlmsg_ok_ffi(nlh, len_c) } != 0);
        if !ok {
            break;
        }
        unsafe {
            assert_eq!(libc::NLMSG_DATA(nlh), nlmsg_data_ffi(nlh));
            for size in [0, 4, 8] {
                assert_eq!(libc::NLMSG_PAYLOAD(nlh, size), nlmsg_payload_ffi(nlh, size));
            }
        }
        let next = unsafe { libc::NLMSG_NEXT(nlh, &mut len) };
        let next_c = unsafe { nlmsg_next_ffi(nlh, &mut len_c) };
        assert_eq!(next, next_c);
        assert_eq!(len, len_c);
        nlh = next;
        seen += 1;
    }
    assert_eq!(seen, 3);
    assert_eq!(len, 0);
}

// Boundary cases for the validity check: exact fit, short buffer, truncated
// header, message longer than the remaining buffer.
#[test]
fn test_ok_boundaries() {
    let mut buf = [0u32; 32];
    for (nlmsg_len, len) in [(16, 16), (16, 15), (8, 16), (100, 16)] {
        fill(&mut buf, &[nlmsg_len]);
        let nlh = buf.as_mut_ptr().cast::<nlmsghdr>();
        assert_eq!(
            unsafe { libc::NLMSG_OK(nlh, len) },
            unsafe { nlmsg_ok_ffi(nlh, len) } != 0,
            "nlmsg_len={nlmsg_len} len={len}"
        );
    }
}
