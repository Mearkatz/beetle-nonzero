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
