//! Entrypoint for Apple headers, usually found as part of the Xcode SDK.

pub(crate) mod xnu;
pub(crate) use xnu::*;

pub(crate) mod apple_libc {
    pub(crate) mod signal;
}

pub(crate) use apple_libc::*;
