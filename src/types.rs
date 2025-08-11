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
