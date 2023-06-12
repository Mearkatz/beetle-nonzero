use num::{traits::Pow, Integer, One};
use num::{BigUint, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use traits::{PrimUint, ToNonZero};

pub mod ranges;
pub mod traits;

/// A wrapper around a primitive non-zero integer like `i32` or `u32`.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct NonZero<T: PrimUint> {
    value: T,
}

impl<T: PrimUint> NonZero<T> {
    /// Returns a new NonZero<T> if `value` is non-zero, else None
    pub fn new(value: T) -> Option<Self> {
        if value.is_zero() {
            None
        } else {
            Some(Self { value })
        }
    }

    /// Returns a destructured copy of the NonZero value.
    pub fn get(&self) -> T {
        self.value
    }

    pub fn is_even(&self) -> bool {
        self.get().is_even()
    }
    pub fn is_odd(&self) -> bool {
        self.get().is_odd()
    }

    /// Returns the number of trailing zeros in the binary representation of this number
    pub fn trailing_zeros(&self) -> u32 {
        self.get().trailing_zeros()
    }
}

impl<T: PrimUint> Add for NonZero<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.get() + rhs.get(),
        }
    }
}

impl<T: PrimUint> Sub for NonZero<T> {
    type Output = Self;
    #[cfg(debug_assertions)]
    fn sub(self, rhs: Self) -> Self::Output {
        if self < rhs {
            panic!("{self:?} - {rhs:?} produces an underflow or value equal to zero");
        }

        Self {
            value: self.get() - rhs.get(),
        }
    }

    #[cfg(not(debug_assertions))]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.get() - rhs.get(),
        }
    }
}

impl<T: PrimUint> Mul for NonZero<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: (self.value * rhs.value),
        }
    }
}

impl<T: PrimUint> Div for NonZero<T> {
    type Output = Self;

    #[cfg(debug_assertions)]
    fn div(self, rhs: Self) -> Self::Output {
        if self < rhs {
            panic!("{self:?} / {rhs:?} produces an underflow or value equal to zero");
        }

        Self {
            value: self.get() / rhs.get(),
        }
    }

    #[cfg(not(debug_assertions))]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.get() / rhs.get(),
        }
    }
}

impl<T: PrimUint> Pow<u32> for NonZero<T> {
    type Output = Self;
    fn pow(self, rhs: u32) -> Self::Output {
        Self {
            value: self.get().pow(rhs),
        }
    }
}

impl<T: PrimUint> AddAssign for NonZero<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: PrimUint> SubAssign for NonZero<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<T: PrimUint> MulAssign for NonZero<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl<T: PrimUint> DivAssign for NonZero<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl<T: PrimUint> One for NonZero<T> {
    fn one() -> Self {
        Self { value: T::one() }
    }
}

impl<T: PrimUint> ToNonZero for T {}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct NonZeroBigUint {
    value: BigUint,
}

// NonZero<u*> -> NonZeroBigUint
impl From<NonZero<u8>> for NonZeroBigUint {
    fn from(value: NonZero<u8>) -> Self {
        let value: BigUint = BigUint::from(value.value);
        Self { value }
    }
}
impl From<NonZero<u16>> for NonZeroBigUint {
    fn from(value: NonZero<u16>) -> Self {
        let value: BigUint = BigUint::from(value.value);
        Self { value }
    }
}
impl From<NonZero<u32>> for NonZeroBigUint {
    fn from(value: NonZero<u32>) -> Self {
        let value: BigUint = BigUint::from(value.value);
        Self { value }
    }
}
impl From<NonZero<u64>> for NonZeroBigUint {
    fn from(value: NonZero<u64>) -> Self {
        let value: BigUint = BigUint::from(value.value);
        Self { value }
    }
}
impl From<NonZero<u128>> for NonZeroBigUint {
    fn from(value: NonZero<u128>) -> Self {
        let value: BigUint = BigUint::from(value.value);
        Self { value }
    }
}

impl NonZeroBigUint {
    pub fn new(value: BigUint) -> Option<Self> {
        if value.is_zero() {
            None
        } else {
            Some(Self { value })
        }
    }

    /// Returns a destructured copy of the NonZero value.
    pub fn get(&self) -> &BigUint {
        &self.value
    }

    pub fn is_even(&self) -> bool {
        self.get().is_even()
    }

    pub fn is_odd(&self) -> bool {
        self.get().is_odd()
    }

    /// Returns the number of trailing_zeros
    pub fn trailing_zeros(&self) -> u64 {
        /*
        Justification for unwrap_unchecked:
        `trailing_zeros` only returns None for values equal to zero,
        which should be impossible for this type, so this is fine.
        */
        unsafe { self.get().trailing_zeros().unwrap_unchecked() }
    }
}

impl Add for NonZeroBigUint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.get() + rhs.get(),
        }
    }
}

impl Sub for NonZeroBigUint {
    type Output = Self;
    #[cfg(debug_assertions)]
    fn sub(self, rhs: Self) -> Self::Output {
        if self < rhs {
            panic!("{self:?} - {rhs:?} produces an underflow or value equal to zero");
        }

        Self {
            value: self.get() - rhs.get(),
        }
    }

    #[cfg(not(debug_assertions))]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.get() - rhs.get(),
        }
    }
}

impl Mul for NonZeroBigUint {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: (self.value * rhs.value),
        }
    }
}

impl Div for NonZeroBigUint {
    type Output = Self;

    #[cfg(debug_assertions)]
    fn div(self, rhs: Self) -> Self::Output {
        if self < rhs {
            panic!("{self:?} / {rhs:?} produces an underflow or value equal to zero");
        }

        Self {
            value: self.get() / rhs.get(),
        }
    }

    #[cfg(not(debug_assertions))]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.get() / rhs.get(),
        }
    }
}

impl Pow<u32> for NonZeroBigUint {
    type Output = Self;
    fn pow(self, rhs: u32) -> Self::Output {
        Self {
            value: self.get().pow(rhs),
        }
    }
}

impl AddAssign for NonZeroBigUint {
    fn add_assign(&mut self, rhs: Self) {
        self.value.add_assign(rhs.get())
    }
}

impl SubAssign for NonZeroBigUint {
    fn sub_assign(&mut self, rhs: Self) {
        self.value.sub_assign(rhs.get())
    }
}

impl MulAssign for NonZeroBigUint {
    fn mul_assign(&mut self, rhs: Self) {
        self.value.mul_assign(rhs.get())
    }
}

impl DivAssign for NonZeroBigUint {
    fn div_assign(&mut self, rhs: Self) {
        self.value.div_assign(rhs.get())
    }
}

impl One for NonZeroBigUint {
    fn one() -> Self {
        Self {
            value: BigUint::one(),
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use num::One;

    use crate::NonZeroBigUint;

    #[test]
    fn ops_work() {
        use crate::ToNonZero;
        let one: crate::NonZero<u8> = 1u8.to_nonzero().unwrap();
        let two: crate::NonZero<u8> = 2u8.to_nonzero().unwrap();
        let three: crate::NonZero<u8> = 3u8.to_nonzero().unwrap();

        // + - * /
        assert_eq!(one + two, three);
        assert_eq!(three - two, one);
        assert_eq!(two * one, two);
        assert_eq!(three / two, one);
    }

    #[test]
    fn ranges_work() {
        use crate::ranges::RangeNonZeroUnsigned;
        let _ = RangeNonZeroUnsigned::from_primitives(1u8, 10u8).unwrap();
        let _ = RangeNonZeroUnsigned::from_primitives(1u16, 10u16).unwrap();
        let _ = RangeNonZeroUnsigned::from_primitives(1u32, 10u32).unwrap();
        let _ = RangeNonZeroUnsigned::from_primitives(1u64, 10u64).unwrap();
        let _ = RangeNonZeroUnsigned::from_primitives(1u128, 10u128).unwrap();
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn assignment_ops_for_nonzero_biguints_work() {
        let one = NonZeroBigUint::one();
        let mut big = one.clone();
        big += one.clone();
        assert_eq!(big, one.clone() + one.clone());
    }
}
