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

impl<T> Copy for NonZero<T> where T: Copy + Clone + Uint {}

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
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Shl, Shr, Sub, SubAssign};

    use num::{traits::Pow, BigUint};

    use crate::{traits::*, NonZero};

    // NonZero<u*> Primitive impl's

    /// Generates an impl block for the trailing(or leading) ones(or zeros) methods
    macro_rules! impl_ones_and_zeros {
        ($name: ty) => {
            impl TrailingZeros for $name {
                fn trailing_zeros(&self) -> u64 {
                    self.value.trailing_zeros().into()
                }
            }

            impl TrailingOnes for $name {
                fn trailing_ones(&self) -> u64 {
                    self.value.trailing_ones().into()
                }
            }

            impl LeadingZeros for $name {
                fn leading_zeros(&self) -> u64 {
                    self.value.leading_zeros().into()
                }
            }

            impl LeadingOnes for $name {
                fn leading_ones(&self) -> u64 {
                    self.value.leading_ones().into()
                }
            }
        };
    }

    impl_ones_and_zeros!(NonZero<u8>);
    impl_ones_and_zeros!(NonZero<u16>);
    impl_ones_and_zeros!(NonZero<u32>);
    impl_ones_and_zeros!(NonZero<u64>);
    impl_ones_and_zeros!(NonZero<u128>);
    impl_ones_and_zeros!(NonZero<usize>);

    // For reasons I'm unaware, BigUint does not have leading_ones or leading_zeros methods, so we must leave them out
    impl TrailingZeros for NonZero<BigUint> {
        fn trailing_zeros(&self) -> u64 {
            self.value.trailing_zeros().unwrap_or(0)
        }
    }
    impl TrailingOnes for NonZero<BigUint> {
        fn trailing_ones(&self) -> u64 {
            self.value.trailing_ones()
        }
    }

    /// Impl's Shl and Shr for NonZero<$name>
    macro_rules! impl_primitive_shl_shr {
        ($name: ty) => {
            impl Shl for NonZero<$name> {
                type Output = Self;
                fn shl(self, rhs: Self) -> Self::Output {
                    Self {
                        value: self.value << rhs.value,
                    }
                }
            }
            impl Shr for NonZero<$name> {
                type Output = Self;
                fn shr(self, rhs: Self) -> Self::Output {
                    Self {
                        value: self.value >> rhs.value,
                    }
                }
            }
        };
    }

    impl_primitive_shl_shr!(u8);
    impl_primitive_shl_shr!(u16);
    impl_primitive_shl_shr!(u32);
    impl_primitive_shl_shr!(u64);
    impl_primitive_shl_shr!(u128);
    impl_primitive_shl_shr!(usize);

    impl<T> Shl<T> for NonZero<BigUint>
    where
        BigUint: Shl<T, Output = BigUint>,
    {
        type Output = Self;

        fn shl(self, rhs: T) -> Self::Output {
            Self {
                value: self.value.clone() << rhs,
            }
        }
    }

    impl<T> Shr<T> for NonZero<BigUint>
    where
        BigUint: Shr<T, Output = BigUint>,
    {
        type Output = Self;

        fn shr(self, rhs: T) -> Self::Output {
            Self {
                value: self.value.clone() >> rhs,
            }
        }
    }

    macro_rules! impl_primitive_without_trailing_zeros {
        ($name: ty) => {
            impl WithoutTrailingZeros for NonZero<$name> {
                fn without_trailing_zeros(&self) -> Self {
                    Self {
                        value: self.value >> self.value.trailing_zeros(),
                    }
                }
            }
        };
    }

    impl_primitive_without_trailing_zeros!(u8);
    impl_primitive_without_trailing_zeros!(u16);
    impl_primitive_without_trailing_zeros!(u32);
    impl_primitive_without_trailing_zeros!(u64);
    impl_primitive_without_trailing_zeros!(u128);
    impl_primitive_without_trailing_zeros!(usize);

    impl WithoutTrailingZeros for NonZero<BigUint> {
        fn without_trailing_zeros(&self) -> Self {
            let value = self.value.clone() >> self.trailing_zeros();
            Self { value }
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

    impl<T: Uint> Pow<u32> for NonZero<T> {
        type Output = Self;
        fn pow(self, rhs: u32) -> Self::Output {
            Self {
                value: self.value.clone().pow(rhs),
            }
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use crate::{
        ranges::RangeNonZero,
        traits::{WithoutTrailingZeros, *},
        NonZero,
    };
    use num::{BigUint, FromPrimitive, One};

    #[test]
    fn ops_work() {
        use crate::NonZero;
        let one: NonZero<u8> = NonZero::new(1u8).unwrap();
        let two: NonZero<u8> = NonZero::new(2u8).unwrap();
        let three: NonZero<u8> = NonZero::new(3u8).unwrap();

        // + - * /
        assert_eq!(one + two, three);
        assert_eq!(three - two, one);
        assert_eq!(two * one, two);
        assert_eq!(three / two, one);
    }

    #[test]
    fn without_trailing_zeros_works() {
        let x = NonZero::new(166_u8).unwrap();
        assert_eq!(x.without_trailing_zeros().value, 83);

        let x = NonZero::new(166_u16).unwrap();
        assert_eq!(x.without_trailing_zeros().value, 83);

        let x = NonZero::new(166_u32).unwrap();
        assert_eq!(x.without_trailing_zeros().value, 83);

        let x = NonZero::new(166_u64).unwrap();
        assert_eq!(x.without_trailing_zeros().value, 83);

        let x = NonZero::new(166_u128).unwrap();
        assert_eq!(x.without_trailing_zeros().value, 83);

        let x = NonZero::new(166_usize).unwrap();
        assert_eq!(x.without_trailing_zeros().value, 83);

        let x = NonZero::new(BigUint::from(166u8)).unwrap();
        assert_eq!(x.without_trailing_zeros().value, 83u8.into());

        assert!(NonZero::new(128u8)
            .unwrap()
            .without_trailing_zeros()
            .is_one());

        assert!(NonZero::new(128u16)
            .unwrap()
            .without_trailing_zeros()
            .is_one());
        assert!(NonZero::new(128u32)
            .unwrap()
            .without_trailing_zeros()
            .is_one());
        assert!(NonZero::new(128u64)
            .unwrap()
            .without_trailing_zeros()
            .is_one());
        assert!(NonZero::new(128u128)
            .unwrap()
            .without_trailing_zeros()
            .is_one());
        assert!(NonZero::new(128usize)
            .unwrap()
            .without_trailing_zeros()
            .is_one());

        assert!(NonZero::new(BigUint::from(128u8))
            .unwrap()
            .without_trailing_zeros()
            .is_one());
    }

    #[test]
    fn ranges_work() {
        let _: RangeNonZero<u8> = RangeNonZero::from_primitives(1u8, 10u8).unwrap();
        let _: RangeNonZero<u16> = RangeNonZero::from_primitives(1u16, 10u16).unwrap();
        let _: RangeNonZero<u32> = RangeNonZero::from_primitives(1u32, 10u32).unwrap();
        let _: RangeNonZero<u64> = RangeNonZero::from_primitives(1u64, 10u64).unwrap();
        let _: RangeNonZero<u128> = RangeNonZero::from_primitives(1u128, 10u128).unwrap();
    }
}
