//! Range related types

use num::One;

use crate::{traits::Uint, NonZero};

#[derive(Debug, Clone)]
pub struct RangeNonZero<T: Uint> {
    pub start: NonZero<T>,
    pub stop: NonZero<T>,

    // Keeps track of the current value
    value: NonZero<T>,
}

impl<T: Uint> RangeNonZero<T> {
    pub fn new(start: NonZero<T>, stop: NonZero<T>) -> Self {
        Self {
            start: start.clone(),
            stop,
            value: start,
        }
    }

    pub fn from_primitives(start: T, stop: T) -> Option<Self> {
        let start = NonZero::new(start)?;
        let stop = NonZero::new(stop)?;
        Some(Self {
            start: start.clone(),
            stop,
            value: start,
        })
    }
}

impl<T: Uint> Iterator for RangeNonZero<T> {
    type Item = NonZero<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value < self.stop {
            let current_value = self.value.clone();
            self.value += One::one();
            Some(current_value)
        } else {
            None
        }
    }
}
