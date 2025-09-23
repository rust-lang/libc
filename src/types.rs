//! Platform-agnostic support types.

use core::mem::MaybeUninit;

use crate::prelude::*;

/// A transparent wrapper over `MaybeUninit<T>` to represent uninitialized padding
/// while providing `Default`.
#[allow(unused)]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct Padding<T: Copy>(MaybeUninit<T>);

impl<T: Copy> Default for Padding<T> {
    fn default() -> Self {
        Self(MaybeUninit::zeroed())
    }
}

/// The default repr type used for C style enums in Rust.
#[cfg(target_env = "msvc")]
#[allow(unused)]
pub(crate) type CEnumRepr = c_int;
#[cfg(not(target_env = "msvc"))]
#[allow(unused)]
pub(crate) type CEnumRepr = c_uint;
