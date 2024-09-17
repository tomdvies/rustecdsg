use crate::ellipticcurve::ECPoint;
use crate::fpelem::{FpElem, GenericUInt, Pow};
use rand::thread_rng;
use rand::Rng;

fn count_bits_generic_uint<T: GenericUInt>(x: T) -> u64 {
    let one = T::from(1);
    let mut count = 0;
    let mut upperbound = T::from(0);
    loop {
        upperbound = upperbound << one;
        upperbound = upperbound + one;
        count = count + 1;
        if upperbound >= x {
            return count;
        }
    }
}

// generate n random bits where n is minimal bits to represent "under" (in the canonical binary rep)
// if number generated is bigger, sack this run and try again - rejection sampling - however 0.5 prob of success each time, range is NON INCLUSIVE
fn get_generic_uint_below<T: GenericUInt>(under: T) -> T {
    let bits = count_bits_generic_uint(under - T::from(1));
    let mut nextout = T::from(0);
    let mut random_bit: u8;
    // loop will exit with probability 1
    loop {
        // draw n uniform random bits
        for _ in 0..bits {
            random_bit = thread_rng().gen_range(0..=1);
            nextout = (nextout << T::from(1)) + T::from(random_bit);
        }
        // if the solution is bigger than our desired bound, sample again
        if nextout >= under {
            nextout = T::from(0);
            continue;
        }
        // else pass it back out
        return nextout;
    }
}

pub struct KeyChain<T> {
    pubkey: ECPoint<FpElem<T>>,
    generator: (ECPoint<FpElem<T>>, T),
    privkey: Option<FpElem<T>>,
}

impl<T: GenericUInt> KeyChain<T> {
    pub fn new(
        privkey: Option<FpElem<T>>,
        pubkey: ECPoint<FpElem<T>>,
        generator: (ECPoint<FpElem<T>>, T),
    ) -> Self {
        KeyChain {
            privkey,
            pubkey,
            generator,
        }
    }

    pub fn new_pub(pubkey: ECPoint<FpElem<T>>, generator: (ECPoint<FpElem<T>>, T)) -> Self {
        KeyChain {
            privkey: None,
            pubkey,
            generator,
        }
    }

    pub fn new_priv(
        privkey: FpElem<T>,
        generator: (ECPoint<FpElem<T>>, T),
    ) -> Self {
        let pubkey =  &generator.0 * privkey; 
        KeyChain {
            privkey: Some(privkey),
            pubkey,
            generator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits_generic_uint() {
        assert_eq!(count_bits_generic_uint(0u32), 1);
        assert_eq!(count_bits_generic_uint(1u32), 1);
        assert_eq!(count_bits_generic_uint(2u32), 2);
        assert_eq!(count_bits_generic_uint(3u32), 2);
        assert_eq!(count_bits_generic_uint(255u32), 8);
    }

    #[test]
    fn test_get_generic_uint_below() {
        let under = 331u32;
        for _ in 0..100000 {
            let result = get_generic_uint_below(under);
            assert!(result < under);
            // Check the distribution is approximately uniform
        }
        let mut counts = vec![0; under as usize];
        for _ in 0..10000 {
            let result = get_generic_uint_below(under);
            assert!(result < under);
            counts[result as usize] += 1;
        }
        let expected_count = 10000 / under;
        let tolerance = expected_count / 10; // 10% tolerance
        assert!((counts[4] as i32 - expected_count as i32).abs() <= tolerance as i32);
    }
}
