//! ctest can't check `capget` and `capset`: the C library headers have no prototype.

#![cfg(target_os = "linux")]

use libc::{
    __user_cap_data_struct,
    __user_cap_header_struct,
    _LINUX_CAPABILITY_U32S_3,
    _LINUX_CAPABILITY_VERSION_3,
};

const U32S: usize = _LINUX_CAPABILITY_U32S_3 as usize;
const EMPTY: __user_cap_data_struct = __user_cap_data_struct {
    effective: 0,
    permitted: 0,
    inheritable: 0,
};

fn header() -> __user_cap_header_struct {
    __user_cap_header_struct {
        version: _LINUX_CAPABILITY_VERSION_3 as u32,
        pid: 0,
    }
}

fn sets(data: &[__user_cap_data_struct; U32S]) -> [(u32, u32, u32); U32S] {
    data.map(|d| (d.effective, d.permitted, d.inheritable))
}

#[test]
fn test_capget_matches_syscall() {
    let mut from_fn = [EMPTY; U32S];
    let mut from_syscall = [EMPTY; U32S];
    let mut hdr = header();
    assert_eq!(unsafe { libc::capget(&mut hdr, from_fn.as_mut_ptr()) }, 0);
    let mut hdr = header();
    assert_eq!(
        unsafe { libc::syscall(libc::SYS_capget, &raw mut hdr, from_syscall.as_mut_ptr()) },
        0
    );
    assert_eq!(sets(&from_fn), sets(&from_syscall));
}

// Setting the current sets back needs no privilege.
#[test]
fn test_capset_current_sets() {
    let mut before = [EMPTY; U32S];
    let mut after = [EMPTY; U32S];
    let mut hdr = header();
    assert_eq!(unsafe { libc::capget(&mut hdr, before.as_mut_ptr()) }, 0);
    let mut hdr = header();
    assert_eq!(unsafe { libc::capset(&mut hdr, before.as_mut_ptr()) }, 0);
    let mut hdr = header();
    assert_eq!(unsafe { libc::capget(&mut hdr, after.as_mut_ptr()) }, 0);
    assert_eq!(sets(&before), sets(&after));
}
