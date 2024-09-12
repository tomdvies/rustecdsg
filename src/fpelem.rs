use std::fmt::Debug;
use std::ops::{Add, BitAnd, Div, Mul, Rem, Shr, Sub};
//extern crate primitive_types;
//use primitive_types::U512;

pub trait GenericUInt:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Rem<Output = Self>
    + Copy
    + Div<Output = Self>
    + BitAnd<Output = Self>
    + PartialOrd
    + PartialEq
    + Shr<Output = Self>
    + Default
    + From<u8>
{
}

impl<T> GenericUInt for T where
    T: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Rem<Output = Self>
        + Copy
        + Div<Output = Self>
        + BitAnd<Output = Self>
        + PartialOrd
        + PartialEq
        + Shr<Output = Self>
        + Default
        + From<u8>
{
}

// https://www.jjj.de/fxt/fxtbook.pdf
fn sub_mod<T: GenericUInt>(a: T, b: T, m: T) -> T {
    let (a, b) = (a % m, b % m);
    if a >= b {
        return a - b;
    } else {
        return m - b + a;
    }
}

fn add_mod<T: GenericUInt>(a: T, b: T, m: T) -> T {
    let (a, b) = (a % m, b % m);
    if b == T::from(0) {
        a
    } else {
        let b = m - b;
        sub_mod(a, b, m)
    }
}

// Treat multiplication as composition of addiiton and spam binary expansion
// See UofCambridge Quantum Information and Computation sheet 3 lol
fn mul_mod<T: GenericUInt>(a: T, b: T, m: T) -> T {
    let mut b = b;
    let zero = T::from(0);
    let mut result = zero;
    let mut current = a;
    while b != zero {
        if (b & T::from(1)) != zero {
            // note that the overflow safe property hinges on add_mod being overflow safe
            result = add_mod(result, current, m);
        }
        current = add_mod(current, current, m);
        b = b >> T::from(1);
    }
    result
}

// again spam binary expansion - quite slow as mul_mod does the same
pub fn pow_mod<T: GenericUInt>(a: T, e: T, m: T) -> T {
    let mut b = e;
    let zero = T::from(0);
    let one = T::from(1);
    let mut result = one;
    let mut current = a;
    while b != zero {
        if (b & T::from(1)) != zero {
            // note that the overflow safe property hinges on add_mod being overflow safe
            result = mul_mod(result, current, m);
        }
        current = mul_mod(current, current, m);
        b = b >> T::from(1);
    }
    result
}

// this is so bad it's funny, case bashes a signed integer type
#[derive(Clone, Copy)]
struct GenSignedUint<T> {
    value: T,
    isneg: bool
}


fn mul_inv<T: GenericUInt>(a:T, b:T) -> T
{
    let (mut a, mut b) = (a,b);
    if b <= T::from(1){
        return T::from(0);
    }
    let one = T::from(1);
    let zero = T::from(0);
    let b0 = b;
    let mut x0 = GenSignedUint{ value:zero, isneg:false }; // b = 1*b + 0*a
    let mut x1 = GenSignedUint{ value:one, isneg:false }; // a = 0*b + 1*a

    while a > one
    {
        if b == zero // means original A and B were not co-prime so there is no answer
        {return zero;}
        let q = a / b;
        let t = b; b = a % b; a = t;

        let t2 = x0;
        let qx0 = q * x0.value;
        if x0.isneg != x1.isneg
        {
            x0.value = x1.value + qx0;
            x0.isneg = x1.isneg;
        }
        else
        {
            x0.value = if x1.value > qx0 {
                    x1.value - qx0
                } else {
                    qx0 - x1.value
                };

            x0.isneg = if x1.value > qx0 {
                    x1.isneg
                } else {
                    !x0.isneg
                };
        }
        x1 = t2;
    }
    if x1.isneg {
        return b0 - x1.value;
    }
    else {
        return x1.value;
    }
}


// this is slow, FLT
//fn mod_inv<T: GenericUInt>(a: T, m: T) -> T {
//    pow_mod(a, m - T::from(2), m)
//}

// An element of the unique finite field of order p where p is prime
// No checks are done for primality of p as this is expensive, however if p were to be a composite
// number everything should work other than the Div trait (which will break even if m if invertible
// mod p as mod_inv uses FLT)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FpElem<T> {
    pub number: T,
    pub prime: T,
}

impl<T: GenericUInt> FpElem<T> {
    pub fn new(number: T, prime: T) -> Self {
        let number = number % prime;
        FpElem { number, prime }
    }

    pub fn new_from(number: impl Into<T>, prime: impl Into<T>) -> Self {
        let (number, prime) = (number.into(), prime.into());
        let number = number % prime;
        FpElem { number, prime }
    }
}

//impl FpElem<U512> {
//    pub fn new_u512(number: impl Into<U512>, prime: impl Into<U512>) -> Self {
//        let (number, prime) = (number.into(), prime.into());
//        let number = number % prime;
//        FpElem { number, prime }
//    }
//}

impl<T: GenericUInt> Sub for &FpElem<T> {
    type Output = FpElem<T>;
    fn sub(self, tosub: &FpElem<T>) -> FpElem<T> {
        assert!(self.prime == tosub.prime, "Prime base must be the same");
        FpElem {
            number: sub_mod(self.number, tosub.number, self.prime),
            prime: self.prime,
        }
    }
}

impl<T: GenericUInt> Add for &FpElem<T> {
    type Output = FpElem<T>;
    fn add(self, toadd: &FpElem<T>) -> FpElem<T> {
        assert!(self.prime == toadd.prime, "Prime base must be the same");
        FpElem {
            number: add_mod(self.number, toadd.number, self.prime),
            prime: self.prime,
        }
    }
}

impl<T: GenericUInt> Mul for &FpElem<T> {
    type Output = FpElem<T>;
    fn mul(self, tomul: &FpElem<T>) -> FpElem<T> {
        assert!(self.prime == tomul.prime, "Prime base must be the same");
        FpElem {
            number: mul_mod(self.number, tomul.number, self.prime),
            prime: self.prime,
        }
    }
}

impl<T: GenericUInt> Mul<T> for &FpElem<T> {
    type Output = FpElem<T>;
    fn mul(self, tomul: T) -> FpElem<T> {
        FpElem {
            number: mul_mod(self.number, tomul % self.prime, self.prime),
            prime: self.prime,
        }
    }
}

pub trait Pow<T> {
    fn pow_mod(&self, exponent: T) -> Self;
}

impl<T> Pow<T> for FpElem<T>
where
    T: GenericUInt,
{
    fn pow_mod(&self, exponent: T) -> Self {
        FpElem {
            number: pow_mod(self.number, exponent, self.prime),
            prime: self.prime,
        }
    }
}

impl<T: GenericUInt + Debug> Div for &FpElem<T> {
    type Output = FpElem<T>;
    fn div(self, rhs: Self) -> FpElem<T> {
        FpElem {
            number: mul_mod(self.number, mul_inv(rhs.number, rhs.prime), rhs.prime),
            prime: self.prime,
        }
    }
}
