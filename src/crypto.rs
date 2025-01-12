use crate::ellipticcurve::ECPoint;
use crate::fpelem::{FpElem, GenericUInt};
use rand::thread_rng;
use rand::Rng;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyChainError {
    // #[error("data store disconnected")]
    //
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    // #[error("unknown data store error")]
    // Unknown,
    #[error("no private key available for signing")]
    NoPrivateKey,
    #[error("unknown error")]
    Unknown,
}

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

#[derive(Debug)]
pub struct Signature<FE> {
    // r is x value of "target" R = kG w/ k rand in Fp
    pub targetx: FE,
    // s it the value (z + re)/k w/ k as above, z as hash to encode, and e is the priv key
    pub sig: FE,
}

impl<T: GenericUInt> Signature<FpElem<T>> {
    pub fn new(r: FpElem<T>, s: FpElem<T>) -> Signature<FpElem<T>> {
        Signature { targetx: r, sig: s }
    }
}

pub struct KeyChain<T> {
    pub pubkey: ECPoint<FpElem<T>>,
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

    pub fn new_priv(privkey: FpElem<T>, generator: (ECPoint<FpElem<T>>, T)) -> Self {
        let pubkey = &generator.0 * privkey;
        KeyChain {
            privkey: Some(privkey),
            pubkey,
            generator,
        }
    }

    pub fn verify_sig(&self, hash: &FpElem<T>, signature: &Signature<FpElem<T>>) -> bool {
        let u = hash / &signature.sig;
        let v = &signature.targetx / &signature.sig;
        if let Some((r, _)) = (&self.generator.0 * u + &self.pubkey * v).position {
            let r = FpElem::new(r.number, self.generator.1);
            return r == signature.targetx;
        } else {
            return false;
        }
    }

    pub fn sign(&self, hash: &FpElem<T>) -> Result<Signature<FpElem<T>>, KeyChainError> {
        if let Some(privkey) = self.privkey {
            let gen = self.generator.0;
            // TODO: this
            let k = FpElem::new(get_generic_uint_below(hash.number), self.generator.1);
            let r = &gen * k;
            // catch for point at infinity - just try again
            if let Some((r, _)) = r.position {
                let r = FpElem::new(r.number, self.generator.1); //r.to_field(privkeyring.generator.1);
                let s = &(hash + &(&r * &privkey)) / &k;
                return Ok(Signature { targetx: r, sig: s });
            } else {
                return self.sign(hash);
            }
        }
        Err(KeyChainError::NoPrivateKey)
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
        let under = 30u32;
        for _ in 0..100000 {
            let result = get_generic_uint_below(under);
            assert!(result < under);
            // Check the distribution is approximately uniform
        }
        let mut counts = vec![0; under as usize];
        for _ in 0..100000 {
            let result = get_generic_uint_below(under);
            assert!(result < under);
            counts[result as usize] += 1;
        }
        let expected_count = 100000 / under;
        let tolerance = expected_count / 10; // 10% tolerance
        assert!((counts[4] as i32 - expected_count as i32).abs() <= tolerance as i32);
        //
        // // Plot the distribution
        // use plotters::prelude::*;
        //
        // let root = BitMapBackend::new("random_distribution.png", (640, 480))
        //     .into_drawing_area();
        // root.fill(&WHITE).unwrap();
        //
        // let max_count = *counts.iter().max().unwrap() as f32;
        // let mut chart = ChartBuilder::on(&root)
        //     .caption("Distribution of Random Values", ("sans-serif", 30))
        //     .margin(5)
        //     .x_label_area_size(30)
        //     .y_label_area_size(30)
        //     .build_cartesian_2d(0..under as i32, 0f32..max_count)
        //     .unwrap();
        //
        // chart.configure_mesh().draw().unwrap();
        //
        // // Plot actual distribution
        // chart
        //     .draw_series(
        //         Histogram::vertical(&chart)
        //             .style(BLUE.mix(0.5).filled())
        //             .data(counts.iter().enumerate().map(|(x, y)| (x as i32, *y as f32)))
        //     )
        //     .unwrap()
        //     .label("Actual Distribution");
        //
        // // Plot expected line
        // chart
        //     .draw_series(LineSeries::new(
        //         (0..under as i32).map(|x| (x, expected_count as f32)),
        //         &RED,
        //     ))
        //     .unwrap()
        //     .label("Expected Value");
        //
        // chart
        //     .configure_series_labels()
        //     .background_style(&WHITE.mix(0.8))
        //     .border_style(&BLACK)
        //     .draw()
        //     .unwrap();
    }
}
