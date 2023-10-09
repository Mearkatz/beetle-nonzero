use num::{traits::Pow, BigUint, Integer, Unsigned};
use std::fmt::Debug;

pub trait Uint: Sized + Debug + Unsigned + Integer + Clone + Pow<u32, Output = Self> {}
impl Uint for u8 {}
impl Uint for u16 {}
impl Uint for u32 {}
impl Uint for u64 {}
impl Uint for u128 {}
impl Uint for usize {}
impl Uint for BigUint {}

pub trait TrailingZeros {
    /// Returns the number of trailing zeros in the binary representation of the nonzero integer
    fn trailing_zeros(&self) -> u64;
}

pub trait LeadingZeros {
    /// Returns the number of leading zeros in the binary representation of the nonzero integer
    fn leading_zeros(&self) -> u64;
}

pub trait TrailingOnes {
    /// Returns the number of trailing ones in the binary representation of the nonzero integer
    fn trailing_ones(&self) -> u64;
}

pub trait LeadingOnes {
    /// Returns the number of leading ones in the binary representation of the nonzero integer
    fn leading_ones(&self) -> u64;
}

pub trait WithoutTrailingZeros: TrailingZeros {
    /// Returns the number with its trailing zeros removed
    fn without_trailing_zeros(&self) -> Self;

    /// Returns the number with its trailing zeros removed.
    /// May potentially be faster ?
    fn without_trailing_zeros_unchecked(&self) -> Self;
}
