//! Combines the Rust standard library's `NonZero` types into a single struct

use num_traits::Zero;
use std::{
    fmt::Display,
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    ops::Not,
};

/// An integer that is known to not equal zero.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct NonZero<T> {
    value: T,
}

impl<T> Display for NonZero<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> NonZero<T>
where
    T: Zero,
{
    /// Returns a new `NonZero<T>` if `value` is nonzero
    pub fn new(value: T) -> Option<Self> {
        value.is_zero().not().then_some(Self { value })
    }

    /// Returns a new `NonZero` without checking that the provided value is nonzero.
    /// # Safety
    /// `value` must be known to be nonzero
    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self { value }
    }

    /// Tries replacing the nonzero value with a new one.
    /// If the new value is nonzero this returns the old value,
    /// otherwise this returns None.
    pub fn replace(&mut self, new_value: T) -> Option<T> {
        let mut other = Self::new(new_value)?;
        self.swap(&mut other);
        Some(other.value)
    }

    /// Sets `self.value` using the provided value.
    /// Only succeeds if the value provided was nonzero.
    /// Returns whether the operation succeeded.
    pub fn set(&mut self, value: T) -> bool {
        let nonzero = value.is_zero().not();
        if nonzero {
            unsafe { self.set_unchecked(value) }
        }
        nonzero
    }

    /// Sets the internal value of the nonzero integer.
    /// If the value equals zero, this panics.
    /// # Safety
    /// `value` must be known to be nonzero
    pub unsafe fn set_unchecked(&mut self, value: T) {
        self.value = value;
    }

    /// Applies a function to the inner value and returns a `NonZero` if the result was nonzero.
    pub fn map(self, f: impl Fn(T) -> T) -> Option<Self> {
        Self::new(f(self.value))
    }

    /// Applies a function to the inner value and returns a `NonZero` if the result was nonzero.
    /// # Safety
    /// `f` must return a nonzero integer
    #[must_use]
    pub unsafe fn map_unchecked(self, f: impl Fn(T) -> T) -> Self {
        Self::new_unchecked(f(self.value))
    }

    /// A reference to the nonzero value
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// A mutable reference to the nonzero value
    /// # Safety
    /// The caller must guarantee that the value is nonzero when the mutable reference is dropped
    #[deprecated(since = "0.3.14", note = "use `swap` instead")]
    pub const unsafe fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Swap the nonzero value of two `NonZero`s
    pub fn swap(&mut self, other: &mut Self) {
        std::mem::swap(self, other);
    }
}

macro_rules! impl_from_primitive {
    ($new_name: ty, $primitive: ty) => {
        impl From<$primitive> for $new_name {
            fn from(value: $primitive) -> Self {
                Self { value: value.get() }
            }
        }
    };
}

impl_from_primitive!(NonZero<u8>, NonZeroU8);
impl_from_primitive!(NonZero<u16>, NonZeroU16);
impl_from_primitive!(NonZero<u32>, NonZeroU32);
impl_from_primitive!(NonZero<u64>, NonZeroU64);
impl_from_primitive!(NonZero<u128>, NonZeroU128);
impl_from_primitive!(NonZero<usize>, NonZeroUsize);
