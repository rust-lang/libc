use core::mem::MaybeUninit;

/// A transparent wrapper over `MaybeUninit<T>` to represent uninitialized padding.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Padding<T: Copy>(MaybeUninit<T>);

impl<T: Copy> Default for Padding<T> {
    fn default() -> Self {
        Self(MaybeUninit::zeroed())
    }
}
