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

    use crate::{traits::Uint, NonZero};

    // NonZero<u*> Primitive impl's

    /// Generates an impl block for the trailing(or leading) ones(or zeros) methods
    macro_rules! impl_ones_and_zeros {
        ($name: ty) => {
            impl $name {
                /// The number of trailing zeros in the binary representation of the integer
                pub fn trailing_zeros(&self) -> u64 {
                    self.value.clone().trailing_zeros().into()
                }

                /// The number of leading zeros in the binary representation of the integer
                pub fn leading_zeros(&self) -> u64 {
                    self.value.clone().leading_zeros().into()
                }

                /// The number of trailing ones in the binary representation of the integer
                pub fn trailing_ones(&self) -> u64 {
                    self.value.clone().trailing_ones().into()
                }

                /// The number of leading ones in the binary representation of the integer
                pub fn leading_ones(&self) -> u64 {
                    self.value.clone().leading_ones().into()
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
    impl NonZero<BigUint> {
        #[doc = " The number of trailing zeros in the binary representation of the integer"]
        pub fn trailing_zeros(&self) -> u64 {
            self.value.clone().trailing_zeros().unwrap_or(0)
        }
        #[doc = " The number of trailing ones in the binary representation of the integer"]
        pub fn trailing_ones(&self) -> u64 {
            self.value.clone().trailing_ones()
        }
    }

    /// Impl's Shl and Shr for NonZero<$name>
    macro_rules! impl_primitive_shl_shr {
        ($name: ty) => {
            impl Shl for NonZero<$name> {
                type Output = Self;
                fn shl(self, rhs: Self) -> Self::Output {
                    Self {
                        value: self.value.clone() << rhs.value.clone(),
                    }
                }
            }
            impl Shr for NonZero<$name> {
                type Output = Self;
                fn shr(self, rhs: Self) -> Self::Output {
                    Self {
                        value: self.value.clone() >> rhs.value.clone(),
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
