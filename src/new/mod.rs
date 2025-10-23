//! This module contains the future directory structure. If possible, new definitions should
//! get added here.
//!
//! Eventually everything should be moved over, and we will move this directory to the top
//! level in `src`.
//!
//! # Basic structure
//!
//! Each child module here represents a library or group of libraries that we are binding. Each of
//! these has several submodules, representing either a directory or a header file in that library.
//!
//! `#include`s turn into `pub use ...*;` statements. Then at the root level (here), we choose
//! which top-level headers we want to reexport the definitions for.
//!
//! All modules are only crate-public since we don't reexport this structure.

// Libraries available on each platform
cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux_uapi;
        pub(crate) use linux_uapi::*;
    } else if #[cfg(target_os = "android")] {
        mod bionic;
        pub use bionic::*;
    } else if #[cfg(target_vendor = "apple")] {
        mod apple;
        pub(crate) use apple::*;
    }
}

// Headers we export
cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub use linux::can::bcm::*;
        pub use linux::can::j1939::*;
        pub use linux::can::raw::*;
        pub use linux::can::*;
        pub use linux::keyctl::*;
    } else if #[cfg(target_vendor = "apple")] {
        pub use signal::*;
    }
}
