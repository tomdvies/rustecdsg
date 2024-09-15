use rustecdsg::FpElem;
use std::fmt::Debug;
use rustecdsg::Pow;
use rustecdsg::GenericUInt;
extern crate primitive_types;
use primitive_types::{U256, U512};

#[test]
fn fpelem_new() {
    // format: ((innum, inprime), (expectedmodp, expectedprime))
    let pairs = [((8, 7), (1, 7)), ((10, 97), (10, 97)), ((100, 97), (3, 97))];
    for (inpair, outpair) in pairs.iter() {
        let result = FpElem::new(inpair.0, inpair.1);
        assert_eq!((result.number, result.prime), *outpair);
        let result = FpElem::<U512>::new_from(inpair.0, inpair.1);
        assert_eq!(
            (result.number, result.prime),
            (U512::from(outpair.0), U512::from(outpair.1))
        );
    }
    for (inpair, outpair) in pairs.iter() {
        let result = FpElem::new(U512::from(inpair.0), U512::from(inpair.1));
        assert_eq!(
            (result.number, result.prime),
            (U512::from(outpair.0), U512::from(outpair.1))
        );
    }
}

#[test]
fn fpelem_add() {
    // format: ((lhs, rhs, mod), result)
    let pairs = [((9192, 127712, 65537), 5830), ((1, 9911, 65537), 9912)];
    for (in3, out) in pairs.iter() {
        let lhs = FpElem::new(in3.0, in3.2);
        let rhs = FpElem::new(in3.1, in3.2);
        let expectedout = FpElem::new(*out, in3.2);
        assert_eq!(&lhs + &rhs, expectedout);
    }
}

#[test]
fn fpelem_sub() {
    // format: ((lhs, rhs, mod), result)
    let pairs = [((9192, 127712, 65537), 12554), ((1, 9911, 65537), 55627)];
    for (in3, out) in pairs.iter() {
        let lhs = FpElem::new(in3.0, in3.2);
        let rhs = FpElem::new(in3.1, in3.2);
        let expectedout = FpElem::new(*out, in3.2);
        assert_eq!(&lhs - &rhs, expectedout);
    }
}

#[test]
fn fpelem_mul() {
    // format: ((lhs, rhs, mod), result)
    let pairs = [((9192, 127712, 65537), 29960), ((1, 9911, 65537), 9911)];
    for (in3, out) in pairs.iter() {
        let lhs = FpElem::new(in3.0, in3.2);
        let rhs = FpElem::new(in3.1, in3.2);
        let expectedout = FpElem::new(*out, in3.2);
        assert_eq!(&lhs * &rhs, expectedout);
    }
}

#[test]
fn fpelem_div() {
    // format: ((lhs, rhs, mod), result)
    let pairs = [((92192, 127712, 65537), 7653), ((1, 9911, 65537), 47941)];
    for (in3, out) in pairs.iter() {
        let lhs = FpElem::new(in3.0, in3.2);
        let rhs = FpElem::new(in3.1, in3.2);
        let expectedout = FpElem::new(*out, in3.2);
        assert_eq!(&lhs / &rhs, expectedout);
    }
    // for runtime
    let p = "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";
    let to_invert = [10, 99991, 2, 1208128, 1299392];
    for r in to_invert.iter() {
        let to_inv = FpElem::<U256>::new_from(*r, p);
        let one = FpElem::<U256>::new_from(1, p);
        assert_eq!(&to_inv / &to_inv, one);
    }
}

// slow loop through and mul
fn slow_pow<T: GenericUInt + Debug>(exp: FpElem<T>, base: T) -> FpElem<T>{
    let one = T::from(1);
    let mut out = FpElem::new(one, exp.prime);
    let zero = T::from(0);
    let mut counter = base;
    loop{
        if counter == zero {
            return out;
        }
        out = &out * &exp;
        counter = counter - one;
    }
}

#[test]
fn fpelem_pow(){
    // compares bashing mul with pow_mod
    let triplets = [(100,3,1033), (100,30,3673), (300,400,200)];
    for (base, exp, prime) in triplets.iter(){
        let base = FpElem::new(*base, *prime);
        let rhs = slow_pow(base, *exp);
        assert_eq!(base.pow(*exp), rhs);
    }
}


