//! Range related types

use num::{BigUint, One};

use crate::{traits::ToNonZero, NonZero, NonZeroBigUint, PrimUint};

#[derive(Debug, Clone, Copy)]
pub struct RangeNonZeroUnsigned<T: PrimUint> {
    pub start: NonZero<T>,
    pub stop: NonZero<T>,

    // Keeps track of the current value
    value: NonZero<T>,
}

impl<T: PrimUint> RangeNonZeroUnsigned<T> {
    pub fn new(start: NonZero<T>, stop: NonZero<T>) -> Self {
        Self {
            start,
            stop,
            value: start,
        }
    }

    pub fn from_primitives(start: T, stop: T) -> Option<Self> {
        let start = start.to_nonzero()?;
        let stop = stop.to_nonzero()?;
        Some(Self {
            start,
            stop,
            value: start,
        })
    }
}

impl<T: PrimUint> Iterator for RangeNonZeroUnsigned<T> {
    type Item = NonZero<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value < self.stop {
            let current_value = self.value;
            let one: NonZero<T> = NonZero { value: T::one() };
            self.value += one;
            Some(current_value)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct RangeNonZeroBigUint {
    pub start: NonZeroBigUint,
    pub stop: NonZeroBigUint,

    // Keeps track of the current value
    value: NonZeroBigUint,
}

impl RangeNonZeroBigUint {
    pub fn new(start: NonZeroBigUint, stop: NonZeroBigUint) -> Self {
        Self {
            start: start.clone(),
            stop,
            value: start,
        }
    }

    pub fn from_biguints(start: BigUint, stop: BigUint) -> Option<Self> {
        let start = NonZeroBigUint::new(start)?;
        let stop = NonZeroBigUint::new(stop)?;
        Some(Self::new(start, stop))
    }
}

impl Iterator for RangeNonZeroBigUint {
    type Item = NonZeroBigUint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value < self.stop {
            let current_value = self.value.clone();
            let one = NonZeroBigUint::one();
            self.value += one;
            Some(current_value)
        } else {
            None
        }
    }
}
