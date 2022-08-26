//! Compare libc's makdev function against the actual C macros, for various
//! inputs.

extern crate libc;

#[cfg(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "openbsd",
))]
mod t {
    use libc::{self, c_uint, dev_t};

    extern "C" {
        pub fn makedev_ffi(major: c_uint, minor: c_uint) -> dev_t;
    }

    fn compare(major: c_uint, minor: c_uint) {
        let expected = unsafe { makedev_ffi(major, minor) };
        assert_eq!(libc::makedev(major, minor), expected);
    }

    // Every OS should be able to handle 8 bit major and minor numbers
    #[test]
    fn test_8bits() {
        for major in 0..256 {
            for minor in 0..256 {
                compare(major, minor);
            }
        }
    }

    // Android allows 12 bits for major and 20 for minor
    #[test]
    #[cfg(target_os = "android")]
    fn test_android_like() {
        for major in [0, 1, 255, 256, 4095] {
            for minor_exp in [1, 8, 16] {
                for minor in [(1 << minor_exp) - 1, (1 << minor_exp)] {
                    compare(major, minor);
                }
            }
            compare(major, (1 << 20) - 1);
        }
    }

    // These OSes allow 32 bits for minor, but only 8 for major
    #[test]
    #[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd",))]
    fn test_fbsd11_like() {
        for major in [0, 1, 255] {
            for minor_exp in [1, 8, 16, 24, 31] {
                for minor in [(1 << minor_exp) - 1, (1 << minor_exp)] {
                    compare(major, minor);
                }
            }
            compare(major, c_uint::MAX);
        }
    }

    // OpenBSD allows 8 bits for major and 24 for minor
    #[test]
    #[cfg(target_os = "openbsd")]
    fn test_openbsd_like() {
        for major in [0, 1, 255] {
            for minor_exp in [1, 8, 16] {
                for minor in [(1 << minor_exp) - 1, (1 << minor_exp)] {
                    compare(major, minor);
                }
            }
            compare(major, (1 << 24) - 1);
        }
    }

    // These OSes allow 32 bits for both minor and major
    #[cfg(any(
        target_os = "empscripten",
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "linux",
    ))]
    #[test]
    fn test_fbsd12_like() {
        if std::mem::size_of::<dev_t>() >= 8 {
            for major_exp in [0, 16, 24, 31] {
                for major in [(1 << major_exp) - 1, (1 << major_exp)] {
                    for minor_exp in [1, 8, 16, 24, 31] {
                        for minor in [(1 << minor_exp) - 1, (1 << minor_exp)] {
                            compare(major, minor);
                        }
                    }
                    compare(major, c_uint::MAX);
                }
                compare(c_uint::MAX, c_uint::MAX);
            }
        }
    }
}
