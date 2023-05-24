use num::{BigUint, CheckedSub, One, PrimInt};

#[deny(missing_docs)]

/// A primitive unsigned integer like u8 or u32 which is known to be non-zero
pub struct NonZero<T: PrimInt + Copy> {
    value: T,
}

impl<T: PrimInt + Copy> NonZero<T> {
    // Tries to create a new instance of NonZero.
    // If `value` equals zero, this returns None,
    // otherwise this returns a new NonZero containing `value`.
    pub fn new(value: T) -> Option<Self> {
        let value: T = value.checked_sub(&T::one())?;
        Some(Self { value })
    }

    /// Unsafe version of new that doesn't ensure the value is nonzero
    pub fn unchecked_new(value: T) -> Self {
        let value = value - T::one();
        Self { value }
    }

    /// Returns a reference to the non-zero value stored in `self`
    pub fn get(&self) -> T {
        self.value + T::one()
    }
}

/// A `BigUint` that is known to be non-zero.
pub struct NonZeroBigUint {
    value: BigUint,
}

impl NonZeroBigUint {
    // Tries to create a new instance of NonZero.
    // If `value` equals zero, this returns None,
    // otherwise this returns a new NonZero containing `value`.
    pub fn new(value: BigUint) -> Option<Self> {
        let value: BigUint = value.checked_sub(&BigUint::one())?;
        Some(Self { value })
    }

    /// Unsafe version of new that doesn't ensure the value is nonzero
    pub fn unchecked_new(value: BigUint) -> Self {
        let value: BigUint = value - BigUint::one();
        Self { value }
    }

    /// Returns a reference to the non-zero value stored in `self`
    pub fn get(&self) -> BigUint {
        self.value.clone() + BigUint::one()
    }
}
