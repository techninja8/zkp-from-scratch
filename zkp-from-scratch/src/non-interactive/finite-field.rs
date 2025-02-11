use num_bigint:: BigInt;
use num_traits::{One, Zero};
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteField<P: u128> {
    value: u128;
}

impl<P: u128> FiniteField<P> {
    pub fn new(value: u128) -> Self {
        Self { value: value % p }
     }
}
