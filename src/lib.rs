use derive_more::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Display,
    DivAssign, MulAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use num::{One, PrimInt, Unsigned};

/// A wrapper around a primitive non-zero integer like `i32` or `u32`.
#[derive(
    Copy,
    Clone,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Add,
    Sub,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    Shl,
    Shr,
    ShlAssign,
    ShrAssign,
    BitAnd,
    BitOr,
    BitXor,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    Display,
    Debug,
)]
pub struct NonZero<T: PrimInt> {
    value: T,
}

impl<T: PrimInt> NonZero<T> {
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

impl<T: PrimInt> std::ops::Mul for NonZero<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: (self.value * rhs.value),
        }
    }
}

impl<T: PrimInt> std::ops::Div for NonZero<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // This would always work, but returning an Option is annoying.
        // Self::new(self.value * rhs.value)

        // Instead, we'll simply use 1 as our minimum value
        let ans: T = (self.value / rhs.value).min(T::one());
        Self { value: ans }
    }
}

pub struct RangeNonZeroUnsigned<T: PrimInt + Unsigned> {
    pub start: NonZero<T>,
    pub stop: NonZero<T>,

    // Keeps track of the current value
    value: NonZero<T>,
}

impl<T: PrimInt + Unsigned> RangeNonZeroUnsigned<T> {
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

impl<T: PrimInt + std::ops::AddAssign + Unsigned> Iterator for RangeNonZeroUnsigned<T> {
    type Item = NonZero<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value < self.stop {
            let one: NonZero<T> = NonZero { value: T::one() };
            self.value += one;
            Some(self.value)
        } else {
            None
        }
    }
}

impl<T: PrimInt> One for NonZero<T> {
    fn one() -> Self {
        Self { value: T::one() }
    }
}

pub trait ToNonZero
where
    Self: PrimInt,
{
    fn to_nonzero(self) -> Option<NonZero<Self>> {
        NonZero::new(self)
    }
}

impl<T: PrimInt> ToNonZero for T {}

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
