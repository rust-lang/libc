//! GNU libc.
//!
//! * Headers: <https://sourceware.org/git/?p=glibc.git> (official)
//! * Headers: <https://github.com/sailfishos-mirror/glibc> (mirror)
//!
//! This module structure is modeled after glibc's source tree. Its build system selects headers
//! from different locations based on the platform, which we mimic here with reexports.

/// Source directory: `bits/`
///
/// <https://github.com/sailfishos-mirror/glibc/tree/master/bits>
mod bits {
    #[cfg(target_os = "linux")]
    pub(crate) mod signum_generic;
}

/// Source directory: `posix/`
///
/// <https://github.com/sailfishos-mirror/glibc/tree/master/posix>
mod posix {
    pub(crate) mod unistd;
}

#[cfg(target_os = "linux")]
pub(crate) mod signal;

/// Source directory: `io/sys/`
///
/// <https://github.com/sailfishos-mirror/glibc/tree/master/io/sys>
pub(crate) mod sys {
    #[cfg(target_os = "linux")]
    pub(crate) mod statvfs;
}

/// Source directory: `sysdeps/`
///
/// <https://github.com/sailfishos-mirror/glibc/tree/master/sysdeps>
mod sysdeps {
    // FIXME(pthread): eventually all platforms should use this module
    #[cfg(target_os = "linux")]
    pub(crate) mod nptl;
    pub(crate) mod unix;

    // You'll notice some directories that provide a `bits` module. These are included via
    // `path = "..."` wherever the generic implementation lives.
}

pub(crate) use posix::*;
// FIXME(pthread): eventually all platforms should use this module
#[cfg(target_os = "linux")]
#[allow(unused)]
pub(crate) use sysdeps::nptl::pthread;
#[cfg(target_os = "linux")]
pub(crate) use sysdeps::unix::linux::net;
