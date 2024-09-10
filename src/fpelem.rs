use std::ops::{Add, Mul, Sub};
extern crate primitive_types;
use primitive_types::U512;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FpElem {
    pub number: U512,
    pub prime: U512,
}

impl FpElem {
    pub fn new<T: Into<U512>, R: Into<U512>>(number: T, prime: R) -> Self {
        let (prime, mut number): (U512, U512) = (prime.into(), number.into());
        number = number % prime;
        FpElem { number, prime }
    }
}


