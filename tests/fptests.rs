use rustecdsg::FpElem;
extern crate primitive_types;
use primitive_types::U512;

#[test]
fn fpelem_new() {
    // format: ((innum, inprime), (expectedmodp, expectedprime))
    let pairs = [((8, 7), (1, 7)), ((10, 97), (10, 97)), ((100, 97), (3, 97))];
    for (inpair, outpair) in pairs.iter() {
        let result = FpElem::new(inpair.0, inpair.1);
        assert_eq!(
            (result.number, result.prime),
            (U512::from(outpair.0), U512::from(outpair.1))
        );
    }
}
