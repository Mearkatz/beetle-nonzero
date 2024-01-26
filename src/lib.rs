//! Combines the Rust standard library's `NonZero` types into a single struct

use std::{
    fmt::Display,
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
};

use num::Integer;

/// An integer that is known to not equal zero.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct NonZero<T: Integer> {
    value: T,
}

impl<T: Integer + Clone + Display> Display for NonZero<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl<T> Copy for NonZero<T> where T: Integer + Copy {}

impl<T: Integer + Clone> NonZero<T> {
    /// A copy of the nonzero value.
    /// # Performance    
    /// May be expensive if `T` doesn't implement `Copy`
    pub fn get(&self) -> T {
        self.value.clone()
    }
}

impl<T: Integer> NonZero<T> {
    /// Returns a new `NonZero<T>` if `value` is nonzero
    pub fn new(value: T) -> Option<Self> {
        let nonzero = !value.is_zero();
        nonzero.then(|| unsafe { Self::new_unchecked(value) })
    }

    /// Returns a new `NonZero` without checking that the provided value is nonzero.
    /// # Safety
    /// `value` must be known to be nonzero
    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self { value }
    }

    /// A reference to the nonzero value
    pub const fn get_ref(&self) -> &T {
        &self.value
    }

    /// Sets the internal value of the nonzero integer.
    /// If the value equals zero, this panics.
    /// # Panics
    /// Will panic if the value provided equals zero
    pub fn set_value(&mut self, value: T) {
        assert!(!value.is_zero(), "Cannot set a NonZero's value to zero");
        unsafe { self.set_value_unchecked(value) }
    }

    /// Sets the internal value of the nonzero integer.
    /// If the value equals zero, this panics.
    /// # Safety
    /// `value` must be known to be nonzero
    pub unsafe fn set_value_unchecked(&mut self, value: T) {
        self.value = value;
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
