//! Verify that `libc::uname()` populates the `utsname` struct with each field
//! starting at the correct C offset on Cygwin.
//!
//! Cygwin's `<sys/utsname.h>` declares all six fields as 65 bytes
//! (`_UTSNAME_LENGTH`). If any field is mis-sized in the Rust declaration,
//! every subsequent field shifts in the struct and `CStr::from_ptr` reads
//! that field starting at the wrong byte. The most observable consequence
//! is that `nodename` no longer matches `gethostname()`, so this test is
//! the smallest behavioural check that catches such a regression.

#![cfg(target_os = "cygwin")]

use std::ffi::CStr;
use std::mem::MaybeUninit;

#[test]
fn uname_nodename_matches_gethostname() {
    let mut uts = MaybeUninit::<libc::utsname>::uninit();
    let mut host = [0u8; 256];

    unsafe {
        assert_ne!(
            libc::uname(uts.as_mut_ptr()),
            -1,
            "uname() failed",
        );
        assert_ne!(
            libc::gethostname(host.as_mut_ptr() as *mut libc::c_char, host.len()),
            -1,
            "gethostname() failed",
        );

        let uts = uts.assume_init();
        let nodename = CStr::from_ptr(uts.nodename.as_ptr())
            .to_str()
            .expect("uname.nodename is not valid UTF-8");
        let hostname = CStr::from_ptr(host.as_ptr() as *const libc::c_char)
            .to_str()
            .expect("gethostname() returned non-UTF-8");

        assert_eq!(
            nodename, hostname,
            "uname.nodename and gethostname() must agree",
        );
    }
}

#[test]
fn uname_fields_are_non_empty() {
    let mut uts = MaybeUninit::<libc::utsname>::uninit();

    unsafe {
        assert_ne!(libc::uname(uts.as_mut_ptr()), -1, "uname() failed");
        let uts = uts.assume_init();

        let read = |buf: &[libc::c_char], name: &str| -> String {
            CStr::from_ptr(buf.as_ptr())
                .to_str()
                .unwrap_or_else(|_| panic!("uname.{name} is not valid UTF-8"))
                .to_owned()
        };

        // Every variable-length field must read as a non-empty string.
        // A leading-zero byte here would mean the field is being read
        // from a misaligned offset (the bug this test guards against).
        assert!(!read(&uts.sysname, "sysname").is_empty());
        assert!(!read(&uts.nodename, "nodename").is_empty());
        assert!(!read(&uts.release, "release").is_empty());
        assert!(!read(&uts.version, "version").is_empty());
        assert!(!read(&uts.machine, "machine").is_empty());
    }
}
