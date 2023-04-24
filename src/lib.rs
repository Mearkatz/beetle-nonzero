use num::Zero;

#[deny(missing_docs)]

/// Represents a value that is known to be unequal to zero.
pub struct NonZero<T: Zero> {
    value: T,
}

impl<T: Zero> NonZero<T> {
    // Tries to create a new instance of NonZero.
    // If `value` equals zero, this returns None,
    // otherwise this returns a new NonZero containing `value`.
    pub fn new(value: T) -> Option<Self> {
        if value.is_zero() {
            None
        } else {
            Some(Self { value })
        }
    }

    /// Returns a reference to the non-zero value stored in `self`
    pub fn get(&self) -> &T {
        &self.value
    }
}

/// Represents a value that might be unequal to zero.
/// Only use this instead of NonZero if you are sure that `value` is NonZero,
/// and you need the extra performance
pub struct NonZeroUnchecked<T: Zero> {
    /// The non-zero value
    pub value: T,
}

impl<T: Zero> NonZeroUnchecked<T> {
    /// Creates a new instance of NonZeroUnchecked, without checking if value is non-zero
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Tries converting a NonZeroUnchecked to a NonZero.
    pub fn check(self) -> Option<NonZero<T>> {
        NonZero::new(self.value)
    }
}
