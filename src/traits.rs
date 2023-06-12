use num::{Integer, PrimInt, Unsigned};
use std::fmt::Debug;

use crate::NonZero;

pub trait PrimUint: Sized + Debug + PrimInt + Unsigned + Integer {}
impl PrimUint for u8 {}
impl PrimUint for u16 {}
impl PrimUint for u32 {}
impl PrimUint for u64 {}
impl PrimUint for u128 {}

pub trait ToNonZero
where
    Self: PrimUint,
{
    fn to_nonzero(self) -> Option<NonZero<Self>> {
        NonZero::new(self)
    }
}
