use crate::fpelem::{FpElem, Pow, GenericUInt};
use std::ops::{Mul, Add};
use std::fmt::Debug;
use std::fmt;


#[derive(PartialEq, Clone, Copy)]
pub struct ECPoint<T>{
    // None = Infinity here
    pub position: Option<(T,T)>,
    a: T,
    b:T
}

// Formatting is very ugly else
impl<T: Debug + Copy> Debug for ECPoint<FpElem<T>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // case bash infinity or not
        if let Some((x, y)) = self.position {
            return f
                .debug_struct("ECPoint")
                .field("pos", &(x.number, y.number))
                .field("a", &self.a.number)
                .field("b", &self.b.number)
                .field("p", &self.a.prime)
                .finish();
        } else {
            return f
                .debug_struct("ECPoint")
                .field("pos", &"infinity")
                .field("a", &self.a.number)
                .field("b", &self.b.number)
                .field("p", &self.a.prime)
                .finish();
        }
    }
}

impl<T: GenericUInt> Add for ECPoint<FpElem<T>> {
    type Output = ECPoint<FpElem<T>>;
    fn add(self, toadd: Self) -> ECPoint<FpElem<T>> {
        assert!(
            self.a == toadd.a && self.b == toadd.b,
            "Curves must be the same"
        );
        if let (Some((x1, y1)), Some((x2, y2))) = (self.position, toadd.position) {
            if (x1, y1) == (x2, y2) {
                let s = &(&(&x1.pow(T::from(2)) * T::from(3)) + &self.a) / &(&y1 + &y1);
                let x3 = &s.pow(T::from(2)) - &(&x1 * T::from(2));
                let y3 = &(&s * &(&x1 - &x3)) - &y1;
                return ECPoint {
                    position: Some((x3, y3)),
                    a: self.a,
                    b: self.b,
                };
            } else if x1 == x2 {
                return ECPoint {
                    position: None,
                    a: self.a,
                    b: self.b,
                };
            } else {
                let s = &(&y2 - &y1) / &(&x2 - &x1);
                let x3 = &(&s.pow(T::from(2)) - &x1) - &x2;
                let y3 = &(&s * &(&x1 - &x3)) - &y1;
                return ECPoint {
                    position: Some((x3, y3)),
                    a: self.a,
                    b: self.b,
                };
            }
        } else if let Some((_x, _y)) = self.position {
            return ECPoint {
                position: self.position.clone(),
                a: self.a,
                b: self.b,
            };
        } else if let Some((_x, _y)) = toadd.position {
            return ECPoint {
                position: toadd.position.clone(),
                a: self.a,
                b: self.b,
            };
        }
        return ECPoint {
            position: None,
            a: self.a,
            b: self.b,
        };
    }
}

impl<T: GenericUInt> Mul<T> for ECPoint<FpElem<T>> {
    type Output = ECPoint<FpElem<T>>;
    fn mul(self, tomul: T) -> ECPoint<FpElem<T>> {
        let mut exp = tomul;
        let mut result = ECPoint {
            position: None,
            a: self.a,
            b: self.b,
        };
        let mut current = self;
        while exp != T::from(0) {
            if (exp & T::from(1)) != T::from(0) {
                result = result + current;
            }
            current = current + current;
            exp = exp >> T::from(1);
        }
        result
    }
}

impl<T: GenericUInt> Mul<FpElem<T>> for &ECPoint<FpElem<T>> {
    type Output = ECPoint<FpElem<T>>;
    fn mul(self, tomul: FpElem<T>) -> ECPoint<FpElem<T>> {
        let tomul = tomul.number;
        *self * tomul
    }
}

impl<T: GenericUInt> ECPoint<FpElem<T>> {
    pub fn new(x: impl Into<T>, y: impl Into<T>, a: impl Into<T>, b: impl Into<T>, prime: impl Into<T>) -> Self {
        let prime = prime.into();
        ECPoint {
            position: Some((
                FpElem::new_from(x, prime),
                FpElem::new_from(y, prime)
            )),
            a: FpElem::new_from(a, prime),
            b: FpElem::new_from(b, prime)
        }
    }

    pub fn new_infinity(a: impl Into<T>, b: impl Into<T>, prime: impl Into<T>) -> Self {
        let prime = prime.into();
        ECPoint {
            position: None,
            a: FpElem::new_from(a, prime),
            b: FpElem::new_from(b, prime)
        }
    }
}
