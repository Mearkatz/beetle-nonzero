use num::{traits::Pow, Integer, One, PrimInt, Unsigned};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A wrapper around a primitive non-zero integer like `i32` or `u32`.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct NonZero<T: PrimUint> {
    value: T,
}

pub trait PrimUint: Sized + Debug + PrimInt + Unsigned + Integer {}
impl PrimUint for u8 {}
impl PrimUint for u16 {}
impl PrimUint for u32 {}
impl PrimUint for u64 {}
impl PrimUint for u128 {}

impl<T: PrimUint> NonZero<T> {
    pub fn is_even(&self) -> bool {
        self.get().is_even()
    }
    pub fn is_odd(&self) -> bool {
        self.get().is_odd()
    }
}

impl<T: PrimUint> NonZero<T> {
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
    fn sub(self, rhs: Self) -> Self::Output {
        if self <= rhs {
            panic!("{self:?} - {rhs:?} produces an underflow or value equal to zero");
        }

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

    fn div(self, rhs: Self) -> Self::Output {
        if self < rhs {
            panic!("{self:?} / {rhs:?} produces an underflow or value equal to zero");
        }

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

pub trait ToNonZero
where
    Self: PrimUint,
{
    fn to_nonzero(self) -> Option<NonZero<Self>> {
        NonZero::new(self)
    }
}

impl<T: PrimUint> ToNonZero for T {}

mod tests {

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
        use crate::RangeNonZeroUnsigned;
        let _ = RangeNonZeroUnsigned::from_primitives(1u8, 10u8).unwrap();
        let _ = RangeNonZeroUnsigned::from_primitives(1u16, 10u16).unwrap();
        let _ = RangeNonZeroUnsigned::from_primitives(1u32, 10u32).unwrap();
        let _ = RangeNonZeroUnsigned::from_primitives(1u64, 10u64).unwrap();
        let _ = RangeNonZeroUnsigned::from_primitives(1u128, 10u128).unwrap();
    }
}
