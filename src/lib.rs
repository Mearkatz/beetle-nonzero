//! Combines the Rust standard library's `NonZero` types into a single struct

use std::{
    fmt::Display,
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    ops::{Not, Shr},
};

use num::{Integer, PrimInt, Zero};

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

impl<T> NonZero<T> {
    /// Returns a new `NonZero` without checking that the provided value is nonzero.
    /// # Safety
    /// `value` must be known to be nonzero
    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self { value }
    }

    /// A reference to the nonzero value
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// A mutable reference to the nonzero value
    /// # Safety
    /// The caller must guarantee that the value is nonzero when the mutable reference is dropped
    pub const unsafe fn get_mut(&mut self) -> &mut T {
        &mut self.value
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
}

impl<T> NonZero<T>
where
    T: Integer,
{
    /// Whether the nonzero integer is even
    pub fn is_even(self) -> bool {
        self.value.is_even()
    }

    /// Whether the nonzero integer is odd
    pub fn is_odd(self) -> bool {
        self.value.is_odd()
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
}

impl<T> NonZero<T>
where
    T: PrimInt + Shr<u32, Output = T>,
{
    /// Returns the number of trailing zeros in the binary representation of the nonzero integer
    pub fn trailing_zeros(self) -> u32 {
        self.value.trailing_zeros()
    }

    /// Returns the number of trailing zeros in the binary representation of the nonzero integer
    #[must_use]
    pub fn without_trailing_zeros(self) -> Self {
        unsafe { Self::new_unchecked(self.value >> self.trailing_zeros()) }
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

#[allow(unused_imports, clippy::unwrap_used)]
mod tests {
    use std::ops::Range;

    use num::{BigInt, FromPrimitive, One};

    use crate::NonZero;

    #[test]
    fn new_works() {
        assert!(NonZero::new(1u8).is_some());
        assert!(NonZero::new(1u16).is_some());
        assert!(NonZero::new(1u32).is_some());
        assert!(NonZero::new(1u64).is_some());
        assert!(NonZero::new(1u128).is_some());
        assert!(NonZero::new(1usize).is_some());
    }
}
