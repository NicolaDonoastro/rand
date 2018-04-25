// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// https://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! The Bernoulli distribution.

use Rng;
use distributions::Distribution;

/// The Bernoulli distribution.
///
/// This is a special case of the Binomial distribution where `n = 1`.
///
/// # Example
///
/// ```rust
/// use rand::distributions::{Bernoulli, Distribution};
///
/// let d = Bernoulli::new(0.3);
/// let v = d.sample(&mut rand::thread_rng());
/// println!("{} is from a Bernoulli distribution", v);
/// ```
///
/// # Precision
///
/// This `Bernoulli` distribution uses 64 bits from the RNG (a `u64`),
/// so only probabilities that are multiples of 2<sup>-64</sup> can be
/// represented.
#[derive(Clone, Copy, Debug)]
pub struct Bernoulli {
    /// Probability of success, relative to the maximal integer.
    p_int: u64,
}

impl Bernoulli {
    /// Construct a new `Bernoulli` with the given probability of success `p`.
    ///
    /// # Panics
    ///
    /// If `p < 0` or `p > 1`.
    ///
    /// # Precision
    ///
    /// For `p = 1.0`, the resulting distribution will always generate true.
    /// For `p = 0.0`, the resulting distribution will always generate false.
    /// Due to the limitations of floating point numbers, not all internally
    /// supported probabilities (multiples of 2<sup>-64</sup>) can be specified
    /// using this constructor. If you need more precision, use
    /// `Bernoulli::from_int` instead.
    #[inline]
    pub fn new(p: f64) -> Bernoulli {
        assert!(p >= 0.0, "Bernoulli::new called with p < 0");
        assert!(p <= 1.0, "Bernoulli::new called with p > 1");
        let p_int = if p < 1.0 {
            (p * (::core::u64::MAX as f64)) as u64
        } else {
            // Avoid overflow: u64::MAX as f64 cannot be represented as u64.
            ::core::u64::MAX
        };
        Bernoulli { p_int }
    }

    /// Construct a new `Bernoulli` with the probability of success given as an
    /// integer `p_int = p * 2^64`.
    ///
    /// `p_int = 0` corresponds to `p = 0.0`.
    /// `p_int = u64::MAX` corresponds to `p = 1.0`.
    ///
    /// This is more precise than using `Bernoulli::new`.
    #[inline]
    pub fn from_int(p_int: u64) -> Bernoulli {
        Bernoulli { p_int }
    }
}

impl Distribution<bool> for Bernoulli {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        // Make sure to always return true for p = 1.0.
        if self.p_int == ::core::u64::MAX {
            return true;
        }
        let r: u64 = rng.gen();
        r < self.p_int
    }
}

#[cfg(test)]
mod test {
    use {Rng, SmallRng, FromEntropy};
    use distributions::Distribution;
    use super::Bernoulli;

    #[test]
    fn test_trivial() {
        let mut r = SmallRng::from_entropy();
        let always_false = Bernoulli::new(0.0);
        let always_true = Bernoulli::new(1.0);
        for _ in 0..5 {
            assert_eq!(r.sample::<bool, _>(&always_false), false);
            assert_eq!(r.sample::<bool, _>(&always_true), true);
            assert_eq!(Distribution::<bool>::sample(&always_false, &mut r), false);
            assert_eq!(Distribution::<bool>::sample(&always_true, &mut r), true);
        }
    }

    #[test]
    fn test_average() {
        const P: f64 = 0.3;
        let d = Bernoulli::new(P);
        const N: u32 = 10_000_000;

        let mut sum: u32 = 0;
        let mut rng = SmallRng::from_entropy();
        for _ in 0..N {
            if d.sample(&mut rng) {
                sum += 1;
            }
        }
        let avg = (sum as f64) / (N as f64);

        assert!((avg - P).abs() < 1e-3);
    }
}
