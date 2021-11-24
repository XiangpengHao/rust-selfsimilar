//! A fast generator of discrete random number generator
//! The self similar distribution (80-20 rule) was described in 
//!     Jim Gray, Prakash Sundaresan, Susanne Englert, Ken Baclawski, and Peter J. Weinberger.1994.
//!     Quickly generating billion-record synthetic databases. 
//!     SIGMOD Rec. 23, 2 (June 1994), 243–252.
//!     DOI:https://doi.org/10.1145/191843.191886
//! 
//! Integers between 1...N.
//!     The first h•N integers get 1-h of the distribution.
//! 
//! For example: 
//!     if N = 25 and h= .10, then
//!     80% of the weight goes to the first 5 integers.
//!     and 64% of the weight goes to the first integer.

#[derive(Clone)]
pub struct SelfSimilarDistribution {
    low: usize,
    high: usize,
    skew: f64,
}

impl SelfSimilarDistribution {
    pub fn new(low: usize, high: usize, skew: f64) -> Self {
        Self { low, high, skew }
    }

    fn next<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let next: f64 = rng.gen_range(0.0..1.0);
        let range = (self.high - self.low) as f64;
        let low = self.low as f64;

        let rv = low + range * next.powf(self.skew.log2() / (1.0 - self.skew).log2());

        rv as usize
    }
}

impl rand::distributions::Distribution<usize> for SelfSimilarDistribution {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> usize {
        self.next(rng)
    }
}

use std::fmt;
impl fmt::Debug for SelfSimilarDistribution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("SelfSimilarDistribution")
            .field("low", &self.low)
            .field("high", &self.high)
            .field("skew", &self.skew)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Distribution;

    #[test]
    fn self_similar() {
        let max = 1000;
        let dist = SelfSimilarDistribution::new(0, max, 0.3);
        let mut rng = rand::thread_rng();

        // a very naive test to ensure numbers smaller than 200 should be more than half generated.
        let division = 200;
        let mut division_cnt = 0;
        for _i in 0..max {
            let v = dist.sample(&mut rng);
            if v < division {
                division_cnt += 1;
            }
        }
        assert!(division_cnt > max / 2);
    }
}
