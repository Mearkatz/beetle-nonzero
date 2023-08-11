pub mod ranges;
pub mod traits;

use num::One;
use std::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
use traits::Uint;

/// A wrapper around a primitive non-zero integer like `i32` or `u32`.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct NonZero<T: Uint> {
    value: T,
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

impl<T: Uint> One for NonZero<T> {
    fn one() -> Self {
        Self { value: T::one() }
    }
}

pub mod ops {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    use num::{traits::Pow, BigUint, PrimInt};

    use crate::{traits::Uint, NonZero};

    // NonZero<u*> Primitive impl's
    impl<T: Uint + PrimInt> NonZero<T> {
        pub fn trailing_zeros(&self) -> u64 {
            self.value.trailing_zeros().into()
        }

        pub fn leading_zeros(&self) -> u64 {
            self.value.leading_zeros().into()
        }

        pub fn trailing_ones(&self) -> u64 {
            self.value.trailing_ones().into()
        }

        pub fn leading_ones(&self) -> u64 {
            self.value.leading_ones().into()
        }

        pub fn shift_right(&self, rhs: usize) -> Self {
            Self {
                value: self.value >> rhs,
            }
        }
        pub fn shift_left(&self, rhs: usize) -> Self {
            Self {
                value: self.value << rhs,
            }
        }
    }

    // NonZero<BigUint> impls
    impl NonZero<BigUint> {
        pub fn trailing_zeros(&self) -> u64 {
            self.value.trailing_zeros()
        }

        pub fn leading_zeros(&self) -> u64 {
            self.value.leading_zeros().into()
        }

        pub fn trailing_ones(&self) -> u64 {
            self.value.trailing_ones().into()
        }

        pub fn leading_ones(&self) -> u64 {
            self.value.leading_ones().into()
        }

        pub fn shift_right(&self, rhs: usize) -> Self {
            Self {
                value: self.value >> rhs,
            }
        }
        pub fn shift_left(&self, rhs: usize) -> Self {
            Self {
                value: self.value << rhs,
            }
        }
    }

    impl<T: Uint> NonZero<T> {
        /// Returns a new NonZero<T> if `value` is non-zero, else None
        pub fn new(value: T) -> Option<Self> {
            if value.is_zero() {
                None
            } else {
                Some(Self { value })
            }
        }

        pub fn is_even(&self) -> bool {
            self.value.is_even()
        }
        pub fn is_odd(&self) -> bool {
            self.value.is_odd()
        }
    }

    impl<T: Uint> Add for NonZero<T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                value: self.value.clone() + rhs.value.clone(),
            }
        }
    }

    impl<T: Uint> Sub for NonZero<T> {
        type Output = Self;
        #[cfg(debug_assertions)]
        fn sub(self, rhs: Self) -> Self::Output {
            if self < rhs {
                panic!("{self:?} - {rhs:?} produces an underflow or value equal to zero");
            }

            Self {
                value: self.value.clone() - rhs.value.clone(),
            }
        }

        #[cfg(not(debug_assertions))]
        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                value: self.value.clone() - rhs.value.clone(),
            }
        }
    }

    impl<T: Uint> Mul for NonZero<T> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self {
                value: (self.value.clone() * rhs.value.clone()),
            }
        }
    }

    impl<T: Uint> Div for NonZero<T> {
        type Output = Self;

        #[cfg(debug_assertions)]
        fn div(self, rhs: Self) -> Self::Output {
            if self < rhs {
                panic!("{self:?} / {rhs:?} produces an underflow or value equal to zero");
            }

            Self {
                value: self.value.clone() / rhs.value.clone(),
            }
        }

        #[cfg(not(debug_assertions))]
        fn div(self, rhs: Self) -> Self::Output {
            Self {
                value: self.value.clone() / rhs.value.clone(),
            }
        }
    }

    impl<T: Uint> Pow<u32> for NonZero<T> {
        type Output = Self;
        fn pow(self, rhs: u32) -> Self::Output {
            Self {
                value: self.value.clone().pow(rhs),
            }
        }
    }

    impl<T: Uint> AddAssign for NonZero<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = self.clone() + rhs
        }
    }

    impl<T: Uint> SubAssign for NonZero<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = self.clone() - rhs
        }
    }

    impl<T: Uint> MulAssign for NonZero<T> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = self.clone() * rhs
        }
    }

    impl<T: Uint> DivAssign for NonZero<T> {
        fn div_assign(&mut self, rhs: Self) {
            *self = self.clone() / rhs
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use num::One;

    use crate::ranges::RangeNonZero;

    #[test]
    fn ops_work() {
        use crate::NonZero;
        let one: NonZero<u8> = NonZero::new(1u8).unwrap();
        let two: NonZero<u8> = NonZero::new(2u8).unwrap();
        let three: NonZero<u8> = NonZero::new(3u8).unwrap();

        // + - * /
        assert_eq!(one.clone() + two.clone(), three.clone());
        assert_eq!(three.clone() - two.clone(), one.clone());
        assert_eq!(two.clone() * one.clone(), two.clone());
        assert_eq!(three / two, one);
    }

    #[test]
    fn ranges_work() {
        let _ = RangeNonZero::from_primitives(1u8, 10u8).unwrap();
        let _ = RangeNonZero::from_primitives(1u16, 10u16).unwrap();
        let _ = RangeNonZero::from_primitives(1u32, 10u32).unwrap();
        let _ = RangeNonZero::from_primitives(1u64, 10u64).unwrap();
        let _ = RangeNonZero::from_primitives(1u128, 10u128).unwrap();
    }
}
